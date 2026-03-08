use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteReleaseParams {
    /// UUID of the release to delete
    pub id: String,
}
