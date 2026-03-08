use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCommentParams {
    /// UUID of the comment to retrieve
    pub id: String,
}
