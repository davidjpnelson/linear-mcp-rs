use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateDocumentParams {
    /// Document UUID
    pub id: String,
    /// New title
    pub title: Option<String>,
    /// New content (markdown)
    pub content: Option<String>,
}
