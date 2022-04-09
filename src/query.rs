pub struct IssueQuery;
pub mod issue_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IssueQuery";
    pub const QUERY : & str = "query IssueQuery($owner: String!, $repository_name: String!, $number: Int!) {\n  repository(owner: $owner, name:$repository_name) {\n    name\n    issue(number: $number){\n          title\n          trackedIssues(first: 10){\n            nodes {\n              number\n              title\n              state\n          }\n      }\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive()]
    pub enum IssueState {
        CLOSED,
        OPEN,
        Other(String),
    }
    impl ::serde::Serialize for IssueState {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                IssueState::CLOSED => "CLOSED",
                IssueState::OPEN => "OPEN",
                IssueState::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IssueState {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "CLOSED" => Ok(IssueState::CLOSED),
                "OPEN" => Ok(IssueState::OPEN),
                _ => Ok(IssueState::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub owner: String,
        pub repository_name: String,
        pub number: Int,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub repository: Option<IssueQueryRepository>,
    }
    #[derive(Deserialize)]
    pub struct IssueQueryRepository {
        pub name: String,
        pub issue: Option<IssueQueryRepositoryIssue>,
    }
    #[derive(Deserialize)]
    pub struct IssueQueryRepositoryIssue {
        pub title: String,
        #[serde(rename = "trackedIssues")]
        pub tracked_issues: IssueQueryRepositoryIssueTrackedIssues,
    }
    #[derive(Deserialize)]
    pub struct IssueQueryRepositoryIssueTrackedIssues {
        pub nodes: Option<Vec<Option<IssueQueryRepositoryIssueTrackedIssuesNodes>>>,
    }
    #[derive(Deserialize)]
    pub struct IssueQueryRepositoryIssueTrackedIssuesNodes {
        pub number: Int,
        pub title: String,
        pub state: IssueState,
    }
}
impl graphql_client::GraphQLQuery for IssueQuery {
    type Variables = issue_query::Variables;
    type ResponseData = issue_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: issue_query::QUERY,
            operation_name: issue_query::OPERATION_NAME,
        }
    }
}
