use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnarchiveReleaseParams {
    /// UUID of the release to unarchive
    pub id: String,
}
