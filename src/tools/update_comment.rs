use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCommentParams {
    /// Comment UUID
    pub id: String,
    /// New comment body (markdown)
    pub body: String,
}
