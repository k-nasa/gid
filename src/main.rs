mod cli;

use anyhow::Error;
use async_recursion::async_recursion;
use cli::{CliArgs, Parser};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;

use std::collections::HashMap;

const GITHUB_URL: &str = "https://api.github.com/graphql";

// impl for graphql query
pub type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.docs.graphql",
    query_path = "graphql/query.graphql"
)]
pub struct IssueQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.docs.graphql",
    query_path = "graphql/query.graphql"
)]
pub struct TrackIssuesQuery;

type IssueGraph = HashMap<Issue, Vec<Issue>>;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Issue {
    number: i64,
    title: String,
    state: String,
    url: String,
}

impl Issue {
    pub fn id(&self) -> String {
        // NOTE own/repo/numberのような一意に定まる文字列を返したい。
        self.url.clone()
    }

    pub fn owner(&self) -> String {
        // NOTE https://github.com/k-nasa/gid/issues/28
        let splited: Vec<_> = self.url.split("/").collect();
        splited[3].to_string()
    }

    pub fn repo(&self) -> String {
        // NOTE https://github.com/k-nasa/gid/issues/28
        let splited: Vec<_> = self.url.split("/").collect();
        splited[4].to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = CliArgs::parse();

    let token = match std::env::var("GITHUB_ACCESS_TOKEN") {
        Ok(token) => token,
        Err(_) => anyhow::bail!(
            r"Please set enviroment GITHUB_ACCESS_TOKEN
export GITHUB_ACCESS_TOKEN=xxxx"
        ),
    };
    let client = github_client(&token)?;

    run(client, args).await?;

    Ok(())
}

async fn run(client: Client, args: CliArgs) -> Result<(), anyhow::Error> {
    let issue_graph = fetch_tracked_issue(
        &client,
        args.issue_number as i64,
        &args.organization,
        &args.repository,
    )
    .await?;

    let mermaid = build_mermaid(issue_graph);

    println!("{}", mermaid);

    Ok(())
}

fn build_mermaid(issue_map: IssueGraph) -> String {
    let mut body = vec![];
    let mut links = vec![];
    for (parent_issue, issues) in issue_map {
        for issue in issues {
            let t = format!(
                "{parent_id}[\"{parent_title} #{parent_number}\"]:::{parent_state} --> {child_id}[\"{title} #{number}\"]:::{state}",
                parent_id = parent_issue.id(),
                parent_title = parent_issue.title,
                parent_state = parent_issue.state,
                parent_number = parent_issue.number,
                child_id = issue.id(),
                title = issue.title,
                state = issue.state,
                number = issue.number,
            );
            body.push(t);

            links.push(format!(
                "click {} href \"{}\" _blank",
                issue.id(),
                issue.url
            ));
        }
    }

    body.sort();
    links.sort();

    format!(
        r#"
```mermaid
graph LR

classDef CLOSED fill:#8256d0,color:#FFFFFF,stroke-width:0px;
classDef OPEN fill:#347d39,color:#FFFFFF,stroke-width:0px;

{body}

{links}
```"#,
        body = body.join("\n"),
        links = links.join("\n")
    )
}

#[async_recursion]
async fn fetch_tracked_issue(
    client: &reqwest::Client,
    root_issue: i64,
    owner: &str,
    repository: &str,
) -> Result<IssueGraph, anyhow::Error> {
    let mut init_state = HashMap::new();
    _fetch_tracked_issue(client, root_issue, owner, repository, &mut init_state).await?;

    return Ok(init_state);
}

#[async_recursion]
async fn _fetch_tracked_issue(
    client: &reqwest::Client,
    root_issue_number: i64,
    owner: &str,
    repository: &str,
    issue_graph: &mut IssueGraph,
) -> Result<(), anyhow::Error> {
    let v = track_issues_query::Variables {
        owner: owner.into(),
        repository_name: repository.into(),
        number: root_issue_number,
    };
    let request_body = TrackIssuesQuery::build_query(v);

    let res = client.post(GITHUB_URL).json(&request_body).send().await?;
    let response_body: Response<track_issues_query::ResponseData> = res.json().await?;

    let parent_issue = response_body
        .data
        .expect("Response is None")
        .repository
        .expect("Not found repository")
        .issue
        .expect("Not found issue");

    // FIXME unwrap地獄を修正したい
    for i in parent_issue
        .tracked_issues
        .nodes
        .expect("Not found tracked issues")
    {
        let i = i.as_ref().unwrap();

        let issue = Issue {
            number: i.number,
            url: i.url.clone().into(),
            title: i.title.clone().into(),
            state: state_to_string(&i.state),
        };

        let parent_issue = Issue {
            number: parent_issue.number,
            url: parent_issue.url.clone().into(),
            title: parent_issue.title.clone().into(),
            state: state_to_string(&parent_issue.state),
        };

        if let Some(issues) = issue_graph.get_mut(&parent_issue) {
            issues.push(issue.clone());
        } else {
            issue_graph.insert(parent_issue, vec![issue.clone()]);
        }

        _fetch_tracked_issue(
            client,
            issue.number,
            &issue.owner(),
            &issue.repo(),
            issue_graph,
        )
        .await?;
    }

    Ok(())
}

fn github_client(github_token: &str) -> Result<Client, Error> {
    let client = reqwest::Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_token))?,
            ))
            .collect(),
        )
        .build()?;
    Ok(client)
}

fn state_to_string(state: &track_issues_query::IssueState) -> String {
    match state {
        track_issues_query::IssueState::OPEN => "OPEN",
        track_issues_query::IssueState::CLOSED => "CLOSED",
        _ => "OTHER",
    }
    .to_string()
}
