use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteEmailIntakeAddressParams {
    /// The email intake address ID
    pub id: String,
}
