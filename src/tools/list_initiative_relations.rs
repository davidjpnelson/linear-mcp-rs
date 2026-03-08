use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListInitiativeRelationsParams {
    /// Maximum number of results to return
    pub limit: Option<i32>,
}
