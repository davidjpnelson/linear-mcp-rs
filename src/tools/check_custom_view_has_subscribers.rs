use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CheckCustomViewHasSubscribersParams {
    /// The custom view ID
    pub id: String,
}
