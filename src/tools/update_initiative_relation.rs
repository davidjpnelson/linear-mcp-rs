use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateInitiativeRelationParams {
    /// The initiative relation ID
    pub id: String,
    /// Sort order for the relation
    pub sort_order: Option<f64>,
}
