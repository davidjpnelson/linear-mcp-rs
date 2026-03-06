use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteProjectUpdateParams {
    /// Project update UUID
    pub id: String,
}
