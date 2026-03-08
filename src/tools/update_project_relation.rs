use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateProjectRelationParams {
    /// UUID of the project relation to update
    pub id: String,

    /// The type of anchor for the source project in the relation (e.g. "relatedTo", "blocks", "dependsOn")
    pub anchor_type: Option<String>,

    /// The type of anchor for the related project in the relation (e.g. "relatedTo", "blocks", "dependsOn")
    pub related_anchor_type: Option<String>,
}
