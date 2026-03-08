use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnarchiveDocumentParams {
    /// UUID of the document to unarchive
    pub id: String,
}
