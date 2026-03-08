use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteEntityExternalLinkParams {
    /// The entity external link ID
    pub id: String,
}
