#![allow(dead_code)]
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

/// Response type for resolve queries (simpler than full list queries).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveStateData {
    pub workflow_states: crate::types::NodeList<ResolvedState>,
}

#[derive(Debug, Deserialize)]
pub struct ResolvedState {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub state_type: String,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCommentData {
    pub comment_update: crate::types::CommentMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCommentData {
    pub comment_delete: crate::types::SuccessResult,
}

// ---- Cycle response data shapes ----

#[derive(Debug, Deserialize)]
pub struct TeamCyclesData {
    pub team: TeamCycles,
}

#[derive(Debug, Deserialize)]
pub struct TeamCycles {
    pub cycles: crate::types::NodeList<crate::types::Cycle>,
}

#[derive(Debug, Deserialize)]
pub struct CycleData {
    pub cycle: crate::types::Cycle,
}

// ---- Label response data shapes ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelsData {
    pub issue_labels: crate::types::NodeList<crate::types::Label>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLabelData {
    pub issue_label_create: crate::types::LabelMutationResult,
}

// ---- Issue relation response data shapes ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIssueRelationData {
    pub issue_relation_create: crate::types::IssueRelationMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteIssueRelationData {
    pub issue_relation_delete: crate::types::SuccessResult,
}

// ---- Archive response data shape ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveIssueData {
    pub issue_archive: crate::types::SuccessResult,
}

// ---- Resolve helpers ----

#[derive(Debug, Deserialize)]
pub struct ResolveProjectData {
    pub projects: crate::types::NodeList<crate::types::Project>,
}

// ---- #17: Documents ----

#[derive(Debug, Deserialize)]
pub struct DocumentsData {
    pub documents: crate::types::NodeList<crate::types::Document>,
}

#[derive(Debug, Deserialize)]
pub struct DocumentData {
    pub document: crate::types::Document,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocumentData {
    pub document_create: DocumentMutationResult,
}

#[derive(Debug, Deserialize)]
pub struct DocumentMutationResult {
    pub success: bool,
    pub document: Option<crate::types::Document>,
}

// ---- #18: Projects ----

#[derive(Debug, Deserialize)]
pub struct ProjectDetailData {
    pub project: crate::types::ProjectDetail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectData {
    pub project_create: crate::types::ProjectMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectData {
    pub project_update: crate::types::ProjectMutationResult,
}

// ---- #19: Project Updates ----

#[derive(Debug, Deserialize)]
pub struct ProjectUpdatesData {
    pub project: ProjectWithUpdates,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectWithUpdates {
    pub project_updates: crate::types::NodeList<crate::types::ProjectUpdate>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectUpdateData {
    pub project_update_create: crate::types::ProjectUpdateMutationResult,
}

// ---- #20: Project Milestones ----

#[derive(Debug, Deserialize)]
pub struct ProjectMilestonesData {
    pub project: ProjectWithMilestones,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectWithMilestones {
    pub project_milestones: crate::types::NodeList<crate::types::ProjectMilestone>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectMilestoneData {
    pub project_milestone_create: crate::types::ProjectMilestoneMutationResult,
}

// ---- #21: Roadmaps and Initiatives ----

#[derive(Debug, Deserialize)]
pub struct RoadmapsData {
    pub roadmaps: crate::types::NodeList<crate::types::Roadmap>,
}

#[derive(Debug, Deserialize)]
pub struct InitiativesData {
    pub initiatives: crate::types::NodeList<crate::types::Initiative>,
}

// ---- #22: Notifications ----

#[derive(Debug, Deserialize)]
pub struct NotificationsData {
    pub notifications: crate::types::NodeList<crate::types::Notification>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkNotificationReadData {
    pub notification_update: crate::types::NotificationMutationResult,
}

// ---- #23: Attachments ----

#[derive(Debug, Deserialize)]
pub struct AttachmentsData {
    pub issue: IssueWithAttachments,
}

#[derive(Debug, Deserialize)]
pub struct IssueWithAttachments {
    pub attachments: crate::types::NodeList<crate::types::Attachment>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAttachmentData {
    pub attachment_create: crate::types::AttachmentMutationResult,
}

// ---- #24: Reactions ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddReactionData {
    pub reaction_create: crate::types::ReactionMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveReactionData {
    pub reaction_delete: crate::types::DeleteMutationResult,
}

// ---- #25: Custom Views ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomViewsData {
    pub custom_views: crate::types::NodeList<crate::types::CustomView>,
}

// ---- #26: Favorites ----

#[derive(Debug, Deserialize)]
pub struct FavoritesData {
    pub favorites: crate::types::NodeList<crate::types::Favorite>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddFavoriteData {
    pub favorite_create: crate::types::FavoriteMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFavoriteData {
    pub favorite_delete: crate::types::DeleteMutationResult,
}

// ---- #29: Templates ----

#[derive(Debug, Deserialize)]
pub struct TemplatesData {
    pub templates: Vec<crate::types::Template>,
}

// ---- #30: Issue History ----

#[derive(Debug, Deserialize)]
pub struct IssueHistoryData {
    pub issue: IssueWithHistory,
}

#[derive(Debug, Deserialize)]
pub struct IssueWithHistory {
    pub history: crate::types::NodeList<crate::types::IssueHistoryEntry>,
}

// ---- #31: Webhooks ----

#[derive(Debug, Deserialize)]
pub struct WebhooksData {
    pub webhooks: crate::types::NodeList<crate::types::Webhook>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookData {
    pub webhook_create: crate::types::WebhookMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteWebhookData {
    pub webhook_delete: crate::types::DeleteMutationResult,
}

// ---- #32: Integrations and Audit Log ----

#[derive(Debug, Deserialize)]
pub struct IntegrationsData {
    pub integrations: crate::types::NodeList<crate::types::Integration>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogData {
    pub audit_entries: crate::types::NodeList<crate::types::AuditEntry>,
}

// ---- #33: Team CRUD ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamData {
    pub team_create: crate::types::TeamMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTeamData {
    pub team_update: crate::types::TeamMutationResult,
}

// ---- Phase 11: Additional tools ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveProjectData {
    pub project_archive: crate::types::DeleteMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocumentData {
    pub document_update: DocumentMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCycleData {
    pub cycle_create: CycleMutationResult,
}

#[derive(Debug, Deserialize)]
pub struct CycleMutationResult {
    pub success: bool,
    pub cycle: Option<crate::types::CycleDetail>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLabelData {
    pub issue_label_update: crate::types::LabelMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteLabelData {
    pub issue_label_delete: crate::types::DeleteMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnarchiveIssueData {
    pub issue_unarchive: crate::types::DeleteMutationResult,
}
