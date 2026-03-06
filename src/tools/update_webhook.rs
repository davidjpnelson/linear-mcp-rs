use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateWebhookParams {
    /// Webhook UUID
    pub id: String,
    /// New webhook URL endpoint
    pub url: Option<String>,
    /// New webhook label/name
    pub label: Option<String>,
    /// Whether the webhook is enabled
    pub enabled: Option<bool>,
    /// Comma-separated resource types (e.g. 'Issue, Comment, Project')
    #[serde(rename = "resourceTypes")]
    pub resource_types: Option<String>,
}
