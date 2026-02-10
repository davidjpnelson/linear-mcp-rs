use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListIssuesParams {
    /// Team key (e.g. 'ENG')
    pub team: Option<String>,
    /// Assignee email or display name
    pub assignee: Option<String>,
    /// Workflow state name (e.g. 'In Progress', 'Done')
    pub status: Option<String>,
    /// Project name
    pub project: Option<String>,
    /// Label name
    pub label: Option<String>,
    /// Priority level
    pub priority: Option<PriorityLevel>,
    /// Sort order (default: updatedAt)
    #[serde(rename = "orderBy")]
    pub order_by: Option<OrderBy>,
    /// Max results (default 25)
    pub limit: Option<u32>,
    /// Pagination cursor from a previous response
    pub cursor: Option<String>,
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
}

impl OrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderBy::CreatedAt => "createdAt",
            OrderBy::UpdatedAt => "updatedAt",
        }
    }
}
