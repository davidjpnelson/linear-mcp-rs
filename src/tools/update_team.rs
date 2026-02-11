use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateTeamParams {
    /// Team key (e.g. 'ENG') or UUID
    pub id: String,
    /// New team name
    pub name: Option<String>,
    /// New description
    pub description: Option<String>,
    /// New timezone
    pub timezone: Option<String>,
}
