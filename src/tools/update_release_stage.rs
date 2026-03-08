use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateReleaseStageParams {
    /// UUID of the release stage to update
    pub id: String,
    /// New name for the release stage
    pub name: Option<String>,
    /// New hex color code (e.g. "#ff0000")
    pub color: Option<String>,
    /// New position in the pipeline
    pub position: Option<f64>,
    /// Whether the stage is frozen
    pub frozen: Option<bool>,
}
