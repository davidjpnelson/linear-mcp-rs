use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateRoadmapParams {
    /// Roadmap name — required
    pub name: String,
    /// Description
    pub description: Option<String>,
}
