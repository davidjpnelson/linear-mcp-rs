use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddReactionParams {
    /// Comment UUID to react to
    #[serde(rename = "commentId")]
    pub comment_id: String,
    /// Emoji name (e.g. 'thumbsup', 'heart')
    pub emoji: String,
}
