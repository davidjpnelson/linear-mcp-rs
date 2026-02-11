use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateInitiativeParams {
    /// Initiative ID (UUID)
    pub id: String,
    /// New name
    pub name: Option<String>,
    /// New description
    pub description: Option<String>,
    /// Status: Planned, Active, or Completed
    pub status: Option<String>,
    /// Owner email address (use "none" to unset)
    pub owner: Option<String>,
    /// Target completion date (YYYY-MM-DD)
    #[serde(rename = "targetDate")]
    pub target_date: Option<String>,
}
