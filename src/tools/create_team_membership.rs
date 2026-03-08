use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateTeamMembershipParams {
    /// Email to resolve
    pub user: String,
    /// Team key
    pub team: String,
    /// Whether the user is a team owner
    pub owner: Option<bool>,
}
