use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveProjectFromInitiativeParams {
    /// Initiative-to-project link UUID
    pub id: String,
}
