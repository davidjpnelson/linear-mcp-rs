use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetViewIssuesParams {
    /// Custom view ID (UUID)
    pub id: String,
    /// Max issues to return (default 50)
    #[serde(default, deserialize_with = "super::serde_helpers::i32_from_str_or_num")]
    pub limit: Option<i32>,
}
