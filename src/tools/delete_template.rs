use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteTemplateParams {
    /// The template ID
    pub id: String,
}
