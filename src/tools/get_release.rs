use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetReleaseParams {
    /// UUID of the release
    pub id: String,
}
