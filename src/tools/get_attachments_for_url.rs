use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetAttachmentsForUrlParams {
    /// URL to search attachments for
    pub url: String,
}
