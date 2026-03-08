use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateTemplateParams {
    /// Template name
    pub name: String,
    /// "issue", "project", or "document"
    #[serde(rename = "type")]
    pub template_type: String,
    /// JSON string of template data
    pub template_data: String,
    /// Team key or UUID
    pub team: Option<String>,
    /// Template description
    pub description: Option<String>,
}
