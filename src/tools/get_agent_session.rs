use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetAgentSessionParams {
    /// Agent session UUID
    pub id: String,
}
