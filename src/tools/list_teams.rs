use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListTeamsParams {
    /// Include member counts per team (default false, saves API calls)
    #[serde(rename = "includeMemberCount")]
    pub include_member_count: Option<bool>,
}
