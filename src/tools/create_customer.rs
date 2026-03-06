use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateCustomerParams {
    /// Customer name — required
    pub name: String,
    /// Comma-separated domain names (e.g. 'example.com, test.org')
    pub domains: Option<String>,
    /// Owner email address
    pub owner: Option<String>,
    /// Annual revenue in dollars
    #[serde(default, deserialize_with = "super::serde_helpers::f64_from_str_or_num")]
    pub revenue: Option<f64>,
    /// Company size (number of employees)
    #[serde(default, deserialize_with = "super::serde_helpers::u32_from_str_or_num")]
    pub size: Option<u32>,
}
