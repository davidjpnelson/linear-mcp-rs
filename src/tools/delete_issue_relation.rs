use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteIssueRelationParams {
    /// Issue relation UUID
    pub id: String,
}
