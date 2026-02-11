use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteInitiativeParams {
    /// Initiative ID (UUID)
    pub id: String,
}
