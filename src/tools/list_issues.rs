use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListIssuesParams {
    /// Team key (e.g. 'ENG')
    pub team: Option<String>,
    /// Assignee email or display name
    pub assignee: Option<String>,
    /// Creator email or display name
    pub creator: Option<String>,
    /// Workflow state name (e.g. 'In Progress', 'Done')
    pub status: Option<String>,
    /// Project name
    pub project: Option<String>,
    /// Label name
    pub label: Option<String>,
    /// Priority level
    pub priority: Option<PriorityLevel>,
    /// Filter by exact estimate value
    pub estimate: Option<f64>,
    /// Filter to issues that are blocked by another issue
    #[serde(rename = "hasBlockedByRelation")]
    pub has_blocked_by_relation: Option<bool>,
    /// Filter to issues that are blocking another issue
    #[serde(rename = "hasBlockingRelation")]
    pub has_blocking_relation: Option<bool>,
    /// Sort order (default: updatedAt)
    #[serde(rename = "orderBy")]
    pub order_by: Option<OrderBy>,
    /// Max results (default 25)
    pub limit: Option<u32>,
    /// Pagination cursor from a previous response
    pub cursor: Option<String>,
    /// Filter issues due before this date (ISO format, e.g. '2025-03-01')
    #[serde(rename = "dueBefore")]
    pub due_before: Option<String>,
    /// Filter issues due after this date (ISO format, e.g. '2025-01-01')
    #[serde(rename = "dueAfter")]
    pub due_after: Option<String>,
    /// Filter issues created before this date (ISO format)
    #[serde(rename = "createdBefore")]
    pub created_before: Option<String>,
    /// Filter issues created after this date (ISO format)
    #[serde(rename = "createdAfter")]
    pub created_after: Option<String>,
    /// Filter issues updated before this date (ISO format)
    #[serde(rename = "updatedBefore")]
    pub updated_before: Option<String>,
    /// Filter issues updated after this date (ISO format)
    #[serde(rename = "updatedAfter")]
    pub updated_after: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum PriorityLevel {
    Urgent,
    High,
    Medium,
    Low,
    None,
}

impl PriorityLevel {
    pub fn to_number(&self) -> i32 {
        match self {
            PriorityLevel::Urgent => 1,
            PriorityLevel::High => 2,
            PriorityLevel::Medium => 3,
            PriorityLevel::Low => 4,
            PriorityLevel::None => 0,
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub enum OrderBy {
    #[serde(rename = "createdAt")]
    CreatedAt,
    #[serde(rename = "updatedAt")]
    UpdatedAt,
    #[serde(rename = "priority")]
    Priority,
}

impl OrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderBy::CreatedAt => "createdAt",
            OrderBy::UpdatedAt => "updatedAt",
            OrderBy::Priority => "priority",
        }
    }
}
