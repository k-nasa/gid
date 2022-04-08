use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(schema_path = "schema.json", query_path = "query.graphql")]
pub struct MyQuery;

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

    let v = my_query::Variables;
    let request_body = MyQuery::build_query(v);

    let res = client.post(GITHUB_URL).json(&request_body).send().await?;
    let response_body: Response<my_query::ResponseData> = res.json().await?;
    let data = response_body.data.unwrap();
    println!("{:?}", data.organization.unwrap().repository.unwrap().name);
    Ok(())
}
