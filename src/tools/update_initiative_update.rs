use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateInitiativeUpdateParams {
    /// The initiative update ID
    pub id: String,
    /// Update body content
    pub body: Option<String>,
    /// Health status
    pub health: Option<String>,
}
