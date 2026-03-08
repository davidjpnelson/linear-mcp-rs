use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetEntityExternalLinkParams {
    /// The entity external link ID
    pub id: String,
}
