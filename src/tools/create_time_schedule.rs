use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateTimeScheduleParams {
    /// Schedule name
    pub name: String,
    /// JSON array of schedule entries
    pub entries: String,
    /// External identifier
    pub external_id: Option<String>,
    /// External URL
    pub external_url: Option<String>,
}
