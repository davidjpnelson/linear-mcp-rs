use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateEmailIntakeAddressParams {
    /// Team key
    pub team: Option<String>,
    /// Template UUID
    pub template: Option<String>,
    /// Sender display name
    pub sender_name: Option<String>,
    /// Whether replies are enabled
    pub replies_enabled: Option<bool>,
    /// Whether customer requests are enabled
    pub customer_requests_enabled: Option<bool>,
}
