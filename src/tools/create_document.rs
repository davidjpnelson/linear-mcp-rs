use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateDocumentParams {
    /// Document title
    pub title: String,
    /// Document content (markdown)
    pub content: Option<String>,
    /// Project name to associate the document with
    pub project: Option<String>,
}
