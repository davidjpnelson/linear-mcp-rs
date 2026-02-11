use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArchiveLabelParams {
    /// Label UUID to archive
    pub id: String,
}
