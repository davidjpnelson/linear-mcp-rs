use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArchiveProjectStatusParams {
    /// UUID of the project status to archive
    pub id: String,
}
