use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteTeamMembershipParams {
    /// The team membership ID
    pub id: String,
}
