use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateEmailIntakeAddressParams {
    /// The email intake address ID
    pub id: String,
    /// Whether the address is enabled
    pub enabled: Option<bool>,
    /// Sender display name
    pub sender_name: Option<String>,
    /// Team key
    pub team: Option<String>,
    /// Template UUID
    pub template: Option<String>,
    /// Whether replies are enabled
    pub replies_enabled: Option<bool>,
    /// Whether customer requests are enabled
    pub customer_requests_enabled: Option<bool>,
}
