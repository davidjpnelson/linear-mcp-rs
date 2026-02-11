use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListTriageIssuesParams {
    /// Team key (e.g. "ENG") — required since triage is per-team
    pub team: String,
    /// Max results to return (default 50)
    pub limit: Option<i32>,
}
