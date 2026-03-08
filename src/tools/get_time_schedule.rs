use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTimeScheduleParams {
    /// The time schedule ID
    pub id: String,
}
