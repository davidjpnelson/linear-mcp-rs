use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddFavoriteParams {
    /// Issue identifier (e.g. 'ENG-123') to favorite
    #[serde(rename = "issueId")]
    pub issue_id: Option<String>,
    /// Project name or UUID to favorite
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
}
