use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCustomerNeedsParams {
    /// Max results (default 25)
    #[serde(default, deserialize_with = "super::serde_helpers::u32_from_str_or_num")]
    pub limit: Option<u32>,
}
