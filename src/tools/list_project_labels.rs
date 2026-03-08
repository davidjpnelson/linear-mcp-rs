use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListProjectLabelsParams {
    /// Maximum number of project labels to return
    pub limit: Option<i32>,
}
