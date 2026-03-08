use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteProjectParams {
    /// Project name or UUID to delete
    pub id: String,
}
