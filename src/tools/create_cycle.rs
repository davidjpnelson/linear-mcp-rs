use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateCycleParams {
    /// Team key (e.g. 'ENG') — required
    pub team: String,
    /// Cycle name
    pub name: Option<String>,
    /// Start date (ISO format, e.g. '2025-01-01')
    #[serde(rename = "startsAt")]
    pub starts_at: String,
    /// End date (ISO format, e.g. '2025-01-15')
    #[serde(rename = "endsAt")]
    pub ends_at: String,
}
