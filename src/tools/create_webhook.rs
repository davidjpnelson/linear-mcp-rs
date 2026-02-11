use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateWebhookParams {
    /// Webhook URL endpoint
    pub url: String,
    /// Webhook label/name
    pub label: Option<String>,
    /// Comma-separated resource types (e.g. 'Issue, Comment, Project')
    #[serde(rename = "resourceTypes")]
    pub resource_types: Option<String>,
    /// Team key to scope the webhook to (e.g. 'ENG')
    pub team: Option<String>,
}
