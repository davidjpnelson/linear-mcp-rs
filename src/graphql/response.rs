use serde::Deserialize;

/// Top-level GraphQL response envelope.
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
}

// ---- Query response data shapes ----

#[derive(Debug, Deserialize)]
pub struct ViewerData {
    pub viewer: crate::types::Viewer,
}

#[derive(Debug, Deserialize)]
pub struct IssuesData {
    pub issues: crate::types::Connection<crate::types::Issue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchIssuesData {
    pub search_issues: crate::types::Connection<crate::types::Issue>,
}

#[derive(Debug, Deserialize)]
pub struct IssueData {
    pub issue: crate::types::Issue,
}

#[derive(Debug, Deserialize)]
pub struct TeamsData {
    pub teams: crate::types::NodeList<crate::types::Team>,
}

#[derive(Debug, Deserialize)]
pub struct TeamsWithMembersData {
    pub teams: crate::types::NodeList<TeamWithMembers>,
}

#[derive(Debug, Deserialize)]
pub struct TeamWithMembers {
    pub id: String,
    pub key: String,
    pub name: String,
    pub members: Option<MemberCountList>,
}

#[derive(Debug, Deserialize)]
pub struct MemberCountList {
    pub nodes: Vec<MemberId>,
}

#[derive(Debug, Deserialize)]
pub struct MemberId {
    #[allow(dead_code)]
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectsData {
    pub projects: crate::types::NodeList<crate::types::Project>,
}

#[derive(Debug, Deserialize)]
pub struct UsersData {
    pub users: crate::types::NodeList<crate::types::User>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStatesData {
    pub workflow_states: crate::types::NodeList<WorkflowStateWithTeam>,
}

/// Workflow state with embedded team info (for list_states grouping).
#[derive(Debug, Deserialize)]
pub struct WorkflowStateWithTeam {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub state_type: String,
    pub color: String,
    pub team: crate::types::Team,
}

// ---- Mutation response data shapes ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIssueData {
    pub issue_create: crate::types::MutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIssueData {
    pub issue_update: crate::types::MutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCommentData {
    pub comment_create: crate::types::CommentMutationResult,
}
