use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListTriageIssuesParams {
    /// Team key (e.g. "ENG") — required since triage is per-team
    pub team: String,
    /// Max results to return (default 50)
    #[serde(default, deserialize_with = "super::serde_helpers::i32_from_str_or_num")]
    pub limit: Option<i32>,
}
