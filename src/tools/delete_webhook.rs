use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteWebhookParams {
    /// Webhook UUID to delete
    pub id: String,
}
