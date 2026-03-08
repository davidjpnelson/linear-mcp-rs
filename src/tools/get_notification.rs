use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetNotificationParams {
    /// UUID of the notification to retrieve
    pub id: String,
}
