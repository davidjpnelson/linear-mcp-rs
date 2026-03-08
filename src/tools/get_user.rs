use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetUserParams {
    /// User's email address or UUID to look up
    pub id: String,
}
