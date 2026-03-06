use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCycleParams {
    /// Cycle UUID
    pub id: String,
    /// New cycle name
    pub name: Option<String>,
    /// New description
    pub description: Option<String>,
    /// New start date (ISO format, e.g. '2025-01-01')
    #[serde(rename = "startsAt")]
    pub starts_at: Option<String>,
    /// New end date (ISO format, e.g. '2025-01-15')
    #[serde(rename = "endsAt")]
    pub ends_at: Option<String>,
}
