use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListTeamMembershipsParams {
    /// Maximum number of results to return
    pub limit: Option<i32>,
    /// Team key
    pub team: Option<String>,
}
