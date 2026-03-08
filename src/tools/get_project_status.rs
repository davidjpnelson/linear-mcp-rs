use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetProjectStatusParams {
    /// UUID of the project status
    pub id: String,
}
