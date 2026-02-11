use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MarkNotificationReadParams {
    /// Notification UUID
    pub id: String,
}
