use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateReleaseStageParams {
    /// Name of the release stage
    pub name: String,
    /// Hex color code for the release stage (e.g. "#ff0000")
    pub color: String,
    /// Type of the release stage
    #[serde(rename = "type")]
    pub stage_type: String,
    /// Position of the release stage in the pipeline
    pub position: f64,
    /// UUID of the release pipeline this stage belongs to
    pub pipeline_id: String,
    /// Whether the stage is frozen (no new issues can be added)
    pub frozen: Option<bool>,
}
