use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArchiveInitiativeUpdateParams {
    /// UUID of the initiative update to archive
    pub id: String,
}
