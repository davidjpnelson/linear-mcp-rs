use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateIssueRelationParams {
    /// Issue relation UUID
    pub id: String,
    /// New relation type: blocks, duplicate, related
    #[serde(rename = "type")]
    pub relation_type: String,
}
