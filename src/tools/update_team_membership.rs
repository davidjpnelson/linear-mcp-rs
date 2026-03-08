use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateTeamMembershipParams {
    /// The team membership ID
    pub id: String,
    /// Whether the user is a team owner
    pub owner: Option<bool>,
}
