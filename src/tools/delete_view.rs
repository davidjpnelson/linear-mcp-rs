use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteViewParams {
    /// Custom view UUID
    pub id: String,
}
