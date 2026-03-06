use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCustomerNeedParams {
    /// Customer need UUID
    pub id: String,
    /// Issue identifier (e.g. 'ENG-123') or UUID
    pub issue: Option<String>,
    /// Customer UUID
    pub customer: Option<String>,
    /// Need description
    pub body: Option<String>,
    /// Priority (0=none, 1=urgent, 2=high, 3=medium, 4=low)
    #[serde(default, deserialize_with = "super::serde_helpers::u32_from_str_or_num")]
    pub priority: Option<u32>,
}
