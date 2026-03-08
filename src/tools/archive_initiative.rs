use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArchiveInitiativeParams {
    /// Name or UUID of the initiative to archive
    pub id: String,
}
