use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnarchiveTeamParams {
    /// Team key (e.g. "ENG") or UUID of the team to unarchive
    pub id: String,
}
