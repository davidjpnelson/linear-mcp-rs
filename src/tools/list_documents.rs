use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListDocumentsParams {
    /// Max results (default 50)
    pub limit: Option<u32>,
}
