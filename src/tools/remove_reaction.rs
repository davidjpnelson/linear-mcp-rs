use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveReactionParams {
    /// Reaction UUID to remove
    pub id: String,
}
