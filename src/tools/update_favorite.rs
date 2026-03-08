use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateFavoriteParams {
    /// UUID of the favorite to update
    pub id: String,

    /// Sort order for the favorite in the sidebar (lower values appear first)
    pub sort_order: Option<f64>,

    /// UUID of the parent favorite folder to nest this favorite under
    pub parent_id: Option<String>,

    /// Name of the folder if converting this favorite into a folder
    pub folder_name: Option<String>,
}
