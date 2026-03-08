use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteProjectLabelParams {
    /// UUID of the project label to delete
    pub id: String,
}
