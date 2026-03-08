use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteEmojiParams {
    /// The emoji ID
    pub id: String,
}
