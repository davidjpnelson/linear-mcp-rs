use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetEmojiParams {
    /// The emoji ID
    pub id: String,
}
