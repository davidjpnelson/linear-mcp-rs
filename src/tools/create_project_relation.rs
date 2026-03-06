use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateProjectRelationParams {
    /// Source project name or UUID
    pub project: String,
    /// Related project name or UUID
    #[serde(rename = "relatedProject")]
    pub related_project: String,
    /// Relation type: 'blocks', 'dependsOn', or 'related'
    #[serde(rename = "type")]
    pub relation_type: String,
}
