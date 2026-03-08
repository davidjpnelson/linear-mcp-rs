use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetEmailIntakeAddressParams {
    /// The email intake address ID
    pub id: String,
}
