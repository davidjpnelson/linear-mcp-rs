use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetProjectMilestoneParams {
    /// UUID of the project milestone to retrieve
    pub id: String,
}
