use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateLabelParams {
    /// Label UUID
    pub id: String,
    /// New label name
    pub name: Option<String>,
    /// New hex color (e.g. '#ff0000')
    pub color: Option<String>,
}
