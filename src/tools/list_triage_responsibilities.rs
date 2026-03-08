use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListTriageResponsibilitiesParams {
    /// Maximum number of results to return
    pub limit: Option<i32>,
}
