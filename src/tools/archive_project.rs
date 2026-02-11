use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArchiveProjectParams {
    /// Project name or UUID to archive
    pub id: String,
}
