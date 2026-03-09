use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCustomViewSuggestionParams {
    /// Model name to suggest for (e.g. "issues", "projects")
    pub model_name: String,
    /// Filter object to get suggestions for
    pub filter: Option<serde_json::Value>,
}
