use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AttachLinkUrlParams {
    /// Issue identifier or UUID
    pub issue: String,
    /// URL to attach
    pub url: String,
    /// Optional title for the link
    pub title: Option<String>,
}
