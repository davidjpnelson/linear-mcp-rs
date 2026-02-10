use crate::error::Error;
use crate::graphql::response::{GraphQLError, GraphQLResponse};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

const LINEAR_API_URL: &str = "https://api.linear.app/graphql";

#[derive(Clone)]
pub struct LinearClient {
    http: Client,
    api_key: String,
}

#[derive(Serialize)]
struct GraphQLRequest<V: Serialize> {
    query: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<V>,
}

impl LinearClient {
    pub fn new(api_key: String) -> Self {
        Self {
            http: Client::new(),
            api_key,
        }
    }

    /// Execute a GraphQL query/mutation and deserialize the response data.
    pub async fn execute<V, T>(&self, query: &'static str, variables: Option<V>) -> Result<T, Error>
    where
        V: Serialize,
        T: DeserializeOwned,
    {
        let body = GraphQLRequest { query, variables };

        let resp = self
            .http
            .post(LINEAR_API_URL)
            .header("Authorization", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::GraphQL(format!("HTTP {status}: {text}")));
        }

        let text = resp.text().await?;
        let gql_resp: GraphQLResponse<T> = serde_json::from_str(&text)
            .map_err(|e| Error::GraphQL(format!("Deserialization error: {e}")))?;

        if let Some(errors) = gql_resp.errors {
            let msg = format_gql_errors(&errors);
            return Err(Error::GraphQL(msg));
        }

        gql_resp
            .data
            .ok_or_else(|| Error::GraphQL("No data in response".into()))
    }

    /// Execute a query with JSON value variables (for dynamic filter construction).
    pub async fn execute_json<T>(&self, query: &'static str, variables: serde_json::Value) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let body = serde_json::json!({
            "query": query,
            "variables": variables,
        });

        let resp = self
            .http
            .post(LINEAR_API_URL)
            .header("Authorization", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::GraphQL(format!("HTTP {status}: {text}")));
        }

        let text = resp.text().await?;
        let gql_resp: GraphQLResponse<T> = serde_json::from_str(&text)
            .map_err(|e| Error::GraphQL(format!("Deserialization error: {e}")))?;

        if let Some(errors) = gql_resp.errors {
            let msg = format_gql_errors(&errors);
            return Err(Error::GraphQL(msg));
        }

        gql_resp
            .data
            .ok_or_else(|| Error::GraphQL("No data in response".into()))
    }
}

fn format_gql_errors(errors: &[GraphQLError]) -> String {
    errors
        .iter()
        .map(|e| e.message.as_str())
        .collect::<Vec<_>>()
        .join("; ")
}
