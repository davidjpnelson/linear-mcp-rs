use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnarchiveProjectParams {
    /// Project name or UUID to unarchive
    pub id: String,
}
