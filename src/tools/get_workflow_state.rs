use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetWorkflowStateParams {
    /// Workflow state UUID
    pub id: String,
}
