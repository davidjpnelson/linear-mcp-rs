use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetApplicationInfoParams {
    /// OAuth application client ID to look up
    pub client_id: String,
}
