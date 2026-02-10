use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: String,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: i32,
    pub estimate: Option<f64>,
    pub due_date: Option<String>,
    pub branch_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
    pub state: Option<WorkflowState>,
    pub assignee: Option<User>,
    pub team: Option<Team>,
    pub project: Option<Project>,
    pub labels: Option<NodeList<Label>>,
    pub parent: Option<Box<IssueRef>>,
    pub children: Option<NodeList<IssueRef>>,
    pub comments: Option<NodeList<Comment>>,
}

/// Lightweight issue reference for parent/children links.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRef {
    pub identifier: String,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub email: Option<String>,
    pub admin: Option<bool>,
    pub guest: Option<bool>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: String,
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowState {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub state_type: String,
    pub color: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub state: String,
    pub progress: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub body: String,
    pub created_at: String,
    pub user: Option<CommentUser>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentUser {
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NodeList<T> {
    pub nodes: Vec<T>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection<T> {
    pub nodes: Vec<T>,
    pub page_info: PageInfo,
}

/// Viewer (authenticated user) with minimal fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
    pub id: String,
    pub display_name: String,
    pub email: Option<String>,
}

/// Result of a create/update mutation.
#[derive(Debug, Clone, Deserialize)]
pub struct MutationResult {
    pub success: bool,
    pub issue: Option<Issue>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommentMutationResult {
    pub success: bool,
    pub comment: Option<Comment>,
}

// Priority mapping
pub fn priority_label(priority: i32) -> &'static str {
    match priority {
        1 => "Urgent",
        2 => "High",
        3 => "Medium",
        4 => "Low",
        _ => "None",
    }
}

pub fn priority_to_number(s: &str) -> Option<i32> {
    match s.to_lowercase().as_str() {
        "urgent" => Some(1),
        "high" => Some(2),
        "medium" => Some(3),
        "low" => Some(4),
        "none" => Some(0),
        _ => None,
    }
}
