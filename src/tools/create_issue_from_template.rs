use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateIssueFromTemplateParams {
    /// Template ID (UUID) — use list_templates to find available templates
    #[serde(rename = "templateId")]
    pub template_id: String,
    /// Team key (e.g. "ENG") — required
    pub team: String,
    /// Override the template's default title
    pub title: Option<String>,
    /// Override the template's default description
    pub description: Option<String>,
    /// Assignee email address
    pub assignee: Option<String>,
    /// Priority: urgent, high, normal, low, none
    pub priority: Option<String>,
}
