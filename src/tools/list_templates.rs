use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListTemplatesParams {
    /// Max results (default 50)
    #[allow(dead_code)]
    pub limit: Option<u32>,
}
