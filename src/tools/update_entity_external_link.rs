use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateEntityExternalLinkParams {
    /// The entity external link ID
    pub id: String,
    /// The URL of the external link
    pub url: Option<String>,
    /// Display label for the link
    pub label: Option<String>,
}
