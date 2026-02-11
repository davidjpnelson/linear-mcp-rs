use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveFavoriteParams {
    /// Favorite UUID to remove
    pub id: String,
}
