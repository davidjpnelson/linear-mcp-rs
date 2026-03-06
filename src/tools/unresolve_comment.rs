use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnresolveCommentParams {
    /// Comment UUID
    pub id: String,
}
