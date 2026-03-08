use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateNotificationSubscriptionParams {
    /// The notification subscription ID
    pub id: String,
    /// Comma-separated notification types
    pub types: Option<String>,
    /// Whether the subscription is active
    pub active: Option<bool>,
}
