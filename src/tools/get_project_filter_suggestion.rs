use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetProjectFilterSuggestionParams {
    /// Natural language prompt describing the filter
    pub prompt: String,
}
