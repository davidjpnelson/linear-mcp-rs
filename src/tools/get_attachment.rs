use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetAttachmentParams {
    /// UUID of the attachment to retrieve
    pub id: String,
}
