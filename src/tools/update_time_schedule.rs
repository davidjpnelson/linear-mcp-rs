use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateTimeScheduleParams {
    /// The time schedule ID
    pub id: String,
    /// Schedule name
    pub name: Option<String>,
    /// JSON array of schedule entries
    pub entries: Option<String>,
    /// External identifier
    pub external_id: Option<String>,
    /// External URL
    pub external_url: Option<String>,
}
