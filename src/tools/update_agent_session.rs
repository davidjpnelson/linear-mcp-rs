use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateAgentSessionParams {
    /// Agent session UUID
    pub id: String,
    /// Plan as JSON string
    pub plan: Option<String>,
    /// External link URL
    #[serde(rename = "externalLink")]
    pub external_link: Option<String>,
}
