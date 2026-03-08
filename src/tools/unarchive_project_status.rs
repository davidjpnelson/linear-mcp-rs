use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnarchiveProjectStatusParams {
    /// UUID of the project status to unarchive
    pub id: String,
}
