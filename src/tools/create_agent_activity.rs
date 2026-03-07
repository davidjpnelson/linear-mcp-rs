use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateAgentActivityParams {
    /// Agent session UUID
    pub session: String,
    /// Activity type — stored in content. Values 'stop', 'continue', 'auth', 'select' also set the signal enum.
    #[serde(rename = "activityType")]
    pub activity_type: String,
    /// Activity body/description
    pub body: Option<String>,
    /// Action performed
    pub action: Option<String>,
    /// Action parameter
    pub parameter: Option<String>,
    /// Action result
    pub result: Option<String>,
    /// Whether this is an ephemeral activity (not persisted long-term)
    pub ephemeral: Option<bool>,
}
