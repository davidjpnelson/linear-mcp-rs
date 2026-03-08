use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCustomerStatusParams {
    /// UUID of the customer status to update
    pub id: String,
    /// New name for the customer status
    pub name: Option<String>,
    /// New hex color code (e.g. "#ff0000")
    pub color: Option<String>,
    /// New description
    pub description: Option<String>,
    /// New position in the list
    pub position: Option<f64>,
    /// New display name
    pub display_name: Option<String>,
}
