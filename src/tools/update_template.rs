use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateTemplateParams {
    /// The template ID
    pub id: String,
    /// Template name
    pub name: Option<String>,
    /// Template description
    pub description: Option<String>,
    /// JSON string of template data
    pub template_data: Option<String>,
    /// Team key or UUID
    pub team: Option<String>,
}
