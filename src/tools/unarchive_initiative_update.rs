use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnarchiveInitiativeUpdateParams {
    /// UUID of the initiative update to unarchive
    pub id: String,
}
