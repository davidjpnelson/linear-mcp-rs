use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteInitiativeRelationParams {
    /// The initiative relation ID
    pub id: String,
}
