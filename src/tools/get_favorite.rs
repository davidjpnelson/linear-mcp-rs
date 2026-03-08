use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetFavoriteParams {
    /// UUID of the favorite to retrieve
    pub id: String,
}
