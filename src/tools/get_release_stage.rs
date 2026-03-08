use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetReleaseStageParams {
    /// UUID of the release stage
    pub id: String,
}
