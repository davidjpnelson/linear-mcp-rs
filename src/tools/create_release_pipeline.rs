use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateReleasePipelineParams {
    /// Name of the release pipeline
    pub name: String,
    /// Type of the release pipeline
    #[serde(rename = "type")]
    pub pipeline_type: Option<String>,
    /// Comma-separated path patterns to include
    pub include_path_patterns: Option<String>,
}
