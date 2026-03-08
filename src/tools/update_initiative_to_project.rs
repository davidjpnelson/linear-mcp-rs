use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateInitiativeToProjectParams {
    /// UUID of the initiative-to-project link to update
    pub id: String,
    /// Sort order for the initiative-to-project link
    pub sort_order: Option<f64>,
}
