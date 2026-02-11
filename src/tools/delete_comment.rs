use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteCommentParams {
    /// Comment UUID
    pub id: String,
}
