use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetIssueLabelParams {
    /// The issue label ID
    pub id: String,
}
