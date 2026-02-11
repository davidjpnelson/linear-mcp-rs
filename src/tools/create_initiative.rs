use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateInitiativeParams {
    /// Name of the initiative (required)
    pub name: String,
    /// Description of the initiative
    pub description: Option<String>,
    /// Status: Planned, Active, or Completed
    pub status: Option<String>,
    /// Owner email address
    pub owner: Option<String>,
    /// Target completion date (YYYY-MM-DD)
    #[serde(rename = "targetDate")]
    pub target_date: Option<String>,
}
