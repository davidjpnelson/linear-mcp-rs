use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTeamParams {
    /// Team key (e.g. "ENG") or UUID of the team to retrieve
    pub id: String,
}
