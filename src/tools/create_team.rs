use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateTeamParams {
    /// Team name
    pub name: String,
    /// Team key (e.g. 'ENG'). Auto-generated if not provided.
    pub key: Option<String>,
    /// Team description
    pub description: Option<String>,
    /// Team timezone (e.g. 'America/New_York')
    pub timezone: Option<String>,
}
