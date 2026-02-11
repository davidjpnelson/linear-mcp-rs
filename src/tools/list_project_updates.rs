use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListProjectUpdatesParams {
    /// Project name or UUID
    pub project: String,
}
