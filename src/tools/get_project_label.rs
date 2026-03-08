use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetProjectLabelParams {
    /// UUID of the project label
    pub id: String,
}
