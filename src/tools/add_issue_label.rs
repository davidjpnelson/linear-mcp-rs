use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddIssueLabelParams {
    /// Issue identifier (e.g. 'ENG-123') or UUID
    pub issue: String,
    /// Label name to add
    pub label: String,
}
