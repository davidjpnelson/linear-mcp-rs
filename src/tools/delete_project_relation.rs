use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteProjectRelationParams {
    /// Project relation UUID
    pub id: String,
}
