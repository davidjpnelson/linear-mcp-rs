use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateEntityExternalLinkParams {
    /// The URL of the external link
    pub url: String,
    /// Display label for the link
    pub label: String,
    /// Initiative name or UUID
    pub initiative: Option<String>,
    /// Project name or UUID
    pub project: Option<String>,
    /// Team key or UUID
    pub team: Option<String>,
}
