use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListLabelsParams {
    /// Team key to filter labels for a specific team
    pub team: Option<String>,
    /// Max results (default 50)
    pub limit: Option<u32>,
}
