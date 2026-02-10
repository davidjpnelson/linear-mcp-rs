use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListStatesParams {
    /// Team key to filter states for a specific team
    pub team: Option<String>,
}
