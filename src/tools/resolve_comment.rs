use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResolveCommentParams {
    /// Comment UUID
    pub id: String,
}
