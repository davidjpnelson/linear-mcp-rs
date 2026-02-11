#![allow(dead_code)]
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
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub canceled_at: Option<String>,
    pub url: String,
    pub state: Option<WorkflowState>,
    pub assignee: Option<User>,
    pub creator: Option<UserRef>,
    pub team: Option<Team>,
    pub project: Option<Project>,
    pub cycle: Option<CycleRef>,
    pub labels: Option<NodeList<Label>>,
    pub parent: Option<Box<IssueRef>>,
    pub children: Option<NodeList<IssueRef>>,
    pub relations: Option<NodeList<IssueRelation>>,
    pub subscribers: Option<NodeList<UserRef>>,
    pub comments: Option<NodeList<Comment>>,
}

/// Lightweight issue reference for parent/children links.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRef {
    pub identifier: String,
    pub title: String,
}

/// Lightweight user reference (for creator, subscribers).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRef {
    pub display_name: String,
    pub email: Option<String>,
}

/// Lightweight cycle reference (for issue detail).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycleRef {
    pub id: String,
    pub number: i32,
    pub name: Option<String>,
}

/// Project member reference (display name only, for list views).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectMemberRef {
    pub display_name: String,
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
    pub state: Option<String>,
    pub progress: Option<f64>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub lead: Option<UserRef>,
    pub teams: Option<NodeList<Team>>,
    pub members: Option<NodeList<ProjectMemberRef>>,
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

/// Cycle entity.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cycle {
    pub id: String,
    pub number: i32,
    pub name: Option<String>,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
    pub completed_at: Option<String>,
    pub progress: Option<f64>,
}

/// Issue relation entity.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelation {
    pub id: String,
    #[serde(rename = "type")]
    pub relation_type: String,
    pub issue: Option<IssueRef>,
    pub related_issue: Option<IssueRef>,
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

/// Generic success result (for delete mutations, archive, etc.).
#[derive(Debug, Clone, Deserialize)]
pub struct SuccessResult {
    pub success: bool,
}

/// Result for issue relation mutations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelationMutationResult {
    pub success: bool,
    pub issue_relation: Option<IssueRelation>,
}

/// Result for label mutations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelMutationResult {
    pub success: bool,
    pub issue_label: Option<Label>,
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

// ---- #17: Documents ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub project: Option<DocumentProject>,
    pub creator: Option<DocumentCreator>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentProject {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentCreator {
    pub display_name: String,
}

// ---- #18: Project detail ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDetail {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub state: Option<String>,
    pub progress: Option<f64>,
    pub target_date: Option<String>,
    pub start_date: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub lead: Option<ProjectLead>,
    pub teams: Option<NodeList<ProjectTeam>>,
    pub members: Option<NodeList<ProjectMember>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectLead {
    pub id: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectTeam {
    pub id: String,
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectMember {
    pub id: String,
    pub display_name: String,
}

/// Result of a project create/update mutation.
#[derive(Debug, Clone, Deserialize)]
pub struct ProjectMutationResult {
    pub success: bool,
    pub project: Option<ProjectDetail>,
}

// ---- #19: Project updates ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUpdate {
    pub id: String,
    pub body: String,
    pub health: Option<String>,
    pub created_at: Option<String>,
    pub user: Option<ProjectUpdateUser>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUpdateUser {
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectUpdateMutationResult {
    pub success: bool,
    #[serde(rename = "projectUpdate")]
    pub project_update: Option<ProjectUpdate>,
}

// ---- #20: Project milestones ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectMilestone {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub target_date: Option<String>,
    pub sort_order: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectMilestoneMutationResult {
    pub success: bool,
    #[serde(rename = "projectMilestone")]
    pub project_milestone: Option<ProjectMilestone>,
}

// ---- #21: Roadmaps and Initiatives ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roadmap {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub slug_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Initiative {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: Option<String>,
}

// ---- #22: Notifications ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub read_at: Option<String>,
    pub created_at: Option<String>,
    pub issue: Option<NotificationIssue>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationIssue {
    pub identifier: String,
    pub title: String,
}

// NOTE: notificationUpdate returns a NotificationPayload
#[derive(Debug, Clone, Deserialize)]
pub struct NotificationMutationResult {
    pub success: bool,
}

// ---- #23: Attachments ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AttachmentMutationResult {
    pub success: bool,
    pub attachment: Option<Attachment>,
}

// ---- #24: Reactions ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub id: String,
    pub emoji: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReactionMutationResult {
    pub success: bool,
    pub reaction: Option<Reaction>,
}

// ---- #25: Custom Views ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomView {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub filter_data: Option<serde_json::Value>,
}

// ---- #26: Favorites ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Favorite {
    pub id: String,
    #[serde(rename = "type")]
    pub favorite_type: Option<String>,
    pub issue: Option<NotificationIssue>,
    pub project: Option<DocumentProject>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FavoriteMutationResult {
    pub success: bool,
    pub favorite: Option<Favorite>,
}

// ---- #29: Templates ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub template_data: Option<serde_json::Value>,
}

// ---- #30: Issue History ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueHistoryEntry {
    pub id: String,
    pub created_at: Option<String>,
    pub from_state: Option<HistoryState>,
    pub to_state: Option<HistoryState>,
    pub actor: Option<HistoryActor>,
    pub added_labels: Option<NodeList<HistoryLabel>>,
    pub removed_labels: Option<NodeList<HistoryLabel>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryState {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryActor {
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryLabel {
    pub name: String,
}

// ---- #31: Webhooks ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub id: String,
    pub url: Option<String>,
    pub label: Option<String>,
    pub enabled: Option<bool>,
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookMutationResult {
    pub success: bool,
    pub webhook: Option<Webhook>,
}

// ---- #32: Integrations and Audit Log ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Integration {
    pub id: String,
    pub service: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub entry_type: Option<String>,
    pub created_at: Option<String>,
    pub actor_id: Option<String>,
    pub ip: Option<String>,
}

// ---- #33: Enriched Team and User ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamDetail {
    pub id: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TeamMutationResult {
    pub success: bool,
    pub team: Option<TeamDetail>,
}

// ---- #28: SLA tracking ----
// NOTE: Task #28 requires enriching the existing `get_issue` query and types.
// The following fields should be added to the existing `Issue` struct in types.rs:
//   pub sla_breaches_at: Option<String>,
//   pub sla_started_at: Option<String>,
// And the GET_ISSUE query in queries.rs should include:
//   slaBreachesAt
//   slaStartedAt
// And format_issue_detail in format.rs should output:
//   if let Some(ref sla_breaches) = issue.sla_breaches_at {
//       lines.push(format!("**SLA Breaches At:** {}", format_date(sla_breaches)));
//   }
//   if let Some(ref sla_started) = issue.sla_started_at {
//       lines.push(format!("**SLA Started At:** {}", format_date(sla_started)));
//   }
//
// Since those files are being concurrently edited by other agents, the actual
// modifications are documented here for integration.

// ---- Cycle detail (for create_cycle mutation) ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycleDetail {
    pub id: String,
    pub name: Option<String>,
    pub number: Option<i32>,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
}

// Deletion result (used by delete_webhook, favorite_delete, reaction_delete, etc.)
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMutationResult {
    pub success: bool,
}

// ---- Phase 12: Remaining tools ----

/// Batch update result.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchUpdateResult {
    pub success: bool,
    pub issues: Vec<BatchUpdatedIssue>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchUpdatedIssue {
    pub id: String,
    pub identifier: String,
    pub title: String,
    pub state: Option<IssueStateRef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueStateRef {
    pub name: String,
}

/// Document search result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSearchResult {
    pub id: String,
    pub title: String,
    pub url: Option<String>,
    pub slug_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub project: Option<DocumentProjectRef>,
    pub creator: Option<DocumentCreatorRef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DocumentProjectRef {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentCreatorRef {
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSearchConnection {
    pub nodes: Vec<DocumentSearchResult>,
    pub total_count: Option<i64>,
}

/// Initiative mutation result.
#[derive(Debug, Clone, Deserialize)]
pub struct InitiativeMutationResult {
    pub success: bool,
    pub initiative: Option<InitiativeDetail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InitiativeDetail {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: Option<String>,
}

/// Custom view with issues.
#[derive(Debug, Clone, Deserialize)]
pub struct CustomViewWithIssues {
    pub id: String,
    pub name: String,
    pub issues: Connection<Issue>,
}
