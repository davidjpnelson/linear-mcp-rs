use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateInitiativeRelationParams {
    /// Name or UUID of the initiative
    pub initiative: String,
    /// Name or UUID of the related initiative
    pub related_initiative: String,
}
