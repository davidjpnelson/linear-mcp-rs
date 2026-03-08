use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BatchCreateIssuesParams {
    /// JSON array of issue objects
    pub issues: String,
    /// Team key for all issues
    pub team: String,
}
