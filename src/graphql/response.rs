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
    pub description: Option<String>,
    pub timezone: Option<String>,
    pub triage_enabled: Option<bool>,
    pub default_issue_state: Option<crate::types::TeamDefaultState>,
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

// ---- Phase 12: Remaining tools ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateIssuesData {
    pub issue_batch_update: crate::types::BatchUpdateResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchDocumentsData {
    pub search_documents: crate::types::DocumentSearchConnection,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInitiativeData {
    pub initiative_create: crate::types::InitiativeMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInitiativeData {
    pub initiative_update: crate::types::InitiativeMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteInitiativeData {
    pub initiative_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewIssuesData {
    pub custom_view: Option<crate::types::CustomViewWithIssues>,
}

// ---- Phase 2: Delete/Archive responses ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDocumentData {
    pub document_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProjectMilestoneData {
    pub project_milestone_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProjectUpdateData {
    pub project_update_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAttachmentData {
    pub attachment_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteIssueData {
    pub issue_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRoadmapData {
    pub roadmap_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteViewData {
    pub custom_view_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveCycleData {
    pub cycle_archive: crate::types::SuccessResult,
}

// ---- Phase 3: Update responses ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCycleData {
    pub cycle_update: CycleMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectMilestoneData {
    pub project_milestone_update: crate::types::ProjectMilestoneMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectUpdateData {
    pub project_update_update: crate::types::ProjectUpdateMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookData {
    pub webhook_update: crate::types::WebhookMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAttachmentData {
    pub attachment_update: crate::types::AttachmentMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoadmapData {
    pub roadmap_update: crate::types::RoadmapMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateViewData {
    pub custom_view_update: crate::types::CustomViewMutationResult,
}

// ---- Phase 4: Comment responses ----

#[derive(Debug, Deserialize)]
pub struct ListCommentsData {
    pub issue: IssueWithComments,
}

#[derive(Debug, Deserialize)]
pub struct IssueWithComments {
    pub comments: crate::types::NodeList<crate::types::Comment>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveCommentData {
    pub comment_resolve: crate::types::CommentMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnresolveCommentData {
    pub comment_unresolve: crate::types::CommentMutationResult,
}

// ---- Phase 5: Subscribe responses ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeToIssueData {
    pub issue_subscribe: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsubscribeFromIssueData {
    pub issue_unsubscribe: crate::types::SuccessResult,
}

// ---- Phase 6: Create responses ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoadmapData {
    pub roadmap_create: crate::types::RoadmapMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateViewData {
    pub custom_view_create: crate::types::CustomViewMutationResult,
}

// ---- Phase 7: Search responses ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchProjectsData {
    pub search_projects: crate::types::ProjectSearchConnection,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueVcsBranchSearchData {
    pub issue_vcs_branch_search: Option<crate::types::Issue>,
}

// ---- Phase 8: Agent Session responses ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSessionCreateOnIssueData {
    pub agent_session_create_on_issue: crate::types::AgentSessionMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSessionCreateOnCommentData {
    pub agent_session_create_on_comment: crate::types::AgentSessionMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAgentSessionData {
    pub agent_session_update: crate::types::AgentSessionMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAgentActivityData {
    pub agent_activity_create: crate::types::AgentActivityMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSessionsData {
    pub agent_sessions: crate::types::Connection<crate::types::AgentSession>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSessionData {
    pub agent_session: crate::types::AgentSession,
}

// ---- Phase 9: Customer responses ----

#[derive(Debug, Deserialize)]
pub struct CustomersData {
    pub customers: crate::types::Connection<crate::types::Customer>,
}

#[derive(Debug, Deserialize)]
pub struct CustomerData {
    pub customer: crate::types::Customer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomerData {
    pub customer_create: crate::types::CustomerMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomerData {
    pub customer_update: crate::types::CustomerMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCustomerData {
    pub customer_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerNeedsData {
    pub customer_needs: crate::types::Connection<crate::types::CustomerNeed>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomerNeedData {
    pub customer_need_create: crate::types::CustomerNeedMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomerNeedData {
    pub customer_need_update: crate::types::CustomerNeedMutationResult,
}

// ---- Phase 10: Initiative Update responses ----

#[derive(Debug, Deserialize)]
pub struct InitiativeUpdatesData {
    pub initiative: InitiativeWithUpdates,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiativeWithUpdates {
    pub initiative_updates: crate::types::NodeList<crate::types::InitiativeStatusUpdate>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInitiativeUpdateData {
    pub initiative_update_create: crate::types::InitiativeUpdateMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddProjectToInitiativeData {
    pub initiative_to_project_create: crate::types::InitiativeToProjectMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveProjectFromInitiativeData {
    pub initiative_to_project_delete: crate::types::SuccessResult,
}

// ---- Phase 11: Project Relation responses ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRelationData {
    pub project_relation_create: crate::types::ProjectRelationMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProjectRelationData {
    pub project_relation_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
pub struct ProjectRelationsData {
    pub project: ProjectWithRelations,
}

#[derive(Debug, Deserialize)]
pub struct ProjectWithRelations {
    pub relations: crate::types::NodeList<crate::types::ProjectRelation>,
}

// ---- Phase 12: Release responses ----

#[derive(Debug, Deserialize)]
pub struct ReleasesData {
    pub releases: crate::types::Connection<crate::types::Release>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReleaseData {
    pub release_create: crate::types::ReleaseMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReleaseData {
    pub release_update: crate::types::ReleaseMutationResult,
}
