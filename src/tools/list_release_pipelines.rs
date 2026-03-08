use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListReleasePipelinesParams {
    /// Maximum number of release pipelines to return
    pub limit: Option<i32>,
}
