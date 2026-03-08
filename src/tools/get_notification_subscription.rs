use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetNotificationSubscriptionParams {
    /// The notification subscription ID
    pub id: String,
}
