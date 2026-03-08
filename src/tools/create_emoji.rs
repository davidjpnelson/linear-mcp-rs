use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateEmojiParams {
    /// Emoji name
    pub name: String,
    /// Image URL
    pub url: String,
}
