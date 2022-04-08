use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.docs.graphql",
    query_path = "graphql/query.graphql"
)]
pub struct IssueQuery;

const GITHUB_URL: &str = "https://api.github.com/graphql";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let token = env!("GITHUB_ACCESS_TOKEN");

    let client = reqwest::Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            ))
            .collect(),
        )
        .build()?;

    let v = issue_query::Variables {
        owner: "k-nasa".into(),
        repository_name: "wai".into(),
    };
    let request_body = IssueQuery::build_query(v);

    let res = client.post(GITHUB_URL).json(&request_body).send().await?;
    let response_body: Response<issue_query::ResponseData> = res.json().await?;
    for i in response_body
        .data
        .unwrap()
        .repository
        .unwrap()
        .issues
        .nodes
        .unwrap()
        .iter()
    {
        let i = i.as_ref().unwrap();
        println!("{:?}", i.title);
    }

    Ok(())
}
