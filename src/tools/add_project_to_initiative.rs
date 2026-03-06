use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddProjectToInitiativeParams {
    /// Initiative name or UUID
    pub initiative: String,
    /// Project name or UUID
    pub project: String,
}
