use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCyclesParams {
    /// Team key (e.g. 'ENG') — required
    pub team: String,
    /// Max results (default 25)
    pub limit: Option<u32>,
}
