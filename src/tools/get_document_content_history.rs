use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDocumentContentHistoryParams {
    /// UUID of the document whose content history to retrieve
    pub id: String,

    /// Maximum number of history entries to return
    pub limit: Option<i32>,
}
