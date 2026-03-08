use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnarchiveInitiativeParams {
    /// Name or UUID of the initiative to unarchive
    pub id: String,
}
