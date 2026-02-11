use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCycleParams {
    /// Cycle UUID
    pub id: String,
}
