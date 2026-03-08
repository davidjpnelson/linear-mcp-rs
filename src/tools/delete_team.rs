use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteTeamParams {
    /// Team key (e.g. "ENG") or UUID of the team to delete
    pub id: String,
}
