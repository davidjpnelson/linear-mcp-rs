use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArchiveCycleParams {
    /// Cycle UUID
    pub id: String,
}
