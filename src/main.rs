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

type IssueGraph = HashMap<i64, Vec<Issue>>;
struct Issue {
    number: i64,
    title: String,
    state: String,
    url: String,
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
    for (number, issues) in issue_map {
        for issue in issues {
            let t = format!(
                "{parent} --> {child}[\"{title} #{number}\"]:::{state}",
                parent = number,
                child = issue.number,
                title = issue.title,
                state = issue.state,
                number = issue.number,
            );
            body.push(t);

            links.push(format!(
                "click {} href \"{}\" _blank",
                issue.number, issue.url
            ));
        }
    }

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
    root_issue: i64,
    owner: &str,
    repository: &str,
    issue_graph: &mut IssueGraph,
) -> Result<(), anyhow::Error> {
    let v = track_issues_query::Variables {
        owner: owner.into(),
        repository_name: repository.into(),
        number: root_issue,
    };
    let request_body = TrackIssuesQuery::build_query(v);

    let res = client.post(GITHUB_URL).json(&request_body).send().await?;
    let response_body: Response<track_issues_query::ResponseData> = res.json().await?;

    // FIXME unwrap地獄を修正したい
    for i in response_body
        .data
        .expect("Response is None")
        .repository
        .expect("Not found repository")
        .issue
        .expect("Not found issue")
        .tracked_issues
        .nodes
        .expect("Not found tracked issues")
    {
        let i = i.as_ref().unwrap();

        let state = match i.state {
            track_issues_query::IssueState::OPEN => "OPEN",
            track_issues_query::IssueState::CLOSED => "CLOSED",
            _ => "OTHER",
        };

        let issue = Issue {
            number: i.number,
            url: i.url.clone().into(),
            title: i.title.clone().into(),
            state: state.into(),
        };

        if let Some(issues) = issue_graph.get_mut(&root_issue) {
            issues.push(issue);
        } else {
            issue_graph.insert(root_issue, vec![issue]);
        }

        _fetch_tracked_issue(client, i.number, owner, repository, issue_graph).await?;
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
