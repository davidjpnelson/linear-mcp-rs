use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateAttachmentParams {
    /// Attachment UUID
    pub id: String,
    /// New title (required by Linear API)
    pub title: String,
    /// New subtitle
    pub subtitle: Option<String>,
}
