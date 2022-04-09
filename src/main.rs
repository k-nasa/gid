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
    fetch_tracked_issue(
        &client,
        args.issue_number as i64,
        &args.organization,
        &args.repository,
    )
    .await?;

    let mermaid = build_mermaid(todo!());

    println!("{}", mermaid);

    Ok(())
}

fn build_mermaid(issue_map: IssueGraph) -> String {
    let head = r#"
```mermaid
graph LR
    classDef CLOSED fill:#8256d0,color:#FFFFFF,stroke-width:0px;
    classDef OPEN fill:#347d39,color:#FFFFFF,stroke-width:0px;
```"#;

    let mut body = String::new();
    for (number, issues) in issue_map {
        for issue in issues {
            let t = format!(
                "\t{parent} --> {child}[\"{title}\"]:::{state}\n",
                parent = number,
                child = issue.number,
                title = issue.title,
                state = issue.state
            );
            body += &t;
        }
    }

    format!("{}\n{}", head, body)
}

#[async_recursion]
async fn fetch_tracked_issue(
    client: &reqwest::Client,
    root_issue: i64,
    owner: &str,
    repository: &str,
) -> Result<(), anyhow::Error> {
    let v = issue_query::Variables {
        owner: owner.into(),
        repository_name: repository.into(),
        number: root_issue,
    };
    let request_body = IssueQuery::build_query(v);

    let res = client.post(GITHUB_URL).json(&request_body).send().await?;
    let response_body: Response<issue_query::ResponseData> = res.json().await?;
    for i in response_body
        .data
        .unwrap()
        .repository
        .unwrap()
        .issue
        .unwrap()
        .tracked_issues
        .nodes
        .unwrap()
    {
        let i = i.as_ref().unwrap();

        let state = match i.state {
            issue_query::IssueState::OPEN => "OPEN",
            issue_query::IssueState::CLOSED => "CLOSED",
            _ => "OTHER",
        };

        println!(
            "\t{} --> {}[\"{}\"]:::{}",
            root_issue, i.number, i.title, state
        );

        println!("click {} href \"{}\" _blank", i.number, i.url);
        fetch_tracked_issue(client, i.number, owner, repository).await?;
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
