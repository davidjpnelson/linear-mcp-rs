use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTeamMembershipParams {
    /// The team membership ID
    pub id: String,
}
