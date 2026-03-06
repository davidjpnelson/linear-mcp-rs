use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteAttachmentParams {
    /// Attachment UUID
    pub id: String,
}
