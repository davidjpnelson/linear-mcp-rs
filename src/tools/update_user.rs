use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateUserParams {
    /// User's email address or UUID to update
    pub id: String,

    /// New display name for the user
    pub display_name: Option<String>,

    /// New bio/description for the user
    pub description: Option<String>,

    /// Emoji to show as the user's status (e.g. "🏠", "🏖️")
    pub status_emoji: Option<String>,

    /// Text label to show as the user's status (e.g. "Working from home", "On vacation")
    pub status_label: Option<String>,
}
