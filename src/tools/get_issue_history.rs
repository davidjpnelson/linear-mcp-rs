use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetIssueHistoryParams {
    /// Issue identifier (e.g. 'ENG-123') or UUID
    pub id: String,
    /// Max results (default 50)
    #[serde(default, deserialize_with = "super::serde_helpers::u32_from_str_or_num")]
    pub limit: Option<u32>,
}
