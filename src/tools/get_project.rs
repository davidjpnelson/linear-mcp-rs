use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetProjectParams {
    /// Project name or UUID
    pub id: String,
}
