use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteTimeScheduleParams {
    /// The time schedule ID
    pub id: String,
}
