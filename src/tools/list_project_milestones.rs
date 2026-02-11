use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListProjectMilestonesParams {
    /// Project name or UUID
    pub project: String,
}
