use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCustomViewSuggestionParams {
    /// Natural language prompt describing the view
    pub prompt: String,
}
