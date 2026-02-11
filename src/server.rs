use std::collections::BTreeMap;

use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router,
    ErrorData as McpError, ServerHandler,
};

use crate::cache::EntityCache;
use crate::client::LinearClient;
use crate::error::Error;
use crate::format;
use crate::graphql::{filters, queries, response};
use crate::tools::*;
use crate::types;

#[derive(Clone)]
pub struct LinearMcp {
    client: LinearClient,
    issue_id_cache: EntityCache<String>,
    tool_router: ToolRouter<Self>,
}

// ---- Tool registration ----

#[tool_router]
impl LinearMcp {
    pub fn new(client: LinearClient) -> Self {
        Self {
            client,
            issue_id_cache: EntityCache::new(),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        name = "list_issues",
        description = "List Linear issues with flexible filtering. Filter by team, assignee, status, project, label, and priority."
    )]
    async fn list_issues(
        &self,
        Parameters(params): Parameters<list_issues::ListIssuesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_issues(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "search_issues",
        description = "Full-text search across Linear issues. Searches titles, descriptions, and comments."
    )]
    async fn search_issues(
        &self,
        Parameters(params): Parameters<search_issues::SearchIssuesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_search_issues(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_issue",
        description = "Get full details of a single Linear issue by identifier (e.g. 'ENG-123'). Includes description, comments, labels, and relations."
    )]
    async fn get_issue(
        &self,
        Parameters(params): Parameters<get_issue::GetIssueParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_issue(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_teams",
        description = "List all teams in the Linear workspace with their keys and names. Optionally include member counts."
    )]
    async fn list_teams(
        &self,
        Parameters(params): Parameters<list_teams::ListTeamsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_teams(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_projects",
        description = "List all projects in the Linear workspace with status and progress."
    )]
    async fn list_projects(
        &self,
        Parameters(params): Parameters<list_projects::ListProjectsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_projects(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_users",
        description = "List all members of the Linear workspace."
    )]
    async fn list_users(
        &self,
        Parameters(params): Parameters<list_users::ListUsersParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_users(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_states",
        description = "List workflow states (statuses) available in the workspace. Optionally filter by team."
    )]
    async fn list_states(
        &self,
        Parameters(params): Parameters<list_states::ListStatesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_states(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "my_issues",
        description = "Get issues assigned to the authenticated user, grouped by status. A quick overview of your current workload."
    )]
    async fn my_issues(
        &self,
        Parameters(params): Parameters<my_issues::MyIssuesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_my_issues(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_issue",
        description = "Create a new Linear issue. Accepts human-friendly inputs (team key, assignee email, state name) and resolves them automatically."
    )]
    async fn create_issue(
        &self,
        Parameters(params): Parameters<create_issue::CreateIssueParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_issue(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_issue",
        description = "Update an existing Linear issue. Accepts human-friendly inputs (state name, assignee email) and resolves them automatically."
    )]
    async fn update_issue(
        &self,
        Parameters(params): Parameters<update_issue::UpdateIssueParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_issue(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "add_comment",
        description = "Add a comment to a Linear issue. Supports markdown and threaded replies via parentId."
    )]
    async fn add_comment(
        &self,
        Parameters(params): Parameters<add_comment::AddCommentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_add_comment(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_comment",
        description = "Update an existing comment on a Linear issue."
    )]
    async fn update_comment(
        &self,
        Parameters(params): Parameters<update_comment::UpdateCommentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_comment(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_comment",
        description = "Delete a comment from a Linear issue."
    )]
    async fn delete_comment(
        &self,
        Parameters(params): Parameters<delete_comment::DeleteCommentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_comment(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_cycles",
        description = "List cycles for a team. Returns cycle names, dates, and progress."
    )]
    async fn list_cycles(
        &self,
        Parameters(params): Parameters<list_cycles::ListCyclesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_cycles(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_cycle",
        description = "Get full details of a single cycle by UUID."
    )]
    async fn get_cycle(
        &self,
        Parameters(params): Parameters<get_cycle::GetCycleParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_cycle(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "add_issue_to_cycle",
        description = "Add an issue to a cycle by setting its cycleId."
    )]
    async fn add_issue_to_cycle(
        &self,
        Parameters(params): Parameters<add_issue_to_cycle::AddIssueToCycleParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_add_issue_to_cycle(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "remove_issue_from_cycle",
        description = "Remove an issue from its current cycle."
    )]
    async fn remove_issue_from_cycle(
        &self,
        Parameters(params): Parameters<remove_issue_from_cycle::RemoveIssueFromCycleParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_remove_issue_from_cycle(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_labels",
        description = "List issue labels in the workspace. Optionally filter by team."
    )]
    async fn list_labels(
        &self,
        Parameters(params): Parameters<list_labels::ListLabelsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_labels(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_label",
        description = "Create a new issue label. Optionally scope to a team and set a color."
    )]
    async fn create_label(
        &self,
        Parameters(params): Parameters<create_label::CreateLabelParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_label(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_issue_relation",
        description = "Create a relation between two issues (blocks, blocked_by, related, duplicate)."
    )]
    async fn create_issue_relation(
        &self,
        Parameters(params): Parameters<create_issue_relation::CreateIssueRelationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_issue_relation(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_issue_relation",
        description = "Delete an issue relation by its UUID."
    )]
    async fn delete_issue_relation(
        &self,
        Parameters(params): Parameters<delete_issue_relation::DeleteIssueRelationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_issue_relation(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "archive_issue",
        description = "Archive a Linear issue."
    )]
    async fn archive_issue(
        &self,
        Parameters(params): Parameters<archive_issue::ArchiveIssueParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_archive_issue(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Document tools ----

    #[tool(
        name = "list_documents",
        description = "List all documents in the workspace with project and creator info."
    )]
    async fn list_documents(
        &self,
        Parameters(params): Parameters<list_documents::ListDocumentsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_documents(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_document",
        description = "Get full details of a document including its content."
    )]
    async fn get_document(
        &self,
        Parameters(params): Parameters<get_document::GetDocumentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_document(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_document",
        description = "Create a new document. Optionally associate it with a project."
    )]
    async fn create_document(
        &self,
        Parameters(params): Parameters<create_document::CreateDocumentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_document(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Project detail tools ----

    #[tool(
        name = "get_project",
        description = "Get full details of a project including teams, members, lead, and dates."
    )]
    async fn get_project(
        &self,
        Parameters(params): Parameters<get_project::GetProjectParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_project(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_project",
        description = "Create a new project with team associations, lead, and dates."
    )]
    async fn create_project(
        &self,
        Parameters(params): Parameters<create_project::CreateProjectParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_project(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_project",
        description = "Update an existing project's name, description, state, lead, or dates."
    )]
    async fn update_project(
        &self,
        Parameters(params): Parameters<update_project::UpdateProjectParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_project(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Project update tools ----

    #[tool(
        name = "list_project_updates",
        description = "List status updates for a project."
    )]
    async fn list_project_updates(
        &self,
        Parameters(params): Parameters<list_project_updates::ListProjectUpdatesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_project_updates(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_project_update",
        description = "Post a status update to a project with optional health indicator."
    )]
    async fn create_project_update(
        &self,
        Parameters(params): Parameters<create_project_update::CreateProjectUpdateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_project_update(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Project milestone tools ----

    #[tool(
        name = "list_project_milestones",
        description = "List milestones for a project."
    )]
    async fn list_project_milestones(
        &self,
        Parameters(params): Parameters<list_project_milestones::ListProjectMilestonesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_project_milestones(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_project_milestone",
        description = "Create a milestone for a project with name, description, and target date."
    )]
    async fn create_project_milestone(
        &self,
        Parameters(params): Parameters<create_project_milestone::CreateProjectMilestoneParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_project_milestone(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Planning tools ----

    #[tool(
        name = "list_roadmaps",
        description = "List all roadmaps in the workspace."
    )]
    async fn list_roadmaps(
        &self,
        Parameters(params): Parameters<list_roadmaps::ListRoadmapsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_roadmaps(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_initiatives",
        description = "List all initiatives in the workspace."
    )]
    async fn list_initiatives(
        &self,
        Parameters(params): Parameters<list_initiatives::ListInitiativesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_initiatives(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Notification tools ----

    #[tool(
        name = "list_notifications",
        description = "List inbox notifications for the authenticated user."
    )]
    async fn list_notifications(
        &self,
        Parameters(params): Parameters<list_notifications::ListNotificationsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_notifications(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "mark_notification_read",
        description = "Mark a notification as read."
    )]
    async fn mark_notification_read(
        &self,
        Parameters(params): Parameters<mark_notification_read::MarkNotificationReadParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_mark_notification_read(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- View tools ----

    #[tool(
        name = "list_views",
        description = "List custom saved views in the workspace."
    )]
    async fn list_views(
        &self,
        Parameters(params): Parameters<list_views::ListViewsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_views(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Attachment tools ----

    #[tool(
        name = "list_attachments",
        description = "List attachments on an issue."
    )]
    async fn list_attachments(
        &self,
        Parameters(params): Parameters<list_attachments::ListAttachmentsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_attachments(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "add_attachment",
        description = "Add a URL attachment to an issue."
    )]
    async fn add_attachment(
        &self,
        Parameters(params): Parameters<add_attachment::AddAttachmentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_add_attachment(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Reaction tools ----

    #[tool(
        name = "add_reaction",
        description = "Add an emoji reaction to a comment."
    )]
    async fn add_reaction(
        &self,
        Parameters(params): Parameters<add_reaction::AddReactionParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_add_reaction(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "remove_reaction",
        description = "Remove an emoji reaction by its UUID."
    )]
    async fn remove_reaction(
        &self,
        Parameters(params): Parameters<remove_reaction::RemoveReactionParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_remove_reaction(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Favorite tools ----

    #[tool(
        name = "list_favorites",
        description = "List the authenticated user's favorites."
    )]
    async fn list_favorites(
        &self,
        Parameters(params): Parameters<list_favorites::ListFavoritesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_favorites(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "add_favorite",
        description = "Favorite an issue or project."
    )]
    async fn add_favorite(
        &self,
        Parameters(params): Parameters<add_favorite::AddFavoriteParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_add_favorite(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "remove_favorite",
        description = "Remove a favorite by its UUID."
    )]
    async fn remove_favorite(
        &self,
        Parameters(params): Parameters<remove_favorite::RemoveFavoriteParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_remove_favorite(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Template tools ----

    #[tool(
        name = "list_templates",
        description = "List available issue templates in the workspace."
    )]
    async fn list_templates(
        &self,
        Parameters(params): Parameters<list_templates::ListTemplatesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_templates(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- History tools ----

    #[tool(
        name = "get_issue_history",
        description = "Get the audit trail of changes for an issue."
    )]
    async fn get_issue_history(
        &self,
        Parameters(params): Parameters<get_issue_history::GetIssueHistoryParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_issue_history(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Webhook tools ----

    #[tool(
        name = "list_webhooks",
        description = "List active webhooks in the workspace."
    )]
    async fn list_webhooks(
        &self,
        Parameters(params): Parameters<list_webhooks::ListWebhooksParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_webhooks(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_webhook",
        description = "Create a webhook endpoint for receiving Linear events."
    )]
    async fn create_webhook(
        &self,
        Parameters(params): Parameters<create_webhook::CreateWebhookParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_webhook(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_webhook",
        description = "Delete a webhook by its UUID."
    )]
    async fn delete_webhook(
        &self,
        Parameters(params): Parameters<delete_webhook::DeleteWebhookParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_webhook(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Integration tools ----

    #[tool(
        name = "list_integrations",
        description = "List active integrations in the workspace."
    )]
    async fn list_integrations(
        &self,
        Parameters(params): Parameters<list_integrations::ListIntegrationsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_integrations(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Audit log tools ----

    #[tool(
        name = "query_audit_log",
        description = "Query audit log entries for the workspace."
    )]
    async fn query_audit_log(
        &self,
        Parameters(params): Parameters<query_audit_log::QueryAuditLogParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_query_audit_log(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Team management tools ----

    #[tool(
        name = "create_team",
        description = "Create a new team in the workspace."
    )]
    async fn create_team(
        &self,
        Parameters(params): Parameters<create_team::CreateTeamParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_team(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_team",
        description = "Update an existing team's name, description, or timezone."
    )]
    async fn update_team(
        &self,
        Parameters(params): Parameters<update_team::UpdateTeamParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_team(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Additional tools (Phase 11) ----

    #[tool(
        name = "archive_project",
        description = "Archive a project by name or UUID."
    )]
    async fn archive_project(
        &self,
        Parameters(params): Parameters<archive_project::ArchiveProjectParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_archive_project(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_document",
        description = "Update an existing document's title or content."
    )]
    async fn update_document(
        &self,
        Parameters(params): Parameters<update_document::UpdateDocumentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_document(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_cycle",
        description = "Create a new cycle for a team with start and end dates."
    )]
    async fn create_cycle(
        &self,
        Parameters(params): Parameters<create_cycle::CreateCycleParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_cycle(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_label",
        description = "Update a label's name or color."
    )]
    async fn update_label(
        &self,
        Parameters(params): Parameters<update_label::UpdateLabelParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_label(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "archive_label",
        description = "Archive (delete) a label by its UUID."
    )]
    async fn archive_label(
        &self,
        Parameters(params): Parameters<archive_label::ArchiveLabelParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_archive_label(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "unarchive_issue",
        description = "Restore an archived issue."
    )]
    async fn unarchive_issue(
        &self,
        Parameters(params): Parameters<unarchive_issue::UnarchiveIssueParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_unarchive_issue(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }
}

// ---- ServerHandler ----

#[tool_handler]
impl ServerHandler for LinearMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "linear-mcp".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                title: None,
                description: None,
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "Linear MCP server. Interact with Linear issues, teams, projects, and more."
                    .into(),
            ),
        }
    }
}

// ---- Business logic handlers ----

impl LinearMcp {
    // ---- Shared helpers ----

    /// Resolve an issue identifier (e.g. "ENG-123") or UUID to a UUID.
    async fn resolve_issue_id(&self, id_or_identifier: &str) -> Result<String, Error> {
        // UUID pattern
        let is_uuid = id_or_identifier.len() == 36
            && id_or_identifier
                .chars()
                .all(|c| c.is_ascii_hexdigit() || c == '-');
        if is_uuid {
            return Ok(id_or_identifier.to_string());
        }

        let identifier = id_or_identifier.to_string();
        self.issue_id_cache
            .get_or_fetch(&identifier, || {
                let client = self.client.clone();
                let id = identifier.clone();
                async move {
                    let vars = serde_json::json!({ "query": id, "first": 5 });
                    let data: response::SearchIssuesData =
                        client.execute_json(queries::SEARCH_ISSUES, vars).await?;
                    let found = data.search_issues.nodes.iter().find(|n| {
                        n.identifier.eq_ignore_ascii_case(&id)
                    });
                    match found {
                        Some(issue) => Ok(issue.id.clone()),
                        None => Err(Error::NotFound(format!("Issue '{}' not found", id))),
                    }
                }
            })
            .await
    }

    /// Resolve a team key (e.g. "ENG") to a team ID.
    async fn resolve_team_id(&self, team_key: &str) -> Result<String, Error> {
        let filter = filters::TeamFilter {
            key: Some(filters::StringFilter::eq_exact(team_key.to_uppercase())),
        };
        let vars = serde_json::json!({ "filter": filter });
        let data: response::TeamsData = self
            .client
            .execute_json(queries::RESOLVE_TEAM, vars)
            .await?;
        data.teams
            .nodes
            .first()
            .map(|t| t.id.clone())
            .ok_or_else(|| Error::NotFound(format!("Team '{}' not found", team_key)))
    }

    /// Resolve a user email to a user ID.
    async fn resolve_user_id(&self, email: &str) -> Result<String, Error> {
        let filter = filters::UserFilter {
            email: filters::StringFilter::eq_ignore_case(email),
        };
        let vars = serde_json::json!({ "filter": filter });
        let data: response::UsersData = self
            .client
            .execute_json(queries::RESOLVE_USER, vars)
            .await?;
        data.users
            .nodes
            .first()
            .map(|u| u.id.clone())
            .ok_or_else(|| Error::NotFound(format!("User with email '{}' not found", email)))
    }

    /// Resolve a workflow state name + team key to a state ID.
    async fn resolve_state_id(&self, state_name: &str, team_key: &str) -> Result<String, Error> {
        let filter = filters::WorkflowStateFilter {
            name: Some(filters::StringFilter::eq_ignore_case(state_name)),
            team: Some(filters::TeamFilter {
                key: Some(filters::StringFilter::eq_exact(team_key.to_uppercase())),
            }),
        };
        let vars = serde_json::json!({ "filter": filter });
        let data: response::ResolveStateData = self
            .client
            .execute_json(queries::RESOLVE_STATE, vars)
            .await?;
        data.workflow_states
            .nodes
            .first()
            .map(|s| s.id.clone())
            .ok_or_else(|| {
                Error::NotFound(format!(
                    "Workflow state '{}' not found for team '{}'",
                    state_name, team_key
                ))
            })
    }

    /// Get the authenticated viewer.
    async fn get_viewer(&self) -> Result<types::Viewer, Error> {
        let data: response::ViewerData = self
            .client
            .execute::<(), _>(queries::VIEWER, None)
            .await?;
        Ok(data.viewer)
    }

    // ---- list_issues ----

    async fn handle_list_issues(
        &self,
        params: list_issues::ListIssuesParams,
    ) -> Result<String, Error> {
        let mut issue_filters = Vec::new();

        if let Some(ref team) = params.team {
            issue_filters.push(filters::team_filter(team));
        }
        if let Some(ref assignee) = params.assignee {
            issue_filters.push(filters::assignee_filter(assignee));
        }
        if let Some(ref creator) = params.creator {
            issue_filters.push(filters::creator_filter(creator));
        }
        if let Some(ref status) = params.status {
            issue_filters.push(filters::status_filter(status));
        }
        if let Some(ref project) = params.project {
            issue_filters.push(filters::project_filter(project));
        }
        if let Some(ref label) = params.label {
            issue_filters.push(filters::label_filter(label));
        }
        if let Some(ref priority) = params.priority {
            issue_filters.push(filters::priority_filter(priority.to_number()));
        }
        if let Some(estimate) = params.estimate {
            issue_filters.push(filters::estimate_filter(estimate));
        }
        // Note: Linear's IssueFilter only supports `relations: { some: {} }` — it cannot
        // distinguish between blocking vs blocked-by at the filter level. Both flags filter
        // to issues that have any relations.
        if params.has_blocked_by_relation == Some(true) || params.has_blocking_relation == Some(true) {
            issue_filters.push(filters::has_relation_filter());
        }

        // Date range filters (#8)
        if params.due_before.is_some() || params.due_after.is_some() {
            issue_filters.push(filters::due_date_filter(
                params.due_before.as_deref(),
                params.due_after.as_deref(),
            ));
        }
        if params.created_before.is_some() || params.created_after.is_some() {
            issue_filters.push(filters::created_at_filter(
                params.created_before.as_deref(),
                params.created_after.as_deref(),
            ));
        }
        if params.updated_before.is_some() || params.updated_after.is_some() {
            issue_filters.push(filters::updated_at_filter(
                params.updated_before.as_deref(),
                params.updated_after.as_deref(),
            ));
        }

        let filter = filters::IssueFilter::combine(issue_filters);
        let limit = params.limit.unwrap_or(25).min(100);
        let order_by = params.order_by.as_ref().map(|o| o.as_str()).unwrap_or("updatedAt");

        let mut vars = serde_json::json!({
            "first": limit,
            "orderBy": order_by,
        });
        if let Some(ref cursor) = params.cursor {
            vars["after"] = serde_json::Value::String(cursor.clone());
        }
        if let Some(f) = filter {
            vars["filter"] = serde_json::to_value(f).unwrap();
        }

        let data: response::IssuesData = self
            .client
            .execute_json(queries::LIST_ISSUES, vars)
            .await?;

        let issues = &data.issues.nodes;
        if issues.is_empty() {
            return Ok("No issues found matching the filters.".to_string());
        }

        let lines: Vec<String> = issues.iter().map(format::format_issue_summary).collect();
        let pagination = format::format_pagination_with_cursor(
            data.issues.page_info.has_next_page,
            issues.len(),
            data.issues.page_info.end_cursor.as_deref(),
        );

        Ok(format!("{}{}", lines.join("\n"), pagination))
    }

    // ---- search_issues ----

    async fn handle_search_issues(
        &self,
        params: search_issues::SearchIssuesParams,
    ) -> Result<String, Error> {
        let mut issue_filters = Vec::new();

        if let Some(ref team) = params.team {
            issue_filters.push(filters::team_filter(team));
        }
        if let Some(ref status) = params.status {
            issue_filters.push(filters::status_filter(status));
        }
        if let Some(ref assignee) = params.assignee {
            issue_filters.push(filters::assignee_filter(assignee));
        }

        let filter = filters::IssueFilter::combine(issue_filters);
        let limit = params.limit.unwrap_or(25).min(100);

        let mut vars = serde_json::json!({
            "query": params.query,
            "first": limit,
        });
        if let Some(ref cursor) = params.cursor {
            vars["after"] = serde_json::Value::String(cursor.clone());
        }
        if let Some(f) = filter {
            vars["filter"] = serde_json::to_value(f).unwrap();
        }

        let data: response::SearchIssuesData = self
            .client
            .execute_json(queries::SEARCH_ISSUES, vars)
            .await?;

        let issues = &data.search_issues.nodes;
        if issues.is_empty() {
            return Ok(format!(
                "No issues found matching \"{}\".",
                params.query
            ));
        }

        let lines: Vec<String> = issues.iter().map(format::format_issue_summary).collect();
        let pagination = format::format_pagination_with_cursor(
            data.search_issues.page_info.has_next_page,
            issues.len(),
            data.search_issues.page_info.end_cursor.as_deref(),
        );

        Ok(format!(
            "Search results for \"{}\":\n\n{}{}",
            params.query,
            lines.join("\n"),
            pagination
        ))
    }

    // ---- get_issue ----

    async fn handle_get_issue(&self, params: get_issue::GetIssueParams) -> Result<String, Error> {
        let uuid = self.resolve_issue_id(&params.id).await?;
        let vars = serde_json::json!({ "id": uuid });
        let data: response::IssueData = self
            .client
            .execute_json(queries::GET_ISSUE, vars)
            .await?;
        Ok(format::format_issue_detail(&data.issue))
    }

    // ---- list_teams ----

    async fn handle_list_teams(
        &self,
        params: list_teams::ListTeamsParams,
    ) -> Result<String, Error> {
        let include_members = params.include_member_count.unwrap_or(false);

        if include_members {
            let data: response::TeamsWithMembersData = self
                .client
                .execute::<(), _>(queries::LIST_TEAMS_WITH_MEMBERS, None)
                .await?;

            let lines: Vec<String> = data
                .teams
                .nodes
                .iter()
                .map(|t| {
                    let count = t.members.as_ref().map(|m| m.nodes.len());
                    let team = types::Team {
                        id: t.id.clone(),
                        key: t.key.clone(),
                        name: t.name.clone(),
                    };
                    format::format_team(&team, count)
                })
                .collect();

            Ok(format!("Teams:\n\n{}", lines.join("\n")))
        } else {
            let data: response::TeamsData = self
                .client
                .execute::<(), _>(queries::LIST_TEAMS, None)
                .await?;

            let lines: Vec<String> = data
                .teams
                .nodes
                .iter()
                .map(|t| format::format_team(t, None))
                .collect();

            Ok(format!("Teams:\n\n{}", lines.join("\n")))
        }
    }

    // ---- list_projects ----

    async fn handle_list_projects(
        &self,
        params: list_projects::ListProjectsParams,
    ) -> Result<String, Error> {
        let desired_limit = params.limit.unwrap_or(50).min(100) as usize;
        // When team filtering is active, fetch more projects server-side since
        // Linear's ProjectFilter doesn't support team key filtering.
        let fetch_limit = if params.team.is_some() { 250 } else { desired_limit };

        let mut project_filters = Vec::new();

        if let Some(ref status) = params.status {
            project_filters.push(filters::ProjectFilter {
                state: Some(filters::StringFilter::eq_exact(status.as_str())),
                ..Default::default()
            });
        }
        if let Some(ref lead) = params.lead {
            project_filters.push(filters::ProjectFilter {
                lead: Some(filters::ProjectLeadFilter {
                    or: Some(vec![
                        filters::ProjectLeadFieldFilter {
                            email: Some(filters::StringFilter::eq_ignore_case(lead)),
                            display_name: None,
                        },
                        filters::ProjectLeadFieldFilter {
                            email: None,
                            display_name: Some(filters::StringFilter::eq_ignore_case(lead)),
                        },
                    ]),
                }),
                ..Default::default()
            });
        }

        let filter = filters::ProjectFilter::combine(project_filters);

        let mut vars = serde_json::json!({ "first": fetch_limit });
        if let Some(f) = filter {
            vars["filter"] = serde_json::to_value(f).unwrap();
        }

        let data: response::ProjectsData = self
            .client
            .execute_json(queries::LIST_PROJECTS, vars)
            .await?;

        // If team filter is specified, filter client-side (Linear's ProjectFilter
        // doesn't have a direct team key filter) and cap at desired limit.
        let projects: Vec<&types::Project> = if let Some(ref team_key) = params.team {
            let key_upper = team_key.to_uppercase();
            data.projects
                .nodes
                .iter()
                .filter(|p| {
                    p.teams.as_ref().map_or(false, |teams| {
                        teams.nodes.iter().any(|t| t.key.eq_ignore_ascii_case(&key_upper))
                    })
                })
                .take(desired_limit)
                .collect()
        } else {
            data.projects.nodes.iter().collect()
        };

        if projects.is_empty() {
            return Ok("No projects found matching the filters.".to_string());
        }

        let lines: Vec<String> = projects
            .iter()
            .map(|p| format::format_project(p))
            .collect();

        Ok(format!("Projects:\n\n{}", lines.join("\n\n")))
    }

    // ---- list_users ----

    async fn handle_list_users(
        &self,
        params: list_users::ListUsersParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).min(100);
        let vars = serde_json::json!({ "first": limit });

        let data: response::UsersData = self
            .client
            .execute_json(queries::LIST_USERS, vars)
            .await?;

        let lines: Vec<String> = data
            .users
            .nodes
            .iter()
            .map(format::format_user)
            .collect();

        Ok(format!("Members:\n\n{}", lines.join("\n")))
    }

    // ---- list_states ----

    async fn handle_list_states(
        &self,
        params: list_states::ListStatesParams,
    ) -> Result<String, Error> {
        let mut vars = serde_json::json!({ "first": 200 });
        if let Some(ref team) = params.team {
            let filter = filters::WorkflowStateFilter {
                name: None,
                team: Some(filters::TeamFilter {
                    key: Some(filters::StringFilter::eq_ignore_case(team)),
                }),
            };
            vars["filter"] = serde_json::to_value(filter).unwrap();
        }

        let data: response::WorkflowStatesData = self
            .client
            .execute_json(queries::LIST_STATES, vars)
            .await?;

        // Group states by team
        let mut groups: BTreeMap<String, Vec<&response::WorkflowStateWithTeam>> = BTreeMap::new();
        for state in &data.workflow_states.nodes {
            groups
                .entry(state.team.key.clone())
                .or_default()
                .push(state);
        }

        let mut sections = Vec::new();
        for (team_key, states) in &groups {
            let lines: Vec<String> = states.iter().map(|s| format::format_workflow_state(s)).collect();
            sections.push(format!("## {}\n{}", team_key, lines.join("\n")));
        }

        Ok(format!("Workflow States:\n\n{}", sections.join("\n\n")))
    }

    // ---- my_issues ----

    async fn handle_my_issues(
        &self,
        params: my_issues::MyIssuesParams,
    ) -> Result<String, Error> {
        let viewer = self.get_viewer().await?;
        let limit = params.limit.unwrap_or(50).min(100);
        let include_completed = params.include_completed.unwrap_or(false);

        let mut issue_filters = vec![filters::viewer_filter(&viewer.id)];
        if !include_completed {
            issue_filters.push(filters::exclude_completed_filter());
        }
        if let Some(ref team) = params.team {
            issue_filters.push(filters::team_filter(team));
        }
        if let Some(ref priority) = params.priority {
            issue_filters.push(filters::priority_filter(priority.to_number()));
        }

        let filter = filters::IssueFilter::combine(issue_filters);

        let mut vars = serde_json::json!({ "first": limit });
        if let Some(ref cursor) = params.cursor {
            vars["after"] = serde_json::Value::String(cursor.clone());
        }
        if let Some(f) = filter {
            vars["filter"] = serde_json::to_value(f).unwrap();
        }

        let data: response::IssuesData = self
            .client
            .execute_json(queries::MY_ISSUES, vars)
            .await?;

        let issues = &data.issues.nodes;
        if issues.is_empty() {
            return Ok(format!("No issues assigned to {}.", viewer.display_name));
        }

        // Group by status
        let mut groups: Vec<(String, Vec<&types::Issue>)> = Vec::new();
        for issue in issues {
            let state_name = issue
                .state
                .as_ref()
                .map(|s| s.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());

            if let Some(group) = groups.iter_mut().find(|(name, _)| *name == state_name) {
                group.1.push(issue);
            } else {
                groups.push((state_name, vec![issue]));
            }
        }

        let sections: Vec<String> = groups
            .iter()
            .map(|(state_name, issues)| {
                let lines: Vec<String> =
                    issues.iter().map(|i| format::format_issue_summary(i)).collect();
                format!("## {} ({})\n{}", state_name, issues.len(), lines.join("\n"))
            })
            .collect();

        let pagination = format::format_pagination_with_cursor(
            data.issues.page_info.has_next_page,
            issues.len(),
            data.issues.page_info.end_cursor.as_deref(),
        );

        Ok(format!(
            "Issues assigned to {}:\n\n{}{}",
            viewer.display_name,
            sections.join("\n\n"),
            pagination
        ))
    }

    // ---- create_issue ----

    async fn handle_create_issue(
        &self,
        params: create_issue::CreateIssueParams,
    ) -> Result<String, Error> {
        let team_id = self.resolve_team_id(&params.team).await?;

        let mut input = serde_json::json!({
            "teamId": team_id,
            "title": params.title,
        });

        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(ref email) = params.assignee {
            let user_id = self.resolve_user_id(email).await?;
            input["assigneeId"] = serde_json::Value::String(user_id);
        }
        if let Some(ref status) = params.status {
            let state_id = self.resolve_state_id(status, &params.team).await?;
            input["stateId"] = serde_json::Value::String(state_id);
        }
        if let Some(ref priority) = params.priority {
            input["priority"] = serde_json::Value::Number(priority.to_number().into());
        }
        if let Some(estimate) = params.estimate {
            input["estimate"] = serde_json::json!(estimate);
        }
        if let Some(ref due_date) = params.due_date {
            input["dueDate"] = serde_json::Value::String(due_date.clone());
        }

        // Task #6: labels, project, parent
        if let Some(ref label_names) = params.labels {
            let label_ids = self.resolve_label_ids(label_names).await?;
            input["labelIds"] = serde_json::json!(label_ids);
        }
        if let Some(ref project_name) = params.project {
            let project_id = self.resolve_project_id(project_name).await?;
            input["projectId"] = serde_json::Value::String(project_id);
        }
        if let Some(ref parent_identifier) = params.parent {
            let parent_id = self.resolve_issue_id(parent_identifier).await?;
            input["parentId"] = serde_json::Value::String(parent_id);
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateIssueData = self
            .client
            .execute_json(queries::CREATE_ISSUE, vars)
            .await?;

        match data.issue_create.issue {
            Some(issue) => {
                let detail = format::format_issue_detail(&issue);
                Ok(format!("Issue created:\n\n{}", detail))
            }
            None => Err(Error::GraphQL("Issue creation returned no issue".into())),
        }
    }

    // ---- update_issue ----

    async fn handle_update_issue(
        &self,
        params: update_issue::UpdateIssueParams,
    ) -> Result<String, Error> {
        let uuid = self.resolve_issue_id(&params.id).await?;

        let mut input = serde_json::Map::new();
        let mut has_fields = false;

        if let Some(ref title) = params.title {
            input.insert("title".into(), serde_json::Value::String(title.clone()));
            has_fields = true;
        }
        if let Some(ref desc) = params.description {
            input.insert(
                "description".into(),
                serde_json::Value::String(desc.clone()),
            );
            has_fields = true;
        }
        if let Some(ref priority) = params.priority {
            input.insert(
                "priority".into(),
                serde_json::Value::Number(priority.to_number().into()),
            );
            has_fields = true;
        }
        if let Some(estimate) = params.estimate {
            input.insert("estimate".into(), serde_json::json!(estimate));
            has_fields = true;
        }

        // Assignee: "none" clears it
        if let Some(ref assignee) = params.assignee {
            if assignee.eq_ignore_ascii_case("none") {
                input.insert("assigneeId".into(), serde_json::Value::Null);
            } else {
                let user_id = self.resolve_user_id(assignee).await?;
                input.insert(
                    "assigneeId".into(),
                    serde_json::Value::String(user_id),
                );
            }
            has_fields = true;
        }

        // Due date: "none" clears it
        if let Some(ref due_date) = params.due_date {
            if due_date.eq_ignore_ascii_case("none") {
                input.insert("dueDate".into(), serde_json::Value::Null);
            } else {
                input.insert(
                    "dueDate".into(),
                    serde_json::Value::String(due_date.clone()),
                );
            }
            has_fields = true;
        }

        // Status: need to resolve state ID — requires knowing the team
        if let Some(ref status) = params.status {
            // Fetch the issue to get its team key
            let issue_vars = serde_json::json!({ "id": uuid });
            let issue_data: response::IssueData = self
                .client
                .execute_json(queries::GET_ISSUE, issue_vars)
                .await?;
            let team_key = issue_data
                .issue
                .team
                .as_ref()
                .map(|t| t.key.clone())
                .ok_or_else(|| Error::InvalidInput("Issue has no team".into()))?;

            let state_id = self.resolve_state_id(status, &team_key).await?;
            input.insert("stateId".into(), serde_json::Value::String(state_id));
            has_fields = true;
        }

        // Task #6: labels, project, parent on update
        if let Some(ref label_names) = params.labels {
            let label_ids = self.resolve_label_ids(label_names).await?;
            input.insert("labelIds".into(), serde_json::json!(label_ids));
            has_fields = true;
        }
        if let Some(ref project_name) = params.project {
            if project_name.eq_ignore_ascii_case("none") {
                input.insert("projectId".into(), serde_json::Value::Null);
            } else {
                let project_id = self.resolve_project_id(project_name).await?;
                input.insert("projectId".into(), serde_json::Value::String(project_id));
            }
            has_fields = true;
        }
        if let Some(ref parent_identifier) = params.parent {
            if parent_identifier.eq_ignore_ascii_case("none") {
                input.insert("parentId".into(), serde_json::Value::Null);
            } else {
                let parent_id = self.resolve_issue_id(parent_identifier).await?;
                input.insert("parentId".into(), serde_json::Value::String(parent_id));
            }
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput(
                "No fields to update. Provide at least one field.".into(),
            ));
        }

        let vars = serde_json::json!({
            "id": uuid,
            "input": serde_json::Value::Object(input),
        });
        let data: response::UpdateIssueData = self
            .client
            .execute_json(queries::UPDATE_ISSUE, vars)
            .await?;

        match data.issue_update.issue {
            Some(issue) => {
                let detail = format::format_issue_detail(&issue);
                Ok(format!("Issue updated:\n\n{}", detail))
            }
            None => Err(Error::GraphQL("Issue update returned no issue".into())),
        }
    }

    // ---- add_comment (with threaded reply support, Task #9) ----

    async fn handle_add_comment(
        &self,
        params: add_comment::AddCommentParams,
    ) -> Result<String, Error> {
        let uuid = self.resolve_issue_id(&params.issue_id).await?;

        // Fetch identifier for the confirmation message
        let issue_vars = serde_json::json!({ "id": uuid });
        let issue_data: response::IssueData = self
            .client
            .execute_json(queries::GET_ISSUE, issue_vars)
            .await?;
        let identifier = &issue_data.issue.identifier;

        let mut comment_input = serde_json::json!({
            "issueId": uuid,
            "body": params.body,
        });
        if let Some(ref parent_id) = params.parent_id {
            comment_input["parentId"] = serde_json::Value::String(parent_id.clone());
        }

        let vars = serde_json::json!({ "input": comment_input });
        let data: response::AddCommentData = self
            .client
            .execute_json(queries::ADD_COMMENT, vars)
            .await?;

        let reply_note = if params.parent_id.is_some() { " (reply)" } else { "" };

        match data.comment_create.comment {
            Some(comment) => {
                let formatted = format::format_comment(&comment);
                Ok(format!("Comment{} added to {}:\n\n{}", reply_note, identifier, formatted))
            }
            None => Ok(format!(
                "Comment{} added to {} but could not fetch details.",
                reply_note, identifier
            )),
        }
    }

    // ---- update_comment (Task #10) ----

    async fn handle_update_comment(
        &self,
        params: update_comment::UpdateCommentParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({
            "id": params.id,
            "input": {
                "body": params.body,
            }
        });
        let data: response::UpdateCommentData = self
            .client
            .execute_json(queries::UPDATE_COMMENT, vars)
            .await?;

        match data.comment_update.comment {
            Some(comment) => {
                let formatted = format::format_comment(&comment);
                Ok(format!("Comment updated:\n\n{}", formatted))
            }
            None => Ok("Comment updated but could not fetch details.".to_string()),
        }
    }

    // ---- delete_comment (Task #10) ----

    async fn handle_delete_comment(
        &self,
        params: delete_comment::DeleteCommentParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteCommentData = self
            .client
            .execute_json(queries::DELETE_COMMENT, vars)
            .await?;

        if data.comment_delete.success {
            Ok(format!("Comment {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Comment deletion failed".into()))
        }
    }

    // ---- list_cycles (Task #4) ----

    async fn handle_list_cycles(
        &self,
        params: list_cycles::ListCyclesParams,
    ) -> Result<String, Error> {
        let team_id = self.resolve_team_id(&params.team).await?;
        let limit = params.limit.unwrap_or(25).min(100);

        let vars = serde_json::json!({
            "teamId": team_id,
            "first": limit,
        });
        let data: response::TeamCyclesData = self
            .client
            .execute_json(queries::LIST_CYCLES, vars)
            .await?;

        let cycles = &data.team.cycles.nodes;
        if cycles.is_empty() {
            return Ok(format!("No cycles found for team {}.", params.team));
        }

        let lines: Vec<String> = cycles.iter().map(format::format_cycle_summary).collect();
        Ok(format!("Cycles for {}:\n\n{}", params.team, lines.join("\n")))
    }

    // ---- get_cycle (Task #4) ----

    async fn handle_get_cycle(
        &self,
        params: get_cycle::GetCycleParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::CycleData = self
            .client
            .execute_json(queries::GET_CYCLE, vars)
            .await?;
        Ok(format::format_cycle_detail(&data.cycle))
    }

    // ---- add_issue_to_cycle (Task #4) ----

    async fn handle_add_issue_to_cycle(
        &self,
        params: add_issue_to_cycle::AddIssueToCycleParams,
    ) -> Result<String, Error> {
        let issue_uuid = self.resolve_issue_id(&params.issue_id).await?;

        let vars = serde_json::json!({
            "id": issue_uuid,
            "input": {
                "cycleId": params.cycle_id,
            }
        });
        let data: response::UpdateIssueData = self
            .client
            .execute_json(queries::UPDATE_ISSUE, vars)
            .await?;

        match data.issue_update.issue {
            Some(issue) => Ok(format!(
                "Issue {} added to cycle {}.",
                issue.identifier, params.cycle_id
            )),
            None => Err(Error::GraphQL("Failed to add issue to cycle".into())),
        }
    }

    // ---- remove_issue_from_cycle (Task #4) ----

    async fn handle_remove_issue_from_cycle(
        &self,
        params: remove_issue_from_cycle::RemoveIssueFromCycleParams,
    ) -> Result<String, Error> {
        let issue_uuid = self.resolve_issue_id(&params.issue_id).await?;

        let vars = serde_json::json!({
            "id": issue_uuid,
            "input": {
                "cycleId": serde_json::Value::Null,
            }
        });
        let data: response::UpdateIssueData = self
            .client
            .execute_json(queries::UPDATE_ISSUE, vars)
            .await?;

        match data.issue_update.issue {
            Some(issue) => Ok(format!(
                "Issue {} removed from cycle.",
                issue.identifier
            )),
            None => Err(Error::GraphQL("Failed to remove issue from cycle".into())),
        }
    }

    // ---- list_labels (Task #5) ----

    async fn handle_list_labels(
        &self,
        params: list_labels::ListLabelsParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).min(100);

        let mut vars = serde_json::json!({ "first": limit });
        if let Some(ref team) = params.team {
            let filter = filters::IssueLabelFilter {
                name: None,
                team: Some(filters::TeamFilter {
                    key: Some(filters::StringFilter::eq_ignore_case(team)),
                }),
                or: None,
            };
            vars["filter"] = serde_json::to_value(filter).unwrap();
        }

        let data: response::LabelsData = self
            .client
            .execute_json(queries::LIST_LABELS, vars)
            .await?;

        let labels = &data.issue_labels.nodes;
        if labels.is_empty() {
            return Ok("No labels found.".to_string());
        }

        let lines: Vec<String> = labels.iter().map(format::format_label).collect();
        Ok(format!("Labels:\n\n{}", lines.join("\n")))
    }

    // ---- create_label (Task #5) ----

    async fn handle_create_label(
        &self,
        params: create_label::CreateLabelParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({
            "name": params.name,
        });

        if let Some(ref team_key) = params.team {
            let team_id = self.resolve_team_id(team_key).await?;
            input["teamId"] = serde_json::Value::String(team_id);
        }
        if let Some(ref color) = params.color {
            input["color"] = serde_json::Value::String(color.clone());
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateLabelData = self
            .client
            .execute_json(queries::CREATE_LABEL, vars)
            .await?;

        match data.issue_label_create.issue_label {
            Some(label) => Ok(format!("Label created: {} [id: {}]", label.name, label.id)),
            None => Err(Error::GraphQL("Label creation returned no label".into())),
        }
    }

    // ---- create_issue_relation (Task #7) ----

    async fn handle_create_issue_relation(
        &self,
        params: create_issue_relation::CreateIssueRelationParams,
    ) -> Result<String, Error> {
        let issue_uuid = self.resolve_issue_id(&params.issue_id).await?;
        let related_uuid = self.resolve_issue_id(&params.related_issue_id).await?;

        let vars = serde_json::json!({
            "input": {
                "issueId": issue_uuid,
                "relatedIssueId": related_uuid,
                "type": params.relation_type.as_str(),
            }
        });
        let data: response::CreateIssueRelationData = self
            .client
            .execute_json(queries::CREATE_ISSUE_RELATION, vars)
            .await?;

        match data.issue_relation_create.issue_relation {
            Some(relation) => {
                let formatted = format::format_issue_relation(&relation);
                Ok(format!("Relation created:\n\n{}", formatted))
            }
            None => Err(Error::GraphQL("Issue relation creation returned no relation".into())),
        }
    }

    // ---- delete_issue_relation (Task #7) ----

    async fn handle_delete_issue_relation(
        &self,
        params: delete_issue_relation::DeleteIssueRelationParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteIssueRelationData = self
            .client
            .execute_json(queries::DELETE_ISSUE_RELATION, vars)
            .await?;

        if data.issue_relation_delete.success {
            Ok(format!("Issue relation {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Issue relation deletion failed".into()))
        }
    }

    // ---- archive_issue (Task #11) ----

    async fn handle_archive_issue(
        &self,
        params: archive_issue::ArchiveIssueParams,
    ) -> Result<String, Error> {
        let uuid = self.resolve_issue_id(&params.id).await?;

        // Fetch identifier for the confirmation message
        let issue_vars = serde_json::json!({ "id": uuid });
        let issue_data: response::IssueData = self
            .client
            .execute_json(queries::GET_ISSUE, issue_vars)
            .await?;
        let identifier = &issue_data.issue.identifier;

        let vars = serde_json::json!({ "id": uuid });
        let data: response::ArchiveIssueData = self
            .client
            .execute_json(queries::ARCHIVE_ISSUE, vars)
            .await?;

        if data.issue_archive.success {
            Ok(format!("Issue {} archived.", identifier))
        } else {
            Err(Error::GraphQL("Issue archive failed".into()))
        }
    }

    // ---- resolve helpers for Task #6 ----

    /// Resolve comma-separated label names to a list of label UUIDs.
    async fn resolve_label_ids(&self, label_names: &str) -> Result<Vec<String>, Error> {
        let names: Vec<&str> = label_names.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        if names.is_empty() {
            return Ok(Vec::new());
        }

        // Build an OR filter to match any of the label names
        let or_filters: Vec<filters::IssueLabelFilter> = names
            .iter()
            .map(|name| filters::IssueLabelFilter {
                name: Some(filters::StringFilter::eq_ignore_case(*name)),
                team: None,
                or: None,
            })
            .collect();

        let filter = filters::IssueLabelFilter {
            name: None,
            team: None,
            or: Some(or_filters),
        };

        let vars = serde_json::json!({ "filter": filter });
        let data: response::LabelsData = self
            .client
            .execute_json(queries::RESOLVE_LABELS, vars)
            .await?;

        // Check all names were found
        let mut ids = Vec::new();
        for name in &names {
            let found = data.issue_labels.nodes.iter().find(|l| {
                l.name.eq_ignore_ascii_case(name)
            });
            match found {
                Some(label) => ids.push(label.id.clone()),
                None => return Err(Error::NotFound(format!("Label '{}' not found", name))),
            }
        }

        Ok(ids)
    }

    /// Resolve a project name to a project UUID.
    /// Prefers exact case-insensitive match; returns ambiguity error if multiple partial matches.
    async fn resolve_project_id(&self, project_name: &str) -> Result<String, Error> {
        let filter = filters::ProjectNameResolveFilter {
            name: filters::StringFilter::contains_ignore_case(project_name),
        };
        let vars = serde_json::json!({ "filter": filter });
        let data: response::ResolveProjectData = self
            .client
            .execute_json(queries::RESOLVE_PROJECT, vars)
            .await?;

        let matches = &data.projects.nodes;
        if matches.is_empty() {
            return Err(Error::NotFound(format!("Project '{}' not found", project_name)));
        }

        // Prefer exact case-insensitive match
        if let Some(exact) = matches.iter().find(|p| p.name.eq_ignore_ascii_case(project_name)) {
            return Ok(exact.id.clone());
        }

        // Single partial match is fine
        if matches.len() == 1 {
            return Ok(matches[0].id.clone());
        }

        // Multiple partial matches — ambiguous
        let names: Vec<&str> = matches.iter().map(|p| p.name.as_str()).collect();
        Err(Error::InvalidInput(format!(
            "Ambiguous project name '{}'. Matches: {}",
            project_name,
            names.join(", ")
        )))
    }

    /// Resolve a project name or UUID to a UUID.
    async fn resolve_project_id_or_uuid(&self, id_or_name: &str) -> Result<String, Error> {
        let is_uuid = id_or_name.len() == 36
            && id_or_name
                .chars()
                .all(|c| c.is_ascii_hexdigit() || c == '-');
        if is_uuid {
            return Ok(id_or_name.to_string());
        }
        self.resolve_project_id(id_or_name).await
    }

    // ---- Document handlers ----

    async fn handle_list_documents(
        &self,
        params: list_documents::ListDocumentsParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::DocumentsData = self
            .client
            .execute_json(queries::LIST_DOCUMENTS, vars)
            .await?;

        let docs = &data.documents.nodes;
        if docs.is_empty() {
            return Ok("No documents found.".to_string());
        }

        let lines: Vec<String> = docs.iter().map(format::format_document_summary).collect();
        Ok(format!("Documents:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_get_document(
        &self,
        params: get_document::GetDocumentParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DocumentData = self
            .client
            .execute_json(queries::GET_DOCUMENT, vars)
            .await?;
        Ok(format::format_document_detail(&data.document))
    }

    async fn handle_create_document(
        &self,
        params: create_document::CreateDocumentParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({
            "title": params.title,
        });

        if let Some(ref content) = params.content {
            input["content"] = serde_json::Value::String(content.clone());
        }
        if let Some(ref project_name) = params.project {
            let project_id = self.resolve_project_id(project_name).await?;
            input["projectId"] = serde_json::Value::String(project_id);
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateDocumentData = self
            .client
            .execute_json(queries::CREATE_DOCUMENT, vars)
            .await?;

        match data.document_create.document {
            Some(doc) => Ok(format!(
                "Document created:\n\n{}",
                format::format_document_detail(&doc)
            )),
            None => Err(Error::GraphQL("Document creation returned no document".into())),
        }
    }

    // ---- Project detail handlers ----

    async fn handle_get_project(
        &self,
        params: get_project::GetProjectParams,
    ) -> Result<String, Error> {
        let project_id = self.resolve_project_id_or_uuid(&params.id).await?;
        let vars = serde_json::json!({ "id": project_id });
        let data: response::ProjectDetailData = self
            .client
            .execute_json(queries::GET_PROJECT, vars)
            .await?;
        Ok(format::format_project_detail(&data.project))
    }

    async fn handle_create_project(
        &self,
        params: create_project::CreateProjectParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({
            "name": params.name,
        });

        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(ref teams) = params.teams {
            let team_keys: Vec<&str> = teams.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            let mut team_ids = Vec::new();
            for key in team_keys {
                team_ids.push(self.resolve_team_id(key).await?);
            }
            input["teamIds"] = serde_json::json!(team_ids);
        }
        if let Some(ref lead_email) = params.lead {
            let lead_id = self.resolve_user_id(lead_email).await?;
            input["leadId"] = serde_json::Value::String(lead_id);
        }
        if let Some(ref target_date) = params.target_date {
            input["targetDate"] = serde_json::Value::String(target_date.clone());
        }
        if let Some(ref start_date) = params.start_date {
            input["startDate"] = serde_json::Value::String(start_date.clone());
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateProjectData = self
            .client
            .execute_json(queries::CREATE_PROJECT, vars)
            .await?;

        match data.project_create.project {
            Some(project) => Ok(format!(
                "Project created:\n\n{}",
                format::format_project_detail(&project)
            )),
            None => Err(Error::GraphQL("Project creation returned no project".into())),
        }
    }

    async fn handle_update_project(
        &self,
        params: update_project::UpdateProjectParams,
    ) -> Result<String, Error> {
        let project_id = self.resolve_project_id_or_uuid(&params.id).await?;

        let mut input = serde_json::Map::new();
        let mut has_fields = false;

        if let Some(ref name) = params.name {
            input.insert("name".into(), serde_json::Value::String(name.clone()));
            has_fields = true;
        }
        if let Some(ref desc) = params.description {
            input.insert("description".into(), serde_json::Value::String(desc.clone()));
            has_fields = true;
        }
        if let Some(ref state) = params.state {
            input.insert("state".into(), serde_json::Value::String(state.clone()));
            has_fields = true;
        }
        if let Some(ref lead) = params.lead {
            if lead.eq_ignore_ascii_case("none") {
                input.insert("leadId".into(), serde_json::Value::Null);
            } else {
                let lead_id = self.resolve_user_id(lead).await?;
                input.insert("leadId".into(), serde_json::Value::String(lead_id));
            }
            has_fields = true;
        }
        if let Some(ref target_date) = params.target_date {
            if target_date.eq_ignore_ascii_case("none") {
                input.insert("targetDate".into(), serde_json::Value::Null);
            } else {
                input.insert("targetDate".into(), serde_json::Value::String(target_date.clone()));
            }
            has_fields = true;
        }
        if let Some(ref start_date) = params.start_date {
            if start_date.eq_ignore_ascii_case("none") {
                input.insert("startDate".into(), serde_json::Value::Null);
            } else {
                input.insert("startDate".into(), serde_json::Value::String(start_date.clone()));
            }
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput(
                "No fields to update. Provide at least one field.".into(),
            ));
        }

        let vars = serde_json::json!({
            "id": project_id,
            "input": serde_json::Value::Object(input),
        });
        let data: response::UpdateProjectData = self
            .client
            .execute_json(queries::UPDATE_PROJECT, vars)
            .await?;

        match data.project_update.project {
            Some(project) => Ok(format!(
                "Project updated:\n\n{}",
                format::format_project_detail(&project)
            )),
            None => Err(Error::GraphQL("Project update returned no project".into())),
        }
    }

    // ---- Project update handlers ----

    async fn handle_list_project_updates(
        &self,
        params: list_project_updates::ListProjectUpdatesParams,
    ) -> Result<String, Error> {
        let project_id = self.resolve_project_id_or_uuid(&params.project).await?;
        let vars = serde_json::json!({ "id": project_id });
        let data: response::ProjectUpdatesData = self
            .client
            .execute_json(queries::LIST_PROJECT_UPDATES, vars)
            .await?;

        let updates = &data.project.project_updates.nodes;
        if updates.is_empty() {
            return Ok("No project updates found.".to_string());
        }

        let lines: Vec<String> = updates.iter().map(format::format_project_update).collect();
        Ok(format!("Project Updates:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_create_project_update(
        &self,
        params: create_project_update::CreateProjectUpdateParams,
    ) -> Result<String, Error> {
        let project_id = self.resolve_project_id_or_uuid(&params.project).await?;

        let mut input = serde_json::json!({
            "projectId": project_id,
            "body": params.body,
        });

        if let Some(ref health) = params.health {
            input["health"] = serde_json::Value::String(health.clone());
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateProjectUpdateData = self
            .client
            .execute_json(queries::CREATE_PROJECT_UPDATE, vars)
            .await?;

        match data.project_update_create.project_update {
            Some(update) => Ok(format!(
                "Project update created:\n\n{}",
                format::format_project_update(&update)
            )),
            None => Err(Error::GraphQL("Project update creation failed".into())),
        }
    }

    // ---- Project milestone handlers ----

    async fn handle_list_project_milestones(
        &self,
        params: list_project_milestones::ListProjectMilestonesParams,
    ) -> Result<String, Error> {
        let project_id = self.resolve_project_id_or_uuid(&params.project).await?;
        let vars = serde_json::json!({ "id": project_id });
        let data: response::ProjectMilestonesData = self
            .client
            .execute_json(queries::LIST_PROJECT_MILESTONES, vars)
            .await?;

        let milestones = &data.project.project_milestones.nodes;
        if milestones.is_empty() {
            return Ok("No milestones found for this project.".to_string());
        }

        let lines: Vec<String> = milestones
            .iter()
            .map(format::format_project_milestone)
            .collect();
        Ok(format!("Project Milestones:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_create_project_milestone(
        &self,
        params: create_project_milestone::CreateProjectMilestoneParams,
    ) -> Result<String, Error> {
        let project_id = self.resolve_project_id_or_uuid(&params.project).await?;

        let mut input = serde_json::json!({
            "projectId": project_id,
            "name": params.name,
        });

        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(ref target_date) = params.target_date {
            input["targetDate"] = serde_json::Value::String(target_date.clone());
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateProjectMilestoneData = self
            .client
            .execute_json(queries::CREATE_PROJECT_MILESTONE, vars)
            .await?;

        match data.project_milestone_create.project_milestone {
            Some(milestone) => Ok(format!(
                "Milestone created:\n\n{}",
                format::format_project_milestone(&milestone)
            )),
            None => Err(Error::GraphQL("Milestone creation failed".into())),
        }
    }

    // ---- Roadmap / Initiative handlers ----

    async fn handle_list_roadmaps(
        &self,
        params: list_roadmaps::ListRoadmapsParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::RoadmapsData = self
            .client
            .execute_json(queries::LIST_ROADMAPS, vars)
            .await?;

        let roadmaps = &data.roadmaps.nodes;
        if roadmaps.is_empty() {
            return Ok("No roadmaps found.".to_string());
        }

        let lines: Vec<String> = roadmaps.iter().map(format::format_roadmap).collect();
        Ok(format!("Roadmaps:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_list_initiatives(
        &self,
        params: list_initiatives::ListInitiativesParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::InitiativesData = self
            .client
            .execute_json(queries::LIST_INITIATIVES, vars)
            .await?;

        let initiatives = &data.initiatives.nodes;
        if initiatives.is_empty() {
            return Ok("No initiatives found.".to_string());
        }

        let lines: Vec<String> = initiatives.iter().map(format::format_initiative).collect();
        Ok(format!("Initiatives:\n\n{}", lines.join("\n\n")))
    }

    // ---- Notification handlers ----

    async fn handle_list_notifications(
        &self,
        params: list_notifications::ListNotificationsParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::NotificationsData = self
            .client
            .execute_json(queries::LIST_NOTIFICATIONS, vars)
            .await?;

        let notifications = &data.notifications.nodes;
        if notifications.is_empty() {
            return Ok("No notifications.".to_string());
        }

        let lines: Vec<String> = notifications
            .iter()
            .map(format::format_notification)
            .collect();
        Ok(format!("Notifications:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_mark_notification_read(
        &self,
        params: mark_notification_read::MarkNotificationReadParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({
            "id": params.id,
            "input": { "readAt": chrono_now_iso() }
        });
        let data: response::MarkNotificationReadData = self
            .client
            .execute_json(queries::MARK_NOTIFICATION_READ, vars)
            .await?;

        if data.notification_update.success {
            Ok(format!("Notification {} marked as read.", params.id))
        } else {
            Err(Error::GraphQL("Failed to mark notification as read".into()))
        }
    }

    // ---- View handlers ----

    async fn handle_list_views(
        &self,
        params: list_views::ListViewsParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::CustomViewsData = self
            .client
            .execute_json(queries::LIST_VIEWS, vars)
            .await?;

        let views = &data.custom_views.nodes;
        if views.is_empty() {
            return Ok("No custom views found.".to_string());
        }

        let lines: Vec<String> = views.iter().map(format::format_custom_view).collect();
        Ok(format!("Custom Views:\n\n{}", lines.join("\n\n")))
    }

    // ---- Attachment handlers ----

    async fn handle_list_attachments(
        &self,
        params: list_attachments::ListAttachmentsParams,
    ) -> Result<String, Error> {
        let issue_uuid = self.resolve_issue_id(&params.issue_id).await?;
        let vars = serde_json::json!({ "id": issue_uuid });
        let data: response::AttachmentsData = self
            .client
            .execute_json(queries::LIST_ATTACHMENTS, vars)
            .await?;

        let attachments = &data.issue.attachments.nodes;
        if attachments.is_empty() {
            return Ok("No attachments found on this issue.".to_string());
        }

        let lines: Vec<String> = attachments.iter().map(format::format_attachment).collect();
        Ok(format!("Attachments:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_add_attachment(
        &self,
        params: add_attachment::AddAttachmentParams,
    ) -> Result<String, Error> {
        let issue_uuid = self.resolve_issue_id(&params.issue_id).await?;

        let input = serde_json::json!({
            "issueId": issue_uuid,
            "title": params.title,
            "url": params.url,
        });

        let vars = serde_json::json!({ "input": input });
        let data: response::AddAttachmentData = self
            .client
            .execute_json(queries::ADD_ATTACHMENT, vars)
            .await?;

        match data.attachment_create.attachment {
            Some(attachment) => Ok(format!(
                "Attachment added:\n\n{}",
                format::format_attachment(&attachment)
            )),
            None => Err(Error::GraphQL("Attachment creation failed".into())),
        }
    }

    // ---- Reaction handlers ----

    async fn handle_add_reaction(
        &self,
        params: add_reaction::AddReactionParams,
    ) -> Result<String, Error> {
        let input = serde_json::json!({
            "commentId": params.comment_id,
            "emoji": params.emoji,
        });

        let vars = serde_json::json!({ "input": input });
        let data: response::AddReactionData = self
            .client
            .execute_json(queries::ADD_REACTION, vars)
            .await?;

        match data.reaction_create.reaction {
            Some(reaction) => Ok(format!(
                "Reaction added: {} [id: {}]",
                reaction.emoji, reaction.id
            )),
            None => Err(Error::GraphQL("Reaction creation failed".into())),
        }
    }

    async fn handle_remove_reaction(
        &self,
        params: remove_reaction::RemoveReactionParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::RemoveReactionData = self
            .client
            .execute_json(queries::REMOVE_REACTION, vars)
            .await?;

        if data.reaction_delete.success {
            Ok(format!("Reaction {} removed.", params.id))
        } else {
            Err(Error::GraphQL("Reaction removal failed".into()))
        }
    }

    // ---- Favorite handlers ----

    async fn handle_list_favorites(
        &self,
        params: list_favorites::ListFavoritesParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::FavoritesData = self
            .client
            .execute_json(queries::LIST_FAVORITES, vars)
            .await?;

        let favorites = &data.favorites.nodes;
        if favorites.is_empty() {
            return Ok("No favorites found.".to_string());
        }

        let lines: Vec<String> = favorites.iter().map(format::format_favorite).collect();
        Ok(format!("Favorites:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_add_favorite(
        &self,
        params: add_favorite::AddFavoriteParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();

        if let Some(ref issue_id) = params.issue_id {
            let uuid = self.resolve_issue_id(issue_id).await?;
            input.insert("issueId".into(), serde_json::Value::String(uuid));
        } else if let Some(ref project_id) = params.project_id {
            let uuid = self.resolve_project_id_or_uuid(project_id).await?;
            input.insert("projectId".into(), serde_json::Value::String(uuid));
        } else {
            return Err(Error::InvalidInput(
                "Provide either issueId or projectId to favorite.".into(),
            ));
        }

        let vars = serde_json::json!({ "input": serde_json::Value::Object(input) });
        let data: response::AddFavoriteData = self
            .client
            .execute_json(queries::ADD_FAVORITE, vars)
            .await?;

        match data.favorite_create.favorite {
            Some(favorite) => Ok(format!(
                "Favorite added:\n\n{}",
                format::format_favorite(&favorite)
            )),
            None => Err(Error::GraphQL("Favorite creation failed".into())),
        }
    }

    async fn handle_remove_favorite(
        &self,
        params: remove_favorite::RemoveFavoriteParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::RemoveFavoriteData = self
            .client
            .execute_json(queries::REMOVE_FAVORITE, vars)
            .await?;

        if data.favorite_delete.success {
            Ok(format!("Favorite {} removed.", params.id))
        } else {
            Err(Error::GraphQL("Favorite removal failed".into()))
        }
    }

    // ---- Template handlers ----

    async fn handle_list_templates(
        &self,
        _params: list_templates::ListTemplatesParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({});
        let data: response::TemplatesData = self
            .client
            .execute_json(queries::LIST_TEMPLATES, vars)
            .await?;

        let templates = &data.templates;
        if templates.is_empty() {
            return Ok("No templates found.".to_string());
        }

        let lines: Vec<String> = templates.iter().map(format::format_template).collect();
        Ok(format!("Templates:\n\n{}", lines.join("\n\n")))
    }

    // ---- Issue history handlers ----

    async fn handle_get_issue_history(
        &self,
        params: get_issue_history::GetIssueHistoryParams,
    ) -> Result<String, Error> {
        let uuid = self.resolve_issue_id(&params.id).await?;
        let limit = params.limit.unwrap_or(50).min(100);

        let vars = serde_json::json!({ "id": uuid, "first": limit });
        let data: response::IssueHistoryData = self
            .client
            .execute_json(queries::GET_ISSUE_HISTORY, vars)
            .await?;

        let entries = &data.issue.history.nodes;
        if entries.is_empty() {
            return Ok("No history entries found for this issue.".to_string());
        }

        let lines: Vec<String> = entries.iter().map(format::format_history_entry).collect();
        Ok(format!("Issue History:\n\n{}", lines.join("\n\n")))
    }

    // ---- Webhook handlers ----

    async fn handle_list_webhooks(
        &self,
        params: list_webhooks::ListWebhooksParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::WebhooksData = self
            .client
            .execute_json(queries::LIST_WEBHOOKS, vars)
            .await?;

        let webhooks = &data.webhooks.nodes;
        if webhooks.is_empty() {
            return Ok("No webhooks found.".to_string());
        }

        let lines: Vec<String> = webhooks.iter().map(format::format_webhook).collect();
        Ok(format!("Webhooks:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_create_webhook(
        &self,
        params: create_webhook::CreateWebhookParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({
            "url": params.url,
        });

        if let Some(ref label) = params.label {
            input["label"] = serde_json::Value::String(label.clone());
        }
        if let Some(ref resource_types) = params.resource_types {
            let types: Vec<&str> = resource_types
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();
            input["resourceTypes"] = serde_json::json!(types);
        }
        if let Some(ref team_key) = params.team {
            let team_id = self.resolve_team_id(team_key).await?;
            input["teamId"] = serde_json::Value::String(team_id);
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateWebhookData = self
            .client
            .execute_json(queries::CREATE_WEBHOOK, vars)
            .await?;

        match data.webhook_create.webhook {
            Some(webhook) => Ok(format!(
                "Webhook created:\n\n{}",
                format::format_webhook(&webhook)
            )),
            None => Err(Error::GraphQL("Webhook creation failed".into())),
        }
    }

    async fn handle_delete_webhook(
        &self,
        params: delete_webhook::DeleteWebhookParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteWebhookData = self
            .client
            .execute_json(queries::DELETE_WEBHOOK, vars)
            .await?;

        if data.webhook_delete.success {
            Ok(format!("Webhook {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Webhook deletion failed".into()))
        }
    }

    // ---- Integration handlers ----

    async fn handle_list_integrations(
        &self,
        params: list_integrations::ListIntegrationsParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::IntegrationsData = self
            .client
            .execute_json(queries::LIST_INTEGRATIONS, vars)
            .await?;

        let integrations = &data.integrations.nodes;
        if integrations.is_empty() {
            return Ok("No integrations found.".to_string());
        }

        let lines: Vec<String> = integrations
            .iter()
            .map(format::format_integration)
            .collect();
        Ok(format!("Integrations:\n\n{}", lines.join("\n\n")))
    }

    // ---- Audit log handlers ----

    async fn handle_query_audit_log(
        &self,
        params: query_audit_log::QueryAuditLogParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::AuditLogData = self
            .client
            .execute_json(queries::QUERY_AUDIT_LOG, vars)
            .await?;

        let entries = &data.audit_entries.nodes;
        if entries.is_empty() {
            return Ok("No audit log entries found.".to_string());
        }

        let lines: Vec<String> = entries.iter().map(format::format_audit_entry).collect();
        Ok(format!("Audit Log:\n\n{}", lines.join("\n\n")))
    }

    // ---- Team management handlers ----

    async fn handle_create_team(
        &self,
        params: create_team::CreateTeamParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({
            "name": params.name,
        });

        if let Some(ref key) = params.key {
            input["key"] = serde_json::Value::String(key.clone());
        }
        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(ref tz) = params.timezone {
            input["timezone"] = serde_json::Value::String(tz.clone());
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateTeamData = self
            .client
            .execute_json(queries::CREATE_TEAM, vars)
            .await?;

        match data.team_create.team {
            Some(team) => Ok(format!(
                "Team created:\n\n{}",
                format::format_team_detail(&team)
            )),
            None => Err(Error::GraphQL("Team creation failed".into())),
        }
    }

    async fn handle_update_team(
        &self,
        params: update_team::UpdateTeamParams,
    ) -> Result<String, Error> {
        // Resolve team key to UUID if needed
        let team_id = {
            let is_uuid = params.id.len() == 36
                && params.id.chars().all(|c| c.is_ascii_hexdigit() || c == '-');
            if is_uuid {
                params.id.clone()
            } else {
                self.resolve_team_id(&params.id).await?
            }
        };

        let mut input = serde_json::Map::new();
        let mut has_fields = false;

        if let Some(ref name) = params.name {
            input.insert("name".into(), serde_json::Value::String(name.clone()));
            has_fields = true;
        }
        if let Some(ref desc) = params.description {
            input.insert("description".into(), serde_json::Value::String(desc.clone()));
            has_fields = true;
        }
        if let Some(ref tz) = params.timezone {
            input.insert("timezone".into(), serde_json::Value::String(tz.clone()));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput(
                "No fields to update. Provide at least one field.".into(),
            ));
        }

        let vars = serde_json::json!({
            "id": team_id,
            "input": serde_json::Value::Object(input),
        });
        let data: response::UpdateTeamData = self
            .client
            .execute_json(queries::UPDATE_TEAM, vars)
            .await?;

        match data.team_update.team {
            Some(team) => Ok(format!(
                "Team updated:\n\n{}",
                format::format_team_detail(&team)
            )),
            None => Err(Error::GraphQL("Team update failed".into())),
        }
    }

    // ---- Phase 11 handlers ----

    async fn handle_archive_project(
        &self,
        params: archive_project::ArchiveProjectParams,
    ) -> Result<String, Error> {
        let project_id = self.resolve_project_id_or_uuid(&params.id).await?;
        let vars = serde_json::json!({ "id": project_id });
        let data: response::ArchiveProjectData = self
            .client
            .execute_json(queries::ARCHIVE_PROJECT, vars)
            .await?;

        if data.project_archive.success {
            Ok(format!("Project '{}' archived.", params.id))
        } else {
            Err(Error::GraphQL("Project archive failed".into()))
        }
    }

    async fn handle_update_document(
        &self,
        params: update_document::UpdateDocumentParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;

        if let Some(ref title) = params.title {
            input.insert("title".into(), serde_json::Value::String(title.clone()));
            has_fields = true;
        }
        if let Some(ref content) = params.content {
            input.insert("content".into(), serde_json::Value::String(content.clone()));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput(
                "No fields to update. Provide title or content.".into(),
            ));
        }

        let vars = serde_json::json!({
            "id": params.id,
            "input": serde_json::Value::Object(input),
        });
        let data: response::UpdateDocumentData = self
            .client
            .execute_json(queries::UPDATE_DOCUMENT, vars)
            .await?;

        match data.document_update.document {
            Some(doc) => Ok(format!(
                "Document updated:\n\n{}",
                format::format_document_detail(&doc)
            )),
            None => Err(Error::GraphQL("Document update returned no document".into())),
        }
    }

    async fn handle_create_cycle(
        &self,
        params: create_cycle::CreateCycleParams,
    ) -> Result<String, Error> {
        let team_id = self.resolve_team_id(&params.team).await?;

        let mut input = serde_json::json!({
            "teamId": team_id,
            "startsAt": params.starts_at,
            "endsAt": params.ends_at,
        });

        if let Some(ref name) = params.name {
            input["name"] = serde_json::Value::String(name.clone());
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateCycleData = self
            .client
            .execute_json(queries::CREATE_CYCLE, vars)
            .await?;

        match data.cycle_create.cycle {
            Some(cycle) => Ok(format!(
                "Cycle created:\n\n{}",
                format::format_cycle_created(&cycle)
            )),
            None => Err(Error::GraphQL("Cycle creation failed".into())),
        }
    }

    async fn handle_update_label(
        &self,
        params: update_label::UpdateLabelParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;

        if let Some(ref name) = params.name {
            input.insert("name".into(), serde_json::Value::String(name.clone()));
            has_fields = true;
        }
        if let Some(ref color) = params.color {
            input.insert("color".into(), serde_json::Value::String(color.clone()));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput(
                "No fields to update. Provide name or color.".into(),
            ));
        }

        let vars = serde_json::json!({
            "id": params.id,
            "input": serde_json::Value::Object(input),
        });
        let data: response::UpdateLabelData = self
            .client
            .execute_json(queries::UPDATE_LABEL, vars)
            .await?;

        match data.issue_label_update.issue_label {
            Some(label) => Ok(format!("Label updated: {} [id: {}]", label.name, label.id)),
            None => Err(Error::GraphQL("Label update failed".into())),
        }
    }

    async fn handle_archive_label(
        &self,
        params: archive_label::ArchiveLabelParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteLabelData = self
            .client
            .execute_json(queries::DELETE_LABEL, vars)
            .await?;

        if data.issue_label_delete.success {
            Ok(format!("Label {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Label deletion failed".into()))
        }
    }

    async fn handle_unarchive_issue(
        &self,
        params: unarchive_issue::UnarchiveIssueParams,
    ) -> Result<String, Error> {
        let uuid = self.resolve_issue_id(&params.id).await?;
        let vars = serde_json::json!({ "id": uuid });
        let data: response::UnarchiveIssueData = self
            .client
            .execute_json(queries::UNARCHIVE_ISSUE, vars)
            .await?;

        if data.issue_unarchive.success {
            Ok(format!("Issue '{}' unarchived.", params.id))
        } else {
            Err(Error::GraphQL("Issue unarchive failed".into()))
        }
    }
}

/// Generate an ISO 8601 timestamp for the current time (UTC).
fn chrono_now_iso() -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let secs = now.as_secs();
    // Simple UTC ISO format without pulling in chrono
    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let minutes = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;
    // Approximate date calculation
    let mut y = 1970i64;
    let mut remaining = days as i64;
    loop {
        let days_in_year = if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    let month_days: [i64; 12] = [
        31,
        if leap { 29 } else { 28 },
        31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ];
    let mut m = 0usize;
    for (i, &md) in month_days.iter().enumerate() {
        if remaining < md {
            m = i;
            break;
        }
        remaining -= md;
    }
    let d = remaining + 1;
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.000Z",
        y,
        m + 1,
        d,
        hours,
        minutes,
        seconds
    )
}

fn error_result(err: &Error) -> CallToolResult {
    CallToolResult::error(vec![Content::text(format!("Error: {}", err))])
}
