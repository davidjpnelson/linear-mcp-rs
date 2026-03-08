use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateCustomerStatusParams {
    /// Name of the customer status
    pub name: String,
    /// Hex color code for the customer status (e.g. "#ff0000")
    pub color: String,
    /// Description of the customer status
    pub description: Option<String>,
    /// Position of the customer status in the list
    pub position: Option<f64>,
    /// Display name for the customer status
    pub display_name: Option<String>,
}
