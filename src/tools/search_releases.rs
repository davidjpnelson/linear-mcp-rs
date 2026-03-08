use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchReleasesParams {
    /// Search query string
    pub query: String,
    /// Maximum number of results to return
    pub limit: Option<i32>,
}
