use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetIssueRelationParams {
    /// The issue relation ID
    pub id: String,
}
