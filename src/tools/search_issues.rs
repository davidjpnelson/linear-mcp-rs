use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchIssuesParams {
    /// Search text
    pub query: String,
    /// Limit search to this team key
    pub team: Option<String>,
    /// Filter by workflow state name
    pub status: Option<String>,
    /// Filter by assignee email or name
    pub assignee: Option<String>,
    /// Max results (default 25)
    #[serde(default, deserialize_with = "super::serde_helpers::u32_from_str_or_num")]
    pub limit: Option<u32>,
    /// Pagination cursor from a previous response
    pub cursor: Option<String>,
}
