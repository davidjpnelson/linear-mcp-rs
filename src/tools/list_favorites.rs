use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListFavoritesParams {
    /// Max results (default 50)
    pub limit: Option<u32>,
}
