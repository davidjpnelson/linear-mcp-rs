use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchDocumentsParams {
    /// Search term to find in documents
    pub term: String,
    /// Max results to return (default 20)
    pub limit: Option<i32>,
    /// Include comments in search (default false)
    #[serde(rename = "includeComments")]
    pub include_comments: Option<bool>,
}
