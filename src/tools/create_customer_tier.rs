use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateCustomerTierParams {
    /// Name of the customer tier
    pub name: String,
    /// Hex color code for the customer tier (e.g. "#ff0000")
    pub color: String,
    /// Description of the customer tier
    pub description: Option<String>,
    /// Position of the customer tier in the list
    pub position: Option<f64>,
    /// Display name for the customer tier
    pub display_name: Option<String>,
}
