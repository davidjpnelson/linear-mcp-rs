use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateRoadmapParams {
    /// Roadmap UUID
    pub id: String,
    /// New roadmap name
    pub name: Option<String>,
    /// New description
    pub description: Option<String>,
}
