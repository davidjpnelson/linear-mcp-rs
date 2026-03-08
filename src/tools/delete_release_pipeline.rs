use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteReleasePipelineParams {
    /// UUID of the release pipeline to delete
    pub id: String,
}
