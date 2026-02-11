use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateIssueRelationParams {
    /// Source issue identifier (e.g. 'ENG-123') or UUID
    #[serde(rename = "issueId")]
    pub issue_id: String,
    /// Related issue identifier (e.g. 'ENG-456') or UUID
    #[serde(rename = "relatedIssueId")]
    pub related_issue_id: String,
    /// Relation type
    #[serde(rename = "type")]
    pub relation_type: RelationType,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    Blocks,
    BlockedBy,
    Related,
    Duplicate,
}

impl RelationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationType::Blocks => "blocks",
            RelationType::BlockedBy => "blocked_by",
            RelationType::Related => "related",
            RelationType::Duplicate => "duplicate",
        }
    }
}
