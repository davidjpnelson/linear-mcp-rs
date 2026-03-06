use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateCustomerNeedParams {
    /// Issue identifier (e.g. 'ENG-123') or UUID
    pub issue: String,
    /// Customer UUID
    pub customer: String,
    /// Need description
    pub body: Option<String>,
    /// Priority (0=none, 1=urgent, 2=high, 3=medium, 4=low)
    #[serde(default, deserialize_with = "super::serde_helpers::u32_from_str_or_num")]
    pub priority: Option<u32>,
}
