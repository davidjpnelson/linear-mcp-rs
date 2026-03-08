use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetReleasePipelineParams {
    /// UUID of the release pipeline
    pub id: String,
}
