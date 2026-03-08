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
pub struct IssueTeamData {
    pub issue: IssueTeamRef,
}

#[derive(Debug, Deserialize)]
pub struct IssueTeamRef {
    pub team: crate::types::TeamRef,
}

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

// ---- #21: Initiatives ----

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

// ========================================================================
// Phase 2 (Complete Coverage): New Response Types
// ========================================================================

// ---- 1A: Workflow State ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStateData {
    pub workflow_state: crate::types::WorkflowState,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkflowStateData {
    pub workflow_state_create: crate::types::WorkflowStateMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkflowStateData {
    pub workflow_state_update: crate::types::WorkflowStateMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveWorkflowStateData {
    pub workflow_state_archive: crate::types::SuccessResult,
}

// ---- 1B: Issue Extras ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueAddLabelData {
    pub issue_add_label: crate::types::MutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRemoveLabelData {
    pub issue_remove_label: crate::types::MutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateIssuesData {
    pub issue_batch_create: crate::types::BatchCreateResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIssueRelationData {
    pub issue_relation_update: crate::types::IssueRelationMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssuePriorityValuesData {
    pub issue_priority_values: Vec<crate::types::IssuePriorityValue>,
}

// ---- 1C: Project Extras ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProjectData {
    pub project_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnarchiveProjectData {
    pub project_unarchive: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectRelationData {
    pub project_relation_update: crate::types::ProjectRelationMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectMilestoneData {
    pub project_milestone: crate::types::ProjectMilestone,
}

// ---- 1D: Team Extras ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTeamData {
    pub team_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnarchiveTeamData {
    pub team_unarchive: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
pub struct GetTeamData {
    pub team: crate::types::Team,
}

// ---- 1E: Document Extras ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnarchiveDocumentData {
    pub document_unarchive: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
pub struct DocumentContentHistoryData {
    pub document: DocumentWithContentHistory,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentWithContentHistory {
    pub content_history: crate::types::NodeList<crate::types::DocumentContentHistoryEntry>,
}

// ---- 1F: Misc ----

#[derive(Debug, Deserialize)]
pub struct GetUserData {
    pub user: crate::types::User,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserData {
    pub user_update: crate::types::UserMutationResult,
}

#[derive(Debug, Deserialize)]
pub struct GetAttachmentData {
    pub attachment: crate::types::Attachment,
}

#[derive(Debug, Deserialize)]
pub struct GetCommentData {
    pub comment: crate::types::Comment,
}

#[derive(Debug, Deserialize)]
pub struct GetFavoriteData {
    pub favorite: crate::types::Favorite,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFavoriteData {
    pub favorite_update: crate::types::FavoriteUpdateMutationResult,
}

#[derive(Debug, Deserialize)]
pub struct GetNotificationData {
    pub notification: crate::types::Notification,
}

// ---- 2A: Customer Status ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerStatusesData {
    pub customer_statuses: crate::types::NodeList<crate::types::CustomerStatusFull>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerStatusData {
    pub customer_status: crate::types::CustomerStatusFull,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomerStatusData {
    pub customer_status_create: crate::types::CustomerStatusMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomerStatusData {
    pub customer_status_update: crate::types::CustomerStatusMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCustomerStatusData {
    pub customer_status_delete: crate::types::SuccessResult,
}

// ---- 2B: Customer Tier ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerTiersData {
    pub customer_tiers: crate::types::NodeList<crate::types::CustomerTierFull>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerTierData {
    pub customer_tier: crate::types::CustomerTierFull,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomerTierData {
    pub customer_tier_create: crate::types::CustomerTierMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomerTierData {
    pub customer_tier_update: crate::types::CustomerTierMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCustomerTierData {
    pub customer_tier_delete: crate::types::SuccessResult,
}

// ---- 2C: Customer Extras ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeCustomersData {
    pub customer_merge: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCustomerNeedData {
    pub customer_need: crate::types::CustomerNeed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveCustomerNeedData {
    pub customer_need_archive: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnarchiveCustomerNeedData {
    pub customer_need_unarchive: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCustomerNeedData {
    pub customer_need_delete: crate::types::SuccessResult,
}

// ---- 2D: Initiative Extras ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveInitiativeData {
    pub initiative_archive: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnarchiveInitiativeData {
    pub initiative_unarchive: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInitiativeToProjectData {
    pub initiative_to_project_update: crate::types::InitiativeToProjectMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveInitiativeUpdateData {
    pub initiative_update_archive: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnarchiveInitiativeUpdateData {
    pub initiative_update_unarchive: crate::types::SuccessResult,
}

// ---- 3A: Release Extras ----

#[derive(Debug, Deserialize)]
pub struct GetReleaseData {
    pub release: crate::types::Release,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveReleaseData {
    pub release_archive: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteReleaseData {
    pub release_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnarchiveReleaseData {
    pub release_unarchive: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchReleasesData {
    pub release_search: crate::types::ReleaseSearchConnection,
}

// ---- 3B: Release Pipeline ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleasePipelinesData {
    pub release_pipelines: crate::types::NodeList<crate::types::ReleasePipelineFull>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleasePipelineData {
    pub release_pipeline: crate::types::ReleasePipelineFull,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReleasePipelineData {
    pub release_pipeline_create: crate::types::ReleasePipelineMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReleasePipelineData {
    pub release_pipeline_update: crate::types::ReleasePipelineMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteReleasePipelineData {
    pub release_pipeline_delete: crate::types::SuccessResult,
}

// ---- 3C: Release Stage ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseStagesData {
    pub release_stages: crate::types::NodeList<crate::types::ReleaseStageFull>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseStageData {
    pub release_stage: crate::types::ReleaseStageFull,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReleaseStageData {
    pub release_stage_create: crate::types::ReleaseStageMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReleaseStageData {
    pub release_stage_update: crate::types::ReleaseStageMutationResult,
}

// ---- 3D: Issue-to-Release ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueToReleasesData {
    pub issue_to_releases: crate::types::NodeList<crate::types::IssueToRelease>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueToReleaseData {
    pub issue_to_release: crate::types::IssueToRelease,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddIssueToReleaseData {
    pub issue_to_release_create: crate::types::IssueToReleaseMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveIssueFromReleaseData {
    pub issue_to_release_delete: crate::types::SuccessResult,
}

// ---- 4A: Project Status ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStatusesData {
    pub project_statuses: crate::types::NodeList<crate::types::ProjectStatusFull>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStatusData {
    pub project_status: crate::types::ProjectStatusFull,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectStatusData {
    pub project_status_create: crate::types::ProjectStatusMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectStatusData {
    pub project_status_update: crate::types::ProjectStatusMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveProjectStatusData {
    pub project_status_archive: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnarchiveProjectStatusData {
    pub project_status_unarchive: crate::types::SuccessResult,
}

// ---- 4B: Project Labels ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectLabelsData {
    pub project_labels: crate::types::NodeList<crate::types::ProjectLabel>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectLabelData {
    pub project_label: crate::types::ProjectLabel,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectLabelData {
    pub project_label_create: crate::types::ProjectLabelMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectLabelData {
    pub project_label_update: crate::types::ProjectLabelMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProjectLabelData {
    pub project_label_delete: crate::types::SuccessResult,
}

// ---- 5A: Team Membership ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamMembershipsData {
    pub team_memberships: crate::types::NodeList<crate::types::TeamMembership>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamMembershipsByTeamData {
    pub team: TeamMembershipsWrapper,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamMembershipsWrapper {
    pub memberships: crate::types::NodeList<crate::types::TeamMembership>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamMembershipData {
    pub team_membership: crate::types::TeamMembership,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamMembershipData {
    pub team_membership_create: crate::types::TeamMembershipMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTeamMembershipData {
    pub team_membership_update: crate::types::TeamMembershipMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTeamMembershipData {
    pub team_membership_delete: crate::types::SuccessResult,
}

// ---- 5B: Notification Subscriptions ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSubscriptionsData {
    pub notification_subscriptions: crate::types::NodeList<crate::types::NotificationSubscription>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSubscriptionData {
    pub notification_subscription: crate::types::NotificationSubscription,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNotificationSubscriptionData {
    pub notification_subscription_create: crate::types::NotificationSubscriptionMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotificationSubscriptionData {
    pub notification_subscription_update: crate::types::NotificationSubscriptionMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationsUnreadCountData {
    pub notifications_unread_count: i32,
}

// ---- 6A: Templates ----

#[derive(Debug, Deserialize)]
pub struct TemplateData {
    pub template: crate::types::Template,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTemplateData {
    pub template_create: crate::types::TemplateMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTemplateData {
    pub template_update: crate::types::TemplateMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTemplateData {
    pub template_delete: crate::types::SuccessResult,
}

// ---- 6B: Entity External Links ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityExternalLinkData {
    pub entity_external_link: crate::types::EntityExternalLink,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEntityExternalLinkData {
    pub entity_external_link_create: crate::types::EntityExternalLinkMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEntityExternalLinkData {
    pub entity_external_link_update: crate::types::EntityExternalLinkMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteEntityExternalLinkData {
    pub entity_external_link_delete: crate::types::SuccessResult,
}

// ---- 6C: Emojis ----

#[derive(Debug, Deserialize)]
pub struct EmojisData {
    pub emojis: crate::types::NodeList<crate::types::Emoji>,
}

#[derive(Debug, Deserialize)]
pub struct EmojiData {
    pub emoji: crate::types::Emoji,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmojiData {
    pub emoji_create: crate::types::EmojiMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteEmojiData {
    pub emoji_delete: crate::types::SuccessResult,
}

// ---- 6D: Initiative Relations ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiativeRelationsData {
    pub initiative_relations: crate::types::NodeList<crate::types::InitiativeRelation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiativeRelationData {
    pub initiative_relation: crate::types::InitiativeRelation,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInitiativeRelationData {
    pub initiative_relation_create: crate::types::InitiativeRelationMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInitiativeRelationData {
    pub initiative_relation_update: crate::types::InitiativeRelationMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteInitiativeRelationData {
    pub initiative_relation_delete: crate::types::SuccessResult,
}

// ---- 7A: Time Schedules ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSchedulesData {
    pub time_schedules: crate::types::NodeList<crate::types::TimeSchedule>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeScheduleData {
    pub time_schedule: crate::types::TimeSchedule,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTimeScheduleData {
    pub time_schedule_create: crate::types::TimeScheduleMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTimeScheduleData {
    pub time_schedule_update: crate::types::TimeScheduleMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTimeScheduleData {
    pub time_schedule_delete: crate::types::SuccessResult,
}

// ---- 7B: Triage Responsibility ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriageResponsibilitiesData {
    pub triage_responsibilities: crate::types::NodeList<crate::types::TriageResponsibility>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriageResponsibilityData {
    pub triage_responsibility: crate::types::TriageResponsibility,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTriageResponsibilityData {
    pub triage_responsibility_create: crate::types::TriageResponsibilityMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTriageResponsibilityData {
    pub triage_responsibility_update: crate::types::TriageResponsibilityMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTriageResponsibilityData {
    pub triage_responsibility_delete: crate::types::SuccessResult,
}

// ---- 7C: Git Automation ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGitAutomationStateData {
    pub git_automation_state_create: crate::types::GitAutomationStateMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGitAutomationStateData {
    pub git_automation_state_update: crate::types::GitAutomationStateMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteGitAutomationStateData {
    pub git_automation_state_delete: crate::types::SuccessResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGitAutomationTargetBranchData {
    pub git_automation_target_branch_create: crate::types::GitAutomationTargetBranchMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGitAutomationTargetBranchData {
    pub git_automation_target_branch_update: crate::types::GitAutomationTargetBranchMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteGitAutomationTargetBranchData {
    pub git_automation_target_branch_delete: crate::types::SuccessResult,
}

// ---- 8A: Email Intake ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailIntakeAddressData {
    pub email_intake_address: crate::types::EmailIntakeAddress,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmailIntakeAddressData {
    pub email_intake_address_create: crate::types::EmailIntakeAddressMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmailIntakeAddressData {
    pub email_intake_address_update: crate::types::EmailIntakeAddressMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteEmailIntakeAddressData {
    pub email_intake_address_delete: crate::types::SuccessResult,
}

// ---- 8B: Misc ----

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivedTeamsData {
    pub archived_teams: crate::types::NodeList<crate::types::TeamDetail>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitStatusData {
    pub rate_limit_status: crate::types::RateLimitStatus,
}

#[derive(Debug, Deserialize)]
pub struct OrganizationData {
    pub organization: crate::types::Organization,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInfoData {
    pub application_info: crate::types::ApplicationInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticSearchData {
    pub semantic_search: crate::types::Connection<crate::types::Issue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachLinkUrlData {
    #[serde(rename = "attachmentLinkURL")]
    pub attachment_link_url: crate::types::AttachmentMutationResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentsForUrlData {
    #[serde(rename = "attachmentsForURL")]
    pub attachments_for_url: crate::types::NodeList<crate::types::Attachment>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueFilterSuggestionData {
    pub issue_filter_suggestion: FilterSuggestionResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectFilterSuggestionData {
    pub project_filter_suggestion: FilterSuggestionResult,
}

#[derive(Debug, Deserialize)]
pub struct FilterSuggestionResult {
    pub filter: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomViewSuggestionData {
    pub custom_view_details_suggestion: CustomViewSuggestionResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomViewSuggestionResult {
    pub name: Option<String>,
    pub description: Option<String>,
    pub filter_data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomViewHasSubscribersData {
    pub custom_view_has_subscribers: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchIssueFigmaFileKeyData {
    pub issue_figma_file_key_search: Option<crate::types::Issue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInitiativeUpdateData {
    pub initiative_update_update: crate::types::InitiativeUpdateMutationResult,
}

#[derive(Debug, Deserialize)]
pub struct ListCommentsAllData {
    pub comments: crate::types::NodeList<crate::types::Comment>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIssueLabelData {
    pub issue_label: crate::types::Label,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIssueRelationData {
    pub issue_relation: crate::types::IssueRelation,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListIssueRelationsData {
    pub issue_relations: crate::types::NodeList<crate::types::IssueRelation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUsersData {
    pub external_users: crate::types::NodeList<crate::types::ExternalUser>,
}
