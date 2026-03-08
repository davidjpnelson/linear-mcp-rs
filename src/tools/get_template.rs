use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTemplateParams {
    /// The template ID
    pub id: String,
}
