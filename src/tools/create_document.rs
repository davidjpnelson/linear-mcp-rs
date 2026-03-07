use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateDocumentParams {
    /// Document title
    pub title: String,
    /// Project name to associate the document with (required by Linear)
    pub project: String,
    /// Document content (markdown)
    pub content: Option<String>,
    /// Issue identifier (e.g. 'ENG-123') to associate the document with
    pub issue: Option<String>,
}
