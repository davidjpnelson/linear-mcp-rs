use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateAgentSessionParams {
    /// Issue identifier (e.g. 'ENG-123') or UUID — mutually exclusive with comment
    pub issue: Option<String>,
    /// Comment UUID — mutually exclusive with issue
    pub comment: Option<String>,
    /// External link URL for the agent session
    #[serde(rename = "externalLink")]
    pub external_link: Option<String>,
}
