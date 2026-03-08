use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateNotificationSubscriptionParams {
    /// Comma-separated notification types
    pub types: Option<String>,
    /// Team key or UUID
    pub team: Option<String>,
    /// Project name or UUID
    pub project: Option<String>,
    /// Label name or UUID
    pub label: Option<String>,
    /// Whether the subscription is active
    pub active: Option<bool>,
}
