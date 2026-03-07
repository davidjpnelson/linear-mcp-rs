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
    pub sla_breaches_at: Option<String>,
    pub sla_started_at: Option<String>,
    pub sla_type: Option<String>,
    pub customer_ticket_count: Option<i32>,
    pub previous_identifiers: Option<Vec<String>>,
    pub auto_closed_at: Option<String>,
    pub auto_archived_at: Option<String>,
    pub trashed: Option<bool>,
    pub snoozed_until_at: Option<String>,
    pub project_milestone: Option<ProjectMilestoneRef>,
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

/// Lightweight project milestone reference.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectMilestoneRef {
    pub id: String,
    pub name: String,
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
    pub description: Option<String>,
    pub timezone: Option<String>,
    pub triage_enabled: Option<bool>,
    pub default_issue_state: Option<TeamDefaultState>,
}

/// Default issue state reference for teams.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamDefaultState {
    pub id: String,
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
    pub health: Option<String>,
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
    pub color: Option<String>,
    pub parent: Option<LabelRef>,
    pub team: Option<Team>,
}

/// Lightweight label reference for parent links.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelRef {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub body: String,
    pub created_at: String,
    pub url: Option<String>,
    pub resolved_at: Option<String>,
    pub user: Option<CommentUser>,
    pub parent: Option<CommentParentRef>,
}

/// Lightweight comment parent reference for threading.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentParentRef {
    pub id: String,
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
    pub description: Option<String>,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
    pub completed_at: Option<String>,
    pub progress: Option<f64>,
    pub issues: Option<NodeList<CycleIssueRef>>,
    pub uncompleted_issues_upon_close: Option<NodeList<CycleIssueRef>>,
}

/// Lightweight issue reference for cycle issue lists.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycleIssueRef {
    pub id: String,
    pub identifier: String,
    pub title: String,
    pub state: Option<IssueStateRef>,
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
    pub health: Option<String>,
    pub url: Option<String>,
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

// ---- #21: Initiatives ----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Initiative {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub target_date: Option<String>,
    pub completed_at: Option<String>,
    pub started_at: Option<String>,
    pub url: Option<String>,
    pub slug_id: Option<String>,
    pub owner: Option<UserRef>,
    pub projects: Option<NodeList<InitiativeProjectRef>>,
}

/// Lightweight project reference for initiatives.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiativeProjectRef {
    pub id: String,
    pub name: String,
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
    pub added_labels: Option<Vec<HistoryLabel>>,
    pub removed_labels: Option<Vec<HistoryLabel>>,
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
#[serde(rename_all = "camelCase")]
pub struct InitiativeDetail {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub target_date: Option<String>,
    pub completed_at: Option<String>,
    pub started_at: Option<String>,
    pub url: Option<String>,
    pub slug_id: Option<String>,
    pub owner: Option<UserRef>,
    pub projects: Option<NodeList<InitiativeProjectRef>>,
}

/// Custom view with issues.
#[derive(Debug, Clone, Deserialize)]
pub struct CustomViewWithIssues {
    pub id: String,
    pub name: String,
    pub issues: Connection<Issue>,
}

/// Custom view mutation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomViewMutationResult {
    pub success: bool,
    pub custom_view: Option<CustomView>,
}

// ---- Agent Sessions & Activities ----

/// Agent session entity.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSession {
    pub id: String,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub url: Option<String>,
    pub plan: Option<serde_json::Value>,
    pub summary: Option<String>,
    pub issue: Option<IssueRef>,
    pub comment: Option<AgentSessionComment>,
    pub activities: Option<NodeList<AgentActivityRef>>,
}

/// Comment reference for agent sessions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSessionComment {
    pub id: String,
    pub body: Option<String>,
}

/// Lightweight agent activity reference.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentActivityRef {
    pub id: String,
    pub created_at: Option<String>,
    pub ephemeral: Option<bool>,
}

/// Agent session mutation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSessionMutationResult {
    pub success: bool,
    pub agent_session: Option<AgentSession>,
}

/// Agent activity mutation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentActivityMutationResult {
    pub success: bool,
    pub agent_activity: Option<AgentActivityRef>,
}

// ---- Customer Management ----

/// Customer entity.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub domains: Option<Vec<String>>,
    pub external_ids: Option<Vec<String>>,
    pub revenue: Option<f64>,
    pub size: Option<f64>,
    pub slug_id: Option<String>,
    pub logo_url: Option<String>,
    pub status: Option<CustomerStatusRef>,
    pub tier: Option<CustomerTierRef>,
    pub owner: Option<UserRef>,
    pub needs: Option<Vec<CustomerNeed>>,
}

/// Customer status reference.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerStatusRef {
    pub display_name: Option<String>,
    pub color: Option<String>,
}

/// Customer tier reference.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerTierRef {
    pub name: Option<String>,
}

/// Customer need (feature request linked to an issue).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerNeed {
    pub id: String,
    pub body: Option<String>,
    pub priority: Option<f64>,
    pub created_at: Option<String>,
    pub customer: Option<CustomerRef>,
    pub issue: Option<IssueRef>,
}

/// Lightweight customer reference.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerRef {
    pub id: String,
    pub name: String,
}

/// Customer mutation result.
#[derive(Debug, Clone, Deserialize)]
pub struct CustomerMutationResult {
    pub success: bool,
    pub customer: Option<Customer>,
}

/// Customer need mutation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerNeedMutationResult {
    pub success: bool,
    pub need: Option<CustomerNeed>,
}

// ---- Initiative Updates ----

/// Initiative status update.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiativeStatusUpdate {
    pub id: String,
    pub body: String,
    pub health: Option<String>,
    pub created_at: Option<String>,
    pub url: Option<String>,
    pub user: Option<ProjectUpdateUser>,
}

/// Initiative update mutation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiativeUpdateMutationResult {
    pub success: bool,
    pub initiative_update: Option<InitiativeStatusUpdate>,
}

// ---- Initiative-to-Project Links ----

/// Initiative-to-project link.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiativeToProject {
    pub id: String,
    pub initiative: Option<InitiativeNameRef>,
    pub project: Option<ProjectNameRefType>,
}

/// Lightweight initiative name reference.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiativeNameRef {
    pub name: String,
}

/// Lightweight project name reference.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectNameRefType {
    pub name: String,
}

/// Initiative-to-project mutation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiativeToProjectMutationResult {
    pub success: bool,
    pub initiative_to_project: Option<InitiativeToProject>,
}

// ---- Project Relations ----

/// Project relation entity.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRelation {
    pub id: String,
    #[serde(rename = "type")]
    pub relation_type: Option<String>,
    pub anchor_type: Option<String>,
    pub related_anchor_type: Option<String>,
    pub project: Option<ProjectNameRefType>,
    pub related_project: Option<ProjectNameRefType>,
}

/// Project relation mutation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRelationMutationResult {
    pub success: bool,
    pub project_relation: Option<ProjectRelation>,
}

// ---- Releases ----

/// Release entity.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub id: String,
    pub name: Option<String>,
    pub version: Option<String>,
    pub url: Option<String>,
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub stage: Option<ReleaseStageRef>,
    pub pipeline: Option<ReleasePipelineRef>,
}

/// Release stage reference.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseStageRef {
    pub name: String,
    pub color: Option<String>,
}

/// Release pipeline reference.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleasePipelineRef {
    pub name: String,
}

/// Release mutation result.
#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseMutationResult {
    pub success: bool,
    pub release: Option<Release>,
}

// ---- Project Search ----

/// Project search result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSearchResult {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub state: Option<String>,
    pub progress: Option<f64>,
    pub url: Option<String>,
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub lead: Option<UserRef>,
    pub teams: Option<NodeList<Team>>,
}

/// Project search connection.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSearchConnection {
    pub nodes: Vec<ProjectSearchResult>,
    pub total_count: Option<i64>,
}
