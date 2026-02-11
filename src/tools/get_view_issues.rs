use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetViewIssuesParams {
    /// Custom view ID (UUID)
    pub id: String,
    /// Max issues to return (default 50)
    pub limit: Option<i32>,
}
