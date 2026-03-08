use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateReleasePipelineParams {
    /// UUID of the release pipeline to update
    pub id: String,
    /// New name for the release pipeline
    pub name: Option<String>,
    /// New type for the release pipeline
    #[serde(rename = "type")]
    pub pipeline_type: Option<String>,
    /// New comma-separated path patterns to include
    pub include_path_patterns: Option<String>,
}
