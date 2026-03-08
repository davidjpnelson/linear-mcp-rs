use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListReleaseStagesParams {
    /// Maximum number of release stages to return
    pub limit: Option<i32>,
}
