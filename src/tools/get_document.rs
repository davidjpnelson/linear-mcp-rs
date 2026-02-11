use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDocumentParams {
    /// Document UUID
    pub id: String,
}
