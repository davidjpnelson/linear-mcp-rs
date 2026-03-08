use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchIssueFigmaFileKeyParams {
    /// Figma file key to search for
    pub file_key: String,
}
