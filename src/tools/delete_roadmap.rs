use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteRoadmapParams {
    /// Roadmap UUID
    pub id: String,
}
