use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MergeCustomersParams {
    /// UUID of the source customer to merge from
    pub source_id: String,
    /// UUID of the target customer to merge into
    pub target_id: String,
}
