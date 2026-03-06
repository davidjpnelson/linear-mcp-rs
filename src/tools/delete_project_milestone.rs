use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteProjectMilestoneParams {
    /// Project milestone UUID
    pub id: String,
}
