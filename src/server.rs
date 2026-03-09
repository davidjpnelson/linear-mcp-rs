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

    // ---- Phase 12 tools ----

    #[tool(
        name = "bulk_update_issues",
        description = "Batch update multiple issues at once. All specified issues get the same update. Max 50 issues per call."
    )]
    async fn bulk_update_issues(
        &self,
        Parameters(params): Parameters<bulk_update_issues::BulkUpdateIssuesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_bulk_update_issues(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "search_documents",
        description = "Full-text search across all documents in the workspace."
    )]
    async fn search_documents(
        &self,
        Parameters(params): Parameters<search_documents::SearchDocumentsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_search_documents(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_initiative",
        description = "Create a new initiative for tracking high-level goals."
    )]
    async fn create_initiative(
        &self,
        Parameters(params): Parameters<create_initiative::CreateInitiativeParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_initiative(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_initiative",
        description = "Update an existing initiative's name, description, status, owner, or target date."
    )]
    async fn update_initiative(
        &self,
        Parameters(params): Parameters<update_initiative::UpdateInitiativeParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_initiative(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_initiative",
        description = "Permanently delete an initiative."
    )]
    async fn delete_initiative(
        &self,
        Parameters(params): Parameters<delete_initiative::DeleteInitiativeParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_initiative(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_view_issues",
        description = "Get issues matching a custom view's saved filters."
    )]
    async fn get_view_issues(
        &self,
        Parameters(params): Parameters<get_view_issues::GetViewIssuesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_view_issues(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_triage_issues",
        description = "List issues in the triage state for a team. Triage must be enabled for the team."
    )]
    async fn list_triage_issues(
        &self,
        Parameters(params): Parameters<list_triage_issues::ListTriageIssuesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_triage_issues(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "triage_issue",
        description = "Move an issue out of triage by changing its state. Optionally set assignee and priority."
    )]
    async fn triage_issue(
        &self,
        Parameters(params): Parameters<triage_issue::TriageIssueParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_triage_issue(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_issue_from_template",
        description = "Create a new issue using a template. Use list_templates to find template IDs. Any provided fields override template defaults."
    )]
    async fn create_issue_from_template(
        &self,
        Parameters(params): Parameters<create_issue_from_template::CreateIssueFromTemplateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_issue_from_template(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 2: Delete/Archive tools ----

    #[tool(
        name = "delete_document",
        description = "Permanently delete a document."
    )]
    async fn delete_document(
        &self,
        Parameters(params): Parameters<delete_document::DeleteDocumentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_document(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_project_milestone",
        description = "Delete a project milestone."
    )]
    async fn delete_project_milestone(
        &self,
        Parameters(params): Parameters<delete_project_milestone::DeleteProjectMilestoneParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_project_milestone(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_project_update",
        description = "Delete a project status update."
    )]
    async fn delete_project_update(
        &self,
        Parameters(params): Parameters<delete_project_update::DeleteProjectUpdateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_project_update(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_attachment",
        description = "Delete an attachment."
    )]
    async fn delete_attachment(
        &self,
        Parameters(params): Parameters<delete_attachment::DeleteAttachmentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_attachment(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_issue",
        description = "PERMANENTLY delete an issue. This cannot be undone. Use archive_issue for reversible removal."
    )]
    async fn delete_issue(
        &self,
        Parameters(params): Parameters<delete_issue::DeleteIssueParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_issue(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_view",
        description = "Delete a custom view."
    )]
    async fn delete_view(
        &self,
        Parameters(params): Parameters<delete_view::DeleteViewParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_view(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "archive_cycle",
        description = "Archive a cycle."
    )]
    async fn archive_cycle(
        &self,
        Parameters(params): Parameters<archive_cycle::ArchiveCycleParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_archive_cycle(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 3: Update tools ----

    #[tool(
        name = "update_cycle",
        description = "Update an existing cycle's name, description, or dates."
    )]
    async fn update_cycle(
        &self,
        Parameters(params): Parameters<update_cycle::UpdateCycleParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_cycle(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_project_milestone",
        description = "Update a project milestone's name, description, or target date."
    )]
    async fn update_project_milestone(
        &self,
        Parameters(params): Parameters<update_project_milestone::UpdateProjectMilestoneParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_project_milestone(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_project_update",
        description = "Update a project status update's body or health."
    )]
    async fn update_project_update(
        &self,
        Parameters(params): Parameters<update_project_update::UpdateProjectUpdateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_project_update(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_webhook",
        description = "Update an existing webhook's URL, label, enabled status, or resource types."
    )]
    async fn update_webhook(
        &self,
        Parameters(params): Parameters<update_webhook::UpdateWebhookParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_webhook(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_attachment",
        description = "Update an attachment's title or subtitle."
    )]
    async fn update_attachment(
        &self,
        Parameters(params): Parameters<update_attachment::UpdateAttachmentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_attachment(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_view",
        description = "Update a custom view's name, description, color, icon, or sharing."
    )]
    async fn update_view(
        &self,
        Parameters(params): Parameters<update_view::UpdateViewParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_view(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 4: Comment tools ----

    #[tool(
        name = "list_comments",
        description = "List comments on an issue, including reply threads and resolution status."
    )]
    async fn list_comments(
        &self,
        Parameters(params): Parameters<list_comments::ListCommentsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_comments(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "resolve_comment",
        description = "Mark a comment thread as resolved."
    )]
    async fn resolve_comment(
        &self,
        Parameters(params): Parameters<resolve_comment::ResolveCommentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_resolve_comment(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "unresolve_comment",
        description = "Reopen a resolved comment thread."
    )]
    async fn unresolve_comment(
        &self,
        Parameters(params): Parameters<unresolve_comment::UnresolveCommentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_unresolve_comment(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 5: Subscription tools ----

    #[tool(
        name = "subscribe_to_issue",
        description = "Subscribe to an issue to receive notifications about it."
    )]
    async fn subscribe_to_issue(
        &self,
        Parameters(params): Parameters<subscribe_to_issue::SubscribeToIssueParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_subscribe_to_issue(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "unsubscribe_from_issue",
        description = "Unsubscribe from an issue to stop receiving notifications."
    )]
    async fn unsubscribe_from_issue(
        &self,
        Parameters(params): Parameters<unsubscribe_from_issue::UnsubscribeFromIssueParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_unsubscribe_from_issue(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 6: Create tools ----

    #[tool(
        name = "create_view",
        description = "Create a new custom view with optional team scope and sharing."
    )]
    async fn create_view(
        &self,
        Parameters(params): Parameters<create_view::CreateViewParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_view(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 7: Search tools ----

    #[tool(
        name = "search_projects",
        description = "Search for projects by name or description."
    )]
    async fn search_projects(
        &self,
        Parameters(params): Parameters<search_projects::SearchProjectsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_search_projects(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "issue_vcs_branch_search",
        description = "Find the Linear issue associated with a git branch name."
    )]
    async fn issue_vcs_branch_search(
        &self,
        Parameters(params): Parameters<issue_vcs_branch_search::IssueVcsBranchSearchParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_issue_vcs_branch_search(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 8: Agent Session tools ----

    #[tool(
        name = "create_agent_session",
        description = "Create a new agent session on an issue or comment for AI-assisted work tracking."
    )]
    async fn create_agent_session(
        &self,
        Parameters(params): Parameters<create_agent_session::CreateAgentSessionParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_agent_session(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_agent_session",
        description = "Update an agent session's plan or external link."
    )]
    async fn update_agent_session(
        &self,
        Parameters(params): Parameters<update_agent_session::UpdateAgentSessionParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_agent_session(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_agent_activity",
        description = "Log an activity (thinking, tool call, code change, error) within an agent session."
    )]
    async fn create_agent_activity(
        &self,
        Parameters(params): Parameters<create_agent_activity::CreateAgentActivityParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_agent_activity(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_agent_sessions",
        description = "List agent sessions in the workspace."
    )]
    async fn list_agent_sessions(
        &self,
        Parameters(params): Parameters<list_agent_sessions::ListAgentSessionsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_agent_sessions(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_agent_session",
        description = "Get full details of an agent session including activities."
    )]
    async fn get_agent_session(
        &self,
        Parameters(params): Parameters<get_agent_session::GetAgentSessionParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_agent_session(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 9: Customer tools ----

    #[tool(
        name = "list_customers",
        description = "List customers in the workspace."
    )]
    async fn list_customers(
        &self,
        Parameters(params): Parameters<list_customers::ListCustomersParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_customers(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_customer",
        description = "Get full details of a customer."
    )]
    async fn get_customer(
        &self,
        Parameters(params): Parameters<get_customer::GetCustomerParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_customer(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_customer",
        description = "Create a new customer with name, domains, owner, revenue, and size."
    )]
    async fn create_customer(
        &self,
        Parameters(params): Parameters<create_customer::CreateCustomerParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_customer(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_customer",
        description = "Update a customer's name, domains, owner, revenue, or size."
    )]
    async fn update_customer(
        &self,
        Parameters(params): Parameters<update_customer::UpdateCustomerParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_customer(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_customer",
        description = "Delete a customer."
    )]
    async fn delete_customer(
        &self,
        Parameters(params): Parameters<delete_customer::DeleteCustomerParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_customer(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_customer_needs",
        description = "List customer needs in the workspace."
    )]
    async fn list_customer_needs(
        &self,
        Parameters(params): Parameters<list_customer_needs::ListCustomerNeedsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_customer_needs(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_customer_need",
        description = "Create a customer need linking a customer to an issue."
    )]
    async fn create_customer_need(
        &self,
        Parameters(params): Parameters<create_customer_need::CreateCustomerNeedParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_customer_need(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_customer_need",
        description = "Update a customer need."
    )]
    async fn update_customer_need(
        &self,
        Parameters(params): Parameters<update_customer_need::UpdateCustomerNeedParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_customer_need(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 10: Initiative extras ----

    #[tool(
        name = "list_initiative_updates",
        description = "List status updates for an initiative."
    )]
    async fn list_initiative_updates(
        &self,
        Parameters(params): Parameters<list_initiative_updates::ListInitiativeUpdatesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_initiative_updates(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_initiative_update",
        description = "Post a status update to an initiative with optional health indicator."
    )]
    async fn create_initiative_update(
        &self,
        Parameters(params): Parameters<create_initiative_update::CreateInitiativeUpdateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_initiative_update(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "add_project_to_initiative",
        description = "Link a project to an initiative."
    )]
    async fn add_project_to_initiative(
        &self,
        Parameters(params): Parameters<add_project_to_initiative::AddProjectToInitiativeParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_add_project_to_initiative(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "remove_project_from_initiative",
        description = "Remove a project-to-initiative link."
    )]
    async fn remove_project_from_initiative(
        &self,
        Parameters(params): Parameters<remove_project_from_initiative::RemoveProjectFromInitiativeParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_remove_project_from_initiative(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 11: Project relations ----

    #[tool(
        name = "create_project_relation",
        description = "Create a relation between two projects (blocks, dependsOn, related)."
    )]
    async fn create_project_relation(
        &self,
        Parameters(params): Parameters<create_project_relation::CreateProjectRelationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_project_relation(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_project_relation",
        description = "Delete a project relation."
    )]
    async fn delete_project_relation(
        &self,
        Parameters(params): Parameters<delete_project_relation::DeleteProjectRelationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_project_relation(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_project_relations",
        description = "List relations for a project."
    )]
    async fn list_project_relations(
        &self,
        Parameters(params): Parameters<list_project_relations::ListProjectRelationsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_project_relations(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 12: Releases ----

    #[tool(
        name = "list_releases",
        description = "List releases in the workspace (alpha feature)."
    )]
    async fn list_releases(
        &self,
        Parameters(params): Parameters<list_releases::ListReleasesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_releases(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_release",
        description = "Create a new release in a pipeline (alpha feature)."
    )]
    async fn create_release(
        &self,
        Parameters(params): Parameters<create_release::CreateReleaseParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_release(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_release",
        description = "Update a release's name, description, version, commit SHA, or stage (alpha feature)."
    )]
    async fn update_release(
        &self,
        Parameters(params): Parameters<update_release::UpdateReleaseParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_release(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 1A: Workflow State CRUD ----

    #[tool(
        name = "get_workflow_state",
        description = "Get a workflow state by UUID."
    )]
    async fn get_workflow_state(
        &self,
        Parameters(params): Parameters<get_workflow_state::GetWorkflowStateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_workflow_state(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_workflow_state",
        description = "Create a new workflow state for a team."
    )]
    async fn create_workflow_state(
        &self,
        Parameters(params): Parameters<create_workflow_state::CreateWorkflowStateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_workflow_state(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_workflow_state",
        description = "Update a workflow state's name, color, description, or position."
    )]
    async fn update_workflow_state(
        &self,
        Parameters(params): Parameters<update_workflow_state::UpdateWorkflowStateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_workflow_state(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "archive_workflow_state",
        description = "Archive a workflow state."
    )]
    async fn archive_workflow_state(
        &self,
        Parameters(params): Parameters<archive_workflow_state::ArchiveWorkflowStateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_archive_workflow_state(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 1B: Issue Extras ----

    #[tool(
        name = "add_issue_label",
        description = "Add a label to an issue."
    )]
    async fn add_issue_label(
        &self,
        Parameters(params): Parameters<add_issue_label::AddIssueLabelParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_add_issue_label(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "remove_issue_label",
        description = "Remove a label from an issue."
    )]
    async fn remove_issue_label(
        &self,
        Parameters(params): Parameters<remove_issue_label::RemoveIssueLabelParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_remove_issue_label(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "batch_create_issues",
        description = "Batch-create multiple issues at once from a JSON array."
    )]
    async fn batch_create_issues(
        &self,
        Parameters(params): Parameters<batch_create_issues::BatchCreateIssuesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_batch_create_issues(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_issue_relation",
        description = "Update an issue relation's type."
    )]
    async fn update_issue_relation(
        &self,
        Parameters(params): Parameters<update_issue_relation::UpdateIssueRelationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_issue_relation(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_issue_priority_values",
        description = "Get the priority scale configuration (labels and values)."
    )]
    async fn get_issue_priority_values(
        &self,
        Parameters(params): Parameters<get_issue_priority_values::GetIssuePriorityValuesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_issue_priority_values(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 1C: Project Extras ----

    #[tool(
        name = "delete_project",
        description = "Permanently delete a project. Warning: this is irreversible."
    )]
    async fn delete_project(
        &self,
        Parameters(params): Parameters<delete_project::DeleteProjectParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_project(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "unarchive_project",
        description = "Unarchive a previously archived project."
    )]
    async fn unarchive_project(
        &self,
        Parameters(params): Parameters<unarchive_project::UnarchiveProjectParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_unarchive_project(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_project_relation",
        description = "Update a project relation's anchor types."
    )]
    async fn update_project_relation(
        &self,
        Parameters(params): Parameters<update_project_relation::UpdateProjectRelationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_project_relation(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_project_milestone",
        description = "Get a project milestone by UUID."
    )]
    async fn get_project_milestone(
        &self,
        Parameters(params): Parameters<get_project_milestone::GetProjectMilestoneParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_project_milestone(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 1D: Team Extras ----

    #[tool(
        name = "delete_team",
        description = "Permanently delete a team. Warning: this is irreversible."
    )]
    async fn delete_team(
        &self,
        Parameters(params): Parameters<delete_team::DeleteTeamParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_team(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "unarchive_team",
        description = "Unarchive a previously archived team."
    )]
    async fn unarchive_team(
        &self,
        Parameters(params): Parameters<unarchive_team::UnarchiveTeamParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_unarchive_team(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_team",
        description = "Get detailed info about a team by key or UUID."
    )]
    async fn get_team(
        &self,
        Parameters(params): Parameters<get_team::GetTeamParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_team(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 1E: Document Extras ----

    #[tool(
        name = "unarchive_document",
        description = "Unarchive a previously archived document."
    )]
    async fn unarchive_document(
        &self,
        Parameters(params): Parameters<unarchive_document::UnarchiveDocumentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_unarchive_document(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_document_content_history",
        description = "Get the content version history for a document."
    )]
    async fn get_document_content_history(
        &self,
        Parameters(params): Parameters<get_document_content_history::GetDocumentContentHistoryParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_document_content_history(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 1F: Misc High-Value ----

    #[tool(
        name = "get_viewer",
        description = "Get the currently authenticated user's info."
    )]
    async fn get_viewer_tool(
        &self,
        Parameters(_params): Parameters<get_viewer::GetViewerParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_viewer_tool().await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_user",
        description = "Get a user by email or UUID."
    )]
    async fn get_user(
        &self,
        Parameters(params): Parameters<get_user::GetUserParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_user(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_user",
        description = "Update a user's display name, description, or status."
    )]
    async fn update_user(
        &self,
        Parameters(params): Parameters<update_user::UpdateUserParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_user(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_attachment",
        description = "Get an attachment by UUID."
    )]
    async fn get_attachment(
        &self,
        Parameters(params): Parameters<get_attachment::GetAttachmentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_attachment(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_comment",
        description = "Get a comment by UUID."
    )]
    async fn get_comment(
        &self,
        Parameters(params): Parameters<get_comment::GetCommentParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_comment(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_favorite",
        description = "Get a favorite by UUID."
    )]
    async fn get_favorite(
        &self,
        Parameters(params): Parameters<get_favorite::GetFavoriteParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_favorite(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_favorite",
        description = "Update a favorite's sort order, parent, or folder name."
    )]
    async fn update_favorite(
        &self,
        Parameters(params): Parameters<update_favorite::UpdateFavoriteParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_favorite(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_notification",
        description = "Get a notification by UUID."
    )]
    async fn get_notification(
        &self,
        Parameters(params): Parameters<get_notification::GetNotificationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_notification(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 2A: Customer Status CRUD ----

    #[tool(
        name = "list_customer_statuses",
        description = "List all customer statuses."
    )]
    async fn list_customer_statuses(
        &self,
        Parameters(params): Parameters<list_customer_statuses::ListCustomerStatusesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_customer_statuses(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_customer_status",
        description = "Get a customer status by UUID."
    )]
    async fn get_customer_status(
        &self,
        Parameters(params): Parameters<get_customer_status::GetCustomerStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_customer_status(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_customer_status",
        description = "Create a new customer status."
    )]
    async fn create_customer_status(
        &self,
        Parameters(params): Parameters<create_customer_status::CreateCustomerStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_customer_status(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_customer_status",
        description = "Update a customer status."
    )]
    async fn update_customer_status(
        &self,
        Parameters(params): Parameters<update_customer_status::UpdateCustomerStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_customer_status(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_customer_status",
        description = "Delete a customer status."
    )]
    async fn delete_customer_status(
        &self,
        Parameters(params): Parameters<delete_customer_status::DeleteCustomerStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_customer_status(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 2B: Customer Tier CRUD ----

    #[tool(
        name = "list_customer_tiers",
        description = "List all customer tiers."
    )]
    async fn list_customer_tiers(
        &self,
        Parameters(params): Parameters<list_customer_tiers::ListCustomerTiersParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_customer_tiers(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_customer_tier",
        description = "Get a customer tier by UUID."
    )]
    async fn get_customer_tier(
        &self,
        Parameters(params): Parameters<get_customer_tier::GetCustomerTierParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_customer_tier(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_customer_tier",
        description = "Create a new customer tier."
    )]
    async fn create_customer_tier(
        &self,
        Parameters(params): Parameters<create_customer_tier::CreateCustomerTierParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_customer_tier(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_customer_tier",
        description = "Update a customer tier."
    )]
    async fn update_customer_tier(
        &self,
        Parameters(params): Parameters<update_customer_tier::UpdateCustomerTierParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_customer_tier(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_customer_tier",
        description = "Delete a customer tier."
    )]
    async fn delete_customer_tier(
        &self,
        Parameters(params): Parameters<delete_customer_tier::DeleteCustomerTierParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_customer_tier(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 2C: Customer Extras ----

    #[tool(
        name = "merge_customers",
        description = "Merge two customers (source into target)."
    )]
    async fn merge_customers(
        &self,
        Parameters(params): Parameters<merge_customers::MergeCustomersParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_merge_customers(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_customer_need",
        description = "Get a customer need by UUID."
    )]
    async fn get_customer_need(
        &self,
        Parameters(params): Parameters<get_customer_need::GetCustomerNeedParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_customer_need(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "archive_customer_need",
        description = "Archive a customer need."
    )]
    async fn archive_customer_need(
        &self,
        Parameters(params): Parameters<archive_customer_need::ArchiveCustomerNeedParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_archive_customer_need(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "unarchive_customer_need",
        description = "Unarchive a customer need."
    )]
    async fn unarchive_customer_need(
        &self,
        Parameters(params): Parameters<unarchive_customer_need::UnarchiveCustomerNeedParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_unarchive_customer_need(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_customer_need",
        description = "Delete a customer need."
    )]
    async fn delete_customer_need(
        &self,
        Parameters(params): Parameters<delete_customer_need::DeleteCustomerNeedParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_customer_need(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 2D: Initiative Extras ----

    #[tool(
        name = "archive_initiative",
        description = "Archive an initiative."
    )]
    async fn archive_initiative(
        &self,
        Parameters(params): Parameters<archive_initiative::ArchiveInitiativeParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_archive_initiative(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "unarchive_initiative",
        description = "Unarchive an initiative."
    )]
    async fn unarchive_initiative(
        &self,
        Parameters(params): Parameters<unarchive_initiative::UnarchiveInitiativeParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_unarchive_initiative(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_initiative_to_project",
        description = "Update an initiative-to-project link's sort order."
    )]
    async fn update_initiative_to_project(
        &self,
        Parameters(params): Parameters<update_initiative_to_project::UpdateInitiativeToProjectParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_initiative_to_project(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "archive_initiative_update",
        description = "Archive an initiative update."
    )]
    async fn archive_initiative_update(
        &self,
        Parameters(params): Parameters<archive_initiative_update::ArchiveInitiativeUpdateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_archive_initiative_update(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "unarchive_initiative_update",
        description = "Unarchive an initiative update."
    )]
    async fn unarchive_initiative_update(
        &self,
        Parameters(params): Parameters<unarchive_initiative_update::UnarchiveInitiativeUpdateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_unarchive_initiative_update(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 3A: Release Extras ----

    #[tool(
        name = "get_release",
        description = "Get a release by UUID."
    )]
    async fn get_release(
        &self,
        Parameters(params): Parameters<get_release::GetReleaseParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_release(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "archive_release",
        description = "Archive a release."
    )]
    async fn archive_release(
        &self,
        Parameters(params): Parameters<archive_release::ArchiveReleaseParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_archive_release(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_release",
        description = "Permanently delete a release."
    )]
    async fn delete_release(
        &self,
        Parameters(params): Parameters<delete_release::DeleteReleaseParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_release(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "unarchive_release",
        description = "Unarchive a release."
    )]
    async fn unarchive_release(
        &self,
        Parameters(params): Parameters<unarchive_release::UnarchiveReleaseParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_unarchive_release(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "search_releases",
        description = "Search releases by name or version."
    )]
    async fn search_releases(
        &self,
        Parameters(params): Parameters<search_releases::SearchReleasesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_search_releases(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 3B: Release Pipeline CRUD ----

    #[tool(
        name = "list_release_pipelines",
        description = "List all release pipelines."
    )]
    async fn list_release_pipelines(
        &self,
        Parameters(params): Parameters<list_release_pipelines::ListReleasePipelinesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_release_pipelines(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_release_pipeline",
        description = "Get a release pipeline by UUID."
    )]
    async fn get_release_pipeline(
        &self,
        Parameters(params): Parameters<get_release_pipeline::GetReleasePipelineParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_release_pipeline(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_release_pipeline",
        description = "Create a new release pipeline."
    )]
    async fn create_release_pipeline(
        &self,
        Parameters(params): Parameters<create_release_pipeline::CreateReleasePipelineParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_release_pipeline(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_release_pipeline",
        description = "Update a release pipeline."
    )]
    async fn update_release_pipeline(
        &self,
        Parameters(params): Parameters<update_release_pipeline::UpdateReleasePipelineParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_release_pipeline(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_release_pipeline",
        description = "Delete a release pipeline."
    )]
    async fn delete_release_pipeline(
        &self,
        Parameters(params): Parameters<delete_release_pipeline::DeleteReleasePipelineParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_release_pipeline(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 3C: Release Stage CRUD ----

    #[tool(
        name = "list_release_stages",
        description = "List all release stages."
    )]
    async fn list_release_stages(
        &self,
        Parameters(params): Parameters<list_release_stages::ListReleaseStagesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_release_stages(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_release_stage",
        description = "Get a release stage by UUID."
    )]
    async fn get_release_stage(
        &self,
        Parameters(params): Parameters<get_release_stage::GetReleaseStageParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_release_stage(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_release_stage",
        description = "Create a new release stage in a pipeline."
    )]
    async fn create_release_stage(
        &self,
        Parameters(params): Parameters<create_release_stage::CreateReleaseStageParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_release_stage(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_release_stage",
        description = "Update a release stage."
    )]
    async fn update_release_stage(
        &self,
        Parameters(params): Parameters<update_release_stage::UpdateReleaseStageParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_release_stage(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 3D: Issue-to-Release ----

    #[tool(
        name = "list_issue_to_releases",
        description = "List issue-to-release links."
    )]
    async fn list_issue_to_releases(
        &self,
        Parameters(params): Parameters<list_issue_to_releases::ListIssueToReleasesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_issue_to_releases(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_issue_to_release",
        description = "Get an issue-to-release link by UUID."
    )]
    async fn get_issue_to_release(
        &self,
        Parameters(params): Parameters<get_issue_to_release::GetIssueToReleaseParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_issue_to_release(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "add_issue_to_release",
        description = "Add an issue to a release."
    )]
    async fn add_issue_to_release(
        &self,
        Parameters(params): Parameters<add_issue_to_release::AddIssueToReleaseParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_add_issue_to_release(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "remove_issue_from_release",
        description = "Remove an issue from a release."
    )]
    async fn remove_issue_from_release(
        &self,
        Parameters(params): Parameters<remove_issue_from_release::RemoveIssueFromReleaseParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_remove_issue_from_release(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 4A: Project Status CRUD ----

    #[tool(
        name = "list_project_statuses",
        description = "List all project statuses."
    )]
    async fn list_project_statuses(
        &self,
        Parameters(params): Parameters<list_project_statuses::ListProjectStatusesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_project_statuses(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_project_status",
        description = "Get a project status by UUID."
    )]
    async fn get_project_status(
        &self,
        Parameters(params): Parameters<get_project_status::GetProjectStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_project_status(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_project_status",
        description = "Create a new project status."
    )]
    async fn create_project_status(
        &self,
        Parameters(params): Parameters<create_project_status::CreateProjectStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_project_status(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_project_status",
        description = "Update a project status."
    )]
    async fn update_project_status(
        &self,
        Parameters(params): Parameters<update_project_status::UpdateProjectStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_project_status(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "archive_project_status",
        description = "Archive a project status."
    )]
    async fn archive_project_status(
        &self,
        Parameters(params): Parameters<archive_project_status::ArchiveProjectStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_archive_project_status(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "unarchive_project_status",
        description = "Unarchive a project status."
    )]
    async fn unarchive_project_status(
        &self,
        Parameters(params): Parameters<unarchive_project_status::UnarchiveProjectStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_unarchive_project_status(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 4B: Project Labels CRUD ----

    #[tool(
        name = "list_project_labels",
        description = "List all project labels."
    )]
    async fn list_project_labels(
        &self,
        Parameters(params): Parameters<list_project_labels::ListProjectLabelsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_project_labels(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_project_label",
        description = "Get a project label by UUID."
    )]
    async fn get_project_label(
        &self,
        Parameters(params): Parameters<get_project_label::GetProjectLabelParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_project_label(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_project_label",
        description = "Create a new project label."
    )]
    async fn create_project_label(
        &self,
        Parameters(params): Parameters<create_project_label::CreateProjectLabelParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_project_label(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_project_label",
        description = "Update a project label."
    )]
    async fn update_project_label(
        &self,
        Parameters(params): Parameters<update_project_label::UpdateProjectLabelParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_project_label(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_project_label",
        description = "Delete a project label."
    )]
    async fn delete_project_label(
        &self,
        Parameters(params): Parameters<delete_project_label::DeleteProjectLabelParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_project_label(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 5A: Team Membership CRUD ----

    #[tool(
        name = "list_team_memberships",
        description = "List team memberships."
    )]
    async fn list_team_memberships(
        &self,
        Parameters(params): Parameters<list_team_memberships::ListTeamMembershipsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_team_memberships(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_team_membership",
        description = "Get a team membership by UUID."
    )]
    async fn get_team_membership(
        &self,
        Parameters(params): Parameters<get_team_membership::GetTeamMembershipParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_team_membership(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_team_membership",
        description = "Add a user to a team."
    )]
    async fn create_team_membership(
        &self,
        Parameters(params): Parameters<create_team_membership::CreateTeamMembershipParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_team_membership(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_team_membership",
        description = "Update a team membership (e.g. toggle owner)."
    )]
    async fn update_team_membership(
        &self,
        Parameters(params): Parameters<update_team_membership::UpdateTeamMembershipParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_team_membership(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_team_membership",
        description = "Remove a user from a team."
    )]
    async fn delete_team_membership(
        &self,
        Parameters(params): Parameters<delete_team_membership::DeleteTeamMembershipParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_team_membership(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 5B: Notification Subscriptions ----

    #[tool(
        name = "list_notification_subscriptions",
        description = "List notification subscriptions."
    )]
    async fn list_notification_subscriptions(
        &self,
        Parameters(params): Parameters<list_notification_subscriptions::ListNotificationSubscriptionsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_notification_subscriptions(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_notification_subscription",
        description = "Get a notification subscription by UUID."
    )]
    async fn get_notification_subscription(
        &self,
        Parameters(params): Parameters<get_notification_subscription::GetNotificationSubscriptionParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_notification_subscription(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_notification_subscription",
        description = "Create a notification subscription for a team, project, or label."
    )]
    async fn create_notification_subscription(
        &self,
        Parameters(params): Parameters<create_notification_subscription::CreateNotificationSubscriptionParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_notification_subscription(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_notification_subscription",
        description = "Update a notification subscription."
    )]
    async fn update_notification_subscription(
        &self,
        Parameters(params): Parameters<update_notification_subscription::UpdateNotificationSubscriptionParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_notification_subscription(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_notifications_unread_count",
        description = "Get the count of unread notifications."
    )]
    async fn get_notifications_unread_count(
        &self,
        Parameters(_params): Parameters<get_notifications_unread_count::GetNotificationsUnreadCountParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_notifications_unread_count().await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 6A: Template CRUD ----

    #[tool(
        name = "get_template",
        description = "Get a template by UUID."
    )]
    async fn get_template(
        &self,
        Parameters(params): Parameters<get_template::GetTemplateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_template(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_template",
        description = "Create a new template (issue, project, or document)."
    )]
    async fn create_template(
        &self,
        Parameters(params): Parameters<create_template::CreateTemplateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_template(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_template",
        description = "Update an existing template."
    )]
    async fn update_template(
        &self,
        Parameters(params): Parameters<update_template::UpdateTemplateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_template(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_template",
        description = "Delete a template."
    )]
    async fn delete_template(
        &self,
        Parameters(params): Parameters<delete_template::DeleteTemplateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_template(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 6B: Entity External Links ----

    #[tool(
        name = "get_entity_external_link",
        description = "Get an entity external link by UUID."
    )]
    async fn get_entity_external_link(
        &self,
        Parameters(params): Parameters<get_entity_external_link::GetEntityExternalLinkParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_entity_external_link(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_entity_external_link",
        description = "Create an external link on an initiative, project, or team."
    )]
    async fn create_entity_external_link(
        &self,
        Parameters(params): Parameters<create_entity_external_link::CreateEntityExternalLinkParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_entity_external_link(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_entity_external_link",
        description = "Update an entity external link."
    )]
    async fn update_entity_external_link(
        &self,
        Parameters(params): Parameters<update_entity_external_link::UpdateEntityExternalLinkParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_entity_external_link(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_entity_external_link",
        description = "Delete an entity external link."
    )]
    async fn delete_entity_external_link(
        &self,
        Parameters(params): Parameters<delete_entity_external_link::DeleteEntityExternalLinkParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_entity_external_link(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 6C: Emoji CRUD ----

    #[tool(
        name = "list_emojis",
        description = "List custom emojis."
    )]
    async fn list_emojis(
        &self,
        Parameters(params): Parameters<list_emojis::ListEmojisParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_emojis(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_emoji",
        description = "Get a custom emoji by UUID."
    )]
    async fn get_emoji(
        &self,
        Parameters(params): Parameters<get_emoji::GetEmojiParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_emoji(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_emoji",
        description = "Create a custom emoji."
    )]
    async fn create_emoji(
        &self,
        Parameters(params): Parameters<create_emoji::CreateEmojiParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_emoji(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_emoji",
        description = "Delete a custom emoji."
    )]
    async fn delete_emoji(
        &self,
        Parameters(params): Parameters<delete_emoji::DeleteEmojiParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_emoji(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 6D: Initiative Relations ----

    #[tool(
        name = "list_initiative_relations",
        description = "List initiative relations."
    )]
    async fn list_initiative_relations(
        &self,
        Parameters(params): Parameters<list_initiative_relations::ListInitiativeRelationsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_initiative_relations(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_initiative_relation",
        description = "Get an initiative relation by UUID."
    )]
    async fn get_initiative_relation(
        &self,
        Parameters(params): Parameters<get_initiative_relation::GetInitiativeRelationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_initiative_relation(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_initiative_relation",
        description = "Create a relation between two initiatives."
    )]
    async fn create_initiative_relation(
        &self,
        Parameters(params): Parameters<create_initiative_relation::CreateInitiativeRelationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_initiative_relation(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_initiative_relation",
        description = "Update an initiative relation's sort order."
    )]
    async fn update_initiative_relation(
        &self,
        Parameters(params): Parameters<update_initiative_relation::UpdateInitiativeRelationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_initiative_relation(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_initiative_relation",
        description = "Delete an initiative relation."
    )]
    async fn delete_initiative_relation(
        &self,
        Parameters(params): Parameters<delete_initiative_relation::DeleteInitiativeRelationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_initiative_relation(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 7A: Time Schedule CRUD ----

    #[tool(
        name = "list_time_schedules",
        description = "List time schedules."
    )]
    async fn list_time_schedules(
        &self,
        Parameters(params): Parameters<list_time_schedules::ListTimeSchedulesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_time_schedules(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_time_schedule",
        description = "Get a time schedule by UUID."
    )]
    async fn get_time_schedule(
        &self,
        Parameters(params): Parameters<get_time_schedule::GetTimeScheduleParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_time_schedule(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_time_schedule",
        description = "Create a new time schedule."
    )]
    async fn create_time_schedule(
        &self,
        Parameters(params): Parameters<create_time_schedule::CreateTimeScheduleParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_time_schedule(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_time_schedule",
        description = "Update a time schedule."
    )]
    async fn update_time_schedule(
        &self,
        Parameters(params): Parameters<update_time_schedule::UpdateTimeScheduleParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_time_schedule(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_time_schedule",
        description = "Delete a time schedule."
    )]
    async fn delete_time_schedule(
        &self,
        Parameters(params): Parameters<delete_time_schedule::DeleteTimeScheduleParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_time_schedule(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 7B: Triage Responsibility CRUD ----

    #[tool(
        name = "list_triage_responsibilities",
        description = "List triage responsibilities."
    )]
    async fn list_triage_responsibilities(
        &self,
        Parameters(params): Parameters<list_triage_responsibilities::ListTriageResponsibilitiesParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_triage_responsibilities(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_triage_responsibility",
        description = "Get a triage responsibility by UUID."
    )]
    async fn get_triage_responsibility(
        &self,
        Parameters(params): Parameters<get_triage_responsibility::GetTriageResponsibilityParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_triage_responsibility(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_triage_responsibility",
        description = "Create a triage responsibility for a team."
    )]
    async fn create_triage_responsibility(
        &self,
        Parameters(params): Parameters<create_triage_responsibility::CreateTriageResponsibilityParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_triage_responsibility(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_triage_responsibility",
        description = "Update a triage responsibility."
    )]
    async fn update_triage_responsibility(
        &self,
        Parameters(params): Parameters<update_triage_responsibility::UpdateTriageResponsibilityParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_triage_responsibility(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_triage_responsibility",
        description = "Delete a triage responsibility."
    )]
    async fn delete_triage_responsibility(
        &self,
        Parameters(params): Parameters<delete_triage_responsibility::DeleteTriageResponsibilityParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_triage_responsibility(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 7C: Git Automation ----

    #[tool(
        name = "create_git_automation_state",
        description = "Create a git automation state mapping for a team."
    )]
    async fn create_git_automation_state(
        &self,
        Parameters(params): Parameters<create_git_automation_state::CreateGitAutomationStateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_git_automation_state(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_git_automation_state",
        description = "Update a git automation state mapping."
    )]
    async fn update_git_automation_state(
        &self,
        Parameters(params): Parameters<update_git_automation_state::UpdateGitAutomationStateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_git_automation_state(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_git_automation_state",
        description = "Delete a git automation state mapping."
    )]
    async fn delete_git_automation_state(
        &self,
        Parameters(params): Parameters<delete_git_automation_state::DeleteGitAutomationStateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_git_automation_state(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_git_automation_target_branch",
        description = "Create a git automation target branch pattern for a team."
    )]
    async fn create_git_automation_target_branch(
        &self,
        Parameters(params): Parameters<create_git_automation_target_branch::CreateGitAutomationTargetBranchParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_git_automation_target_branch(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_git_automation_target_branch",
        description = "Update a git automation target branch pattern."
    )]
    async fn update_git_automation_target_branch(
        &self,
        Parameters(params): Parameters<update_git_automation_target_branch::UpdateGitAutomationTargetBranchParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_git_automation_target_branch(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_git_automation_target_branch",
        description = "Delete a git automation target branch pattern."
    )]
    async fn delete_git_automation_target_branch(
        &self,
        Parameters(params): Parameters<delete_git_automation_target_branch::DeleteGitAutomationTargetBranchParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_git_automation_target_branch(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 8A: Email Intake ----

    #[tool(
        name = "get_email_intake_address",
        description = "Get an email intake address by UUID."
    )]
    async fn get_email_intake_address(
        &self,
        Parameters(params): Parameters<get_email_intake_address::GetEmailIntakeAddressParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_email_intake_address(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "create_email_intake_address",
        description = "Create an email intake address."
    )]
    async fn create_email_intake_address(
        &self,
        Parameters(params): Parameters<create_email_intake_address::CreateEmailIntakeAddressParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_create_email_intake_address(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_email_intake_address",
        description = "Update an email intake address."
    )]
    async fn update_email_intake_address(
        &self,
        Parameters(params): Parameters<update_email_intake_address::UpdateEmailIntakeAddressParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_email_intake_address(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "delete_email_intake_address",
        description = "Delete an email intake address."
    )]
    async fn delete_email_intake_address(
        &self,
        Parameters(params): Parameters<delete_email_intake_address::DeleteEmailIntakeAddressParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_delete_email_intake_address(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    // ---- Phase 8B: Remaining Misc ----

    #[tool(
        name = "list_archived_teams",
        description = "List archived teams."
    )]
    async fn list_archived_teams(
        &self,
        Parameters(params): Parameters<list_archived_teams::ListArchivedTeamsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_archived_teams(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_rate_limit_status",
        description = "Get current API rate limit status."
    )]
    async fn get_rate_limit_status(
        &self,
        Parameters(_params): Parameters<get_rate_limit_status::GetRateLimitStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_rate_limit_status().await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_organization",
        description = "Get the current organization's info."
    )]
    async fn get_organization(
        &self,
        Parameters(_params): Parameters<get_organization::GetOrganizationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_organization().await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_application_info",
        description = "Get the current API application's info."
    )]
    async fn get_application_info(
        &self,
        Parameters(params): Parameters<get_application_info::GetApplicationInfoParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_application_info(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "semantic_search",
        description = "Semantic search across Linear entities."
    )]
    async fn semantic_search(
        &self,
        Parameters(params): Parameters<semantic_search::SemanticSearchParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_semantic_search(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "attach_link_url",
        description = "Attach a URL to an issue."
    )]
    async fn attach_link_url(
        &self,
        Parameters(params): Parameters<attach_link_url::AttachLinkUrlParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_attach_link_url(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_attachments_for_url",
        description = "Get all attachments matching a URL."
    )]
    async fn get_attachments_for_url(
        &self,
        Parameters(params): Parameters<get_attachments_for_url::GetAttachmentsForUrlParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_attachments_for_url(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_issue_filter_suggestion",
        description = "Get AI-powered issue filter suggestion from a natural language prompt."
    )]
    async fn get_issue_filter_suggestion(
        &self,
        Parameters(params): Parameters<get_issue_filter_suggestion::GetIssueFilterSuggestionParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_issue_filter_suggestion(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_project_filter_suggestion",
        description = "Get AI-powered project filter suggestion from a natural language prompt."
    )]
    async fn get_project_filter_suggestion(
        &self,
        Parameters(params): Parameters<get_project_filter_suggestion::GetProjectFilterSuggestionParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_project_filter_suggestion(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_custom_view_suggestion",
        description = "Get AI-powered custom view suggestion from a natural language prompt."
    )]
    async fn get_custom_view_suggestion(
        &self,
        Parameters(params): Parameters<get_custom_view_suggestion::GetCustomViewSuggestionParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_custom_view_suggestion(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "check_custom_view_has_subscribers",
        description = "Check if a custom view has subscribers."
    )]
    async fn check_custom_view_has_subscribers(
        &self,
        Parameters(params): Parameters<check_custom_view_has_subscribers::CheckCustomViewHasSubscribersParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_check_custom_view_has_subscribers(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "search_issue_figma_file_key",
        description = "Search for issues linked to a Figma file key."
    )]
    async fn search_issue_figma_file_key(
        &self,
        Parameters(params): Parameters<search_issue_figma_file_key::SearchIssueFigmaFileKeyParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_search_issue_figma_file_key(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "update_initiative_update",
        description = "Update an initiative update's body or health."
    )]
    async fn update_initiative_update(
        &self,
        Parameters(params): Parameters<update_initiative_update::UpdateInitiativeUpdateParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_update_initiative_update(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_comments_all",
        description = "List all comments globally (not per-issue)."
    )]
    async fn list_comments_all(
        &self,
        Parameters(params): Parameters<list_comments_all::ListCommentsAllParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_comments_all(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_issue_label",
        description = "Get an issue label by UUID."
    )]
    async fn get_issue_label(
        &self,
        Parameters(params): Parameters<get_issue_label::GetIssueLabelParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_issue_label(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "get_issue_relation",
        description = "Get an issue relation by UUID."
    )]
    async fn get_issue_relation(
        &self,
        Parameters(params): Parameters<get_issue_relation::GetIssueRelationParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_get_issue_relation(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_issue_relations",
        description = "List all issue relations."
    )]
    async fn list_issue_relations(
        &self,
        Parameters(params): Parameters<list_issue_relations::ListIssueRelationsParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_issue_relations(params).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(error_result(&e)),
        }
    }

    #[tool(
        name = "list_external_users",
        description = "List external users."
    )]
    async fn list_external_users(
        &self,
        Parameters(params): Parameters<list_external_users::ListExternalUsersParams>,
    ) -> Result<CallToolResult, McpError> {
        match self.handle_list_external_users(params).await {
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
        let is_uuid = team_key.len() == 36
            && team_key.chars().all(|c| c.is_ascii_hexdigit() || c == '-');
        if is_uuid {
            return Ok(team_key.to_string());
        }
        let filter = filters::TeamFilter {
            key: Some(filters::StringFilter::eq_exact(team_key.to_uppercase())),
            ..Default::default()
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
                ..Default::default()
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
        if params.completed_before.is_some() || params.completed_after.is_some() {
            issue_filters.push(filters::completed_at_filter(
                params.completed_before.as_deref(),
                params.completed_after.as_deref(),
            ));
        }
        if params.canceled_before.is_some() || params.canceled_after.is_some() {
            issue_filters.push(filters::canceled_at_filter(
                params.canceled_before.as_deref(),
                params.canceled_after.as_deref(),
            ));
        }
        if let Some(ref snoozed) = params.snoozed_until_after {
            issue_filters.push(filters::snoozed_until_at_filter(snoozed));
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
                        description: t.description.clone(),
                        timezone: t.timezone.clone(),
                        triage_enabled: t.triage_enabled,
                        default_issue_state: t.default_issue_state.clone(),
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
                    ..Default::default()
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
            let label_ids = self.resolve_label_ids(label_names, Some(&params.team)).await?;
            input["labelIds"] = serde_json::json!(label_ids);
        }
        if let Some(ref project_name) = params.project {
            let project_id = self.resolve_project_id_or_uuid(project_name).await?;
            input["projectId"] = serde_json::Value::String(project_id);
        }
        if let Some(ref parent_identifier) = params.parent {
            let parent_id = self.resolve_issue_id(parent_identifier).await?;
            input["parentId"] = serde_json::Value::String(parent_id);
        }
        if let Some(ref cycle_id) = params.cycle_id {
            input["cycleId"] = serde_json::Value::String(cycle_id.clone());
        }
        if let Some(ref subscriber_emails) = params.subscribers {
            let emails: Vec<&str> = subscriber_emails.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            let mut subscriber_ids = Vec::new();
            for email in emails {
                subscriber_ids.push(self.resolve_user_id(email).await?);
            }
            input["subscriberIds"] = serde_json::json!(subscriber_ids);
        }
        if let Some(ref milestone_name) = params.project_milestone {
            if let Some(ref project_name) = params.project {
                let project_id = self.resolve_project_id_or_uuid(project_name).await?;
                let milestone_id = self.resolve_project_milestone_id(milestone_name, &project_id).await?;
                input["projectMilestoneId"] = serde_json::Value::String(milestone_id);
            } else {
                return Err(Error::InvalidInput("projectMilestone requires a project to be specified".into()));
            }
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

        // Fetch issue metadata once if any dependent fields need it
        let needs_issue_data = params.status.is_some() || params.labels.is_some() || params.project_milestone.is_some();
        let issue_data = if needs_issue_data {
            let issue_vars = serde_json::json!({ "id": uuid });
            Some(self.client.execute_json::<response::IssueData>(queries::GET_ISSUE, issue_vars).await?)
        } else {
            None
        };

        let team_key = if params.status.is_some() || params.labels.is_some() {
            Some(
                issue_data.as_ref().unwrap()
                    .issue
                    .team
                    .as_ref()
                    .map(|t| t.key.clone())
                    .ok_or_else(|| Error::InvalidInput("Issue has no team".into()))?,
            )
        } else {
            None
        };

        if let Some(ref status) = params.status {
            let key = team_key.as_ref().unwrap();
            let state_id = self.resolve_state_id(status, key).await?;
            input.insert("stateId".into(), serde_json::Value::String(state_id));
            has_fields = true;
        }

        if let Some(ref label_names) = params.labels {
            let label_ids = self.resolve_label_ids(label_names, team_key.as_deref()).await?;
            input.insert("labelIds".into(), serde_json::json!(label_ids));
            has_fields = true;
        }
        if let Some(ref project_name) = params.project {
            if project_name.eq_ignore_ascii_case("none") {
                input.insert("projectId".into(), serde_json::Value::Null);
            } else {
                let project_id = self.resolve_project_id_or_uuid(project_name).await?;
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
        if let Some(ref cycle_id) = params.cycle_id {
            input.insert("cycleId".into(), serde_json::Value::String(cycle_id.clone()));
            has_fields = true;
        }
        // Subscribers are handled additively after the main update via issueSubscribe
        if let Some(ref milestone_name) = params.project_milestone {
            let project_id = issue_data.as_ref().unwrap().issue.project.as_ref()
                .map(|p| p.id.clone())
                .ok_or_else(|| Error::InvalidInput("Issue has no project — cannot resolve milestone".into()))?;
            let milestone_id = self.resolve_project_milestone_id(milestone_name, &project_id).await?;
            input.insert("projectMilestoneId".into(), serde_json::Value::String(milestone_id));
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

        let result = match data.issue_update.issue {
            Some(issue) => {
                let detail = format::format_issue_detail(&issue);
                Ok(format!("Issue updated:\n\n{}", detail))
            }
            None => Err(Error::GraphQL("Issue update returned no issue".into())),
        };

        // Add subscribers additively via issueSubscribe (after the main update)
        if let Some(ref subscriber_emails) = params.subscribers {
            let emails: Vec<&str> = subscriber_emails.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            for email in emails {
                let user_id = self.resolve_user_id(email).await?;
                let sub_vars = serde_json::json!({ "id": uuid, "userId": user_id });
                let _: response::SubscribeToIssueData = self
                    .client
                    .execute_json(queries::SUBSCRIBE_TO_ISSUE, sub_vars)
                    .await?;
            }
        }

        result
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
                    ..Default::default()
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
    /// When `team_key` is provided, labels are scoped to that team (avoids
    /// ambiguity when workspace and team labels share the same name).
    async fn resolve_label_ids(&self, label_names: &str, team_key: Option<&str>) -> Result<Vec<String>, Error> {
        let names: Vec<&str> = label_names.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        if names.is_empty() {
            return Ok(Vec::new());
        }

        // Build an OR filter for names, with team scoping at the top level
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
            team: team_key.map(|key| {
                let is_uuid = key.len() == 36 && key.chars().all(|c| c.is_ascii_hexdigit() || c == '-');
                if is_uuid {
                    filters::TeamFilter { id: Some(filters::StringFilter::eq_exact(key)), ..Default::default() }
                } else {
                    filters::TeamFilter { key: Some(filters::StringFilter::eq_ignore_case(key)), ..Default::default() }
                }
            }),
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

    /// Resolve a single label name to its UUID.
    async fn resolve_label_id(&self, label_name: &str, team_key: Option<&str>) -> Result<String, Error> {
        let ids = self.resolve_label_ids(label_name, team_key).await?;
        ids.into_iter()
            .next()
            .ok_or_else(|| Error::NotFound(format!("Label '{}' not found", label_name)))
    }

    /// Resolve an issue UUID to its team key (e.g. "ENG").
    async fn resolve_team_key_from_issue(&self, issue_uuid: &str) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": issue_uuid });
        let data: serde_json::Value = self
            .client
            .execute_json(
                r#"query($id: String!) { issue(id: $id) { team { id key } } }"#,
                vars,
            )
            .await?;
        data.get("issue")
            .and_then(|i| i.get("team"))
            .and_then(|t| t.get("key"))
            .and_then(|k| k.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| Error::NotFound(format!("Could not resolve team key for issue {}", issue_uuid)))
    }

    /// Batch-resolve issue UUIDs to their team keys in a single query.
    async fn resolve_team_keys_from_issues(&self, uuids: &[String]) -> Result<Vec<String>, Error> {
        let vars = serde_json::json!({
            "first": uuids.len(),
            "filter": { "id": { "in": uuids } },
        });
        let data: serde_json::Value = self
            .client
            .execute_json(
                r#"query($first: Int!, $filter: IssueFilter) { issues(first: $first, filter: $filter) { nodes { id team { key } } } }"#,
                vars,
            )
            .await?;
        let nodes = data.get("issues")
            .and_then(|i| i.get("nodes"))
            .and_then(|n| n.as_array())
            .ok_or_else(|| Error::NotFound("Failed to batch-resolve issue teams".into()))?;

        // Build a map from issue UUID to team key
        let mut map = std::collections::HashMap::new();
        for node in nodes {
            if let (Some(id), Some(key)) = (
                node.get("id").and_then(|v| v.as_str()),
                node.get("team").and_then(|t| t.get("key")).and_then(|k| k.as_str()),
            ) {
                map.insert(id.to_string(), key.to_string());
            }
        }

        // Return keys in the same order as input UUIDs
        uuids.iter().map(|uuid| {
            map.get(uuid).cloned().ok_or_else(|| {
                Error::NotFound(format!("Could not resolve team key for issue {}", uuid))
            })
        }).collect()
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
        let project_id = self.resolve_project_id_or_uuid(&params.project).await?;
        let mut input = serde_json::json!({
            "title": params.title,
            "projectId": project_id,
        });

        if let Some(ref content) = params.content {
            input["content"] = serde_json::Value::String(content.clone());
        }
        if let Some(ref issue_id) = params.issue {
            let issue_uuid = self.resolve_issue_id(issue_id).await?;
            input["issueId"] = serde_json::Value::String(issue_uuid);
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

    // ---- Initiative handlers ----

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
        } else {
            input["allPublicTeams"] = serde_json::Value::Bool(true);
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

    // ---- Phase 12 handlers ----

    async fn handle_bulk_update_issues(
        &self,
        params: bulk_update_issues::BulkUpdateIssuesParams,
    ) -> Result<String, Error> {
        let id_strs: Vec<&str> = params.ids.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        if id_strs.is_empty() {
            return Err(Error::InvalidInput("No issue IDs provided.".into()));
        }
        if id_strs.len() > 50 {
            return Err(Error::InvalidInput("Maximum 50 issues per batch update.".into()));
        }

        // Resolve all IDs to UUIDs
        let mut uuids = Vec::new();
        for id_str in &id_strs {
            uuids.push(self.resolve_issue_id(id_str).await?);
        }

        let mut input = serde_json::Map::new();

        if params.state.is_some() && params.team.is_some() {
            return Err(Error::InvalidInput(
                "Cannot set both 'state' and 'team' in the same batch update. Change team first, then update state in a separate call.".into()
            ));
        }

        if let Some(ref state) = params.state {
            // States are team-scoped — resolve team keys in a single batch query.
            let team_keys = self.resolve_team_keys_from_issues(&uuids).await?;
            let first_key = &team_keys[0];
            if team_keys.iter().any(|k| k != first_key) {
                return Err(Error::InvalidInput(
                    "Cannot batch-update state across multiple teams. All issues must belong to the same team.".into()
                ));
            }
            let state_id = self.resolve_state_id(state, first_key).await?;
            input.insert("stateId".into(), serde_json::Value::String(state_id));
        }
        if let Some(ref assignee) = params.assignee {
            if assignee.eq_ignore_ascii_case("none") {
                input.insert("assigneeId".into(), serde_json::Value::Null);
            } else {
                let user_id = self.resolve_user_id(assignee).await?;
                input.insert("assigneeId".into(), serde_json::Value::String(user_id));
            }
        }
        if let Some(ref priority) = params.priority {
            let p = match priority.to_lowercase().as_str() {
                "urgent" => 1,
                "high" => 2,
                "normal" | "medium" => 3,
                "low" => 4,
                "none" => 0,
                _ => return Err(Error::InvalidInput(format!("Unknown priority: {}", priority))),
            };
            input.insert("priority".into(), serde_json::json!(p));
        }
        if let Some(ref project) = params.project {
            let project_id = self.resolve_project_id_or_uuid(project).await?;
            input.insert("projectId".into(), serde_json::Value::String(project_id));
        }
        if let Some(ref cycle) = params.cycle {
            input.insert("cycleId".into(), serde_json::Value::String(cycle.clone()));
        }
        if let Some(ref team) = params.team {
            let team_id = self.resolve_team_id(team).await?;
            input.insert("teamId".into(), serde_json::Value::String(team_id));
        }
        // Resolve team key for label scoping when adding/removing labels
        let bulk_team_key = if params.add_labels.is_some() || params.remove_labels.is_some() {
            let team_keys = self.resolve_team_keys_from_issues(&uuids).await?;
            let first_key = &team_keys[0];
            if team_keys.iter().any(|k| k != first_key) {
                return Err(Error::InvalidInput(
                    "Cannot batch-update labels across multiple teams. All issues must belong to the same team.".into()
                ));
            }
            Some(first_key.clone())
        } else {
            None
        };
        if let Some(ref add_labels) = params.add_labels {
            let label_names: Vec<&str> = add_labels.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            let mut label_ids = Vec::new();
            for name in label_names {
                label_ids.push(self.resolve_label_id(name, bulk_team_key.as_deref()).await?);
            }
            input.insert("addedLabelIds".into(), serde_json::json!(label_ids));
        }
        if let Some(ref remove_labels) = params.remove_labels {
            let label_names: Vec<&str> = remove_labels.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            let mut label_ids = Vec::new();
            for name in label_names {
                label_ids.push(self.resolve_label_id(name, bulk_team_key.as_deref()).await?);
            }
            input.insert("removedLabelIds".into(), serde_json::json!(label_ids));
        }

        if input.is_empty() {
            return Err(Error::InvalidInput("No update fields provided.".into()));
        }

        let vars = serde_json::json!({
            "ids": uuids,
            "input": serde_json::Value::Object(input),
        });
        let data: response::BatchUpdateIssuesData = self
            .client
            .execute_json(queries::BATCH_UPDATE_ISSUES, vars)
            .await?;

        if !data.issue_batch_update.success {
            return Err(Error::GraphQL("Batch update failed".into()));
        }

        let issues = &data.issue_batch_update.issues;
        let lines: Vec<String> = issues
            .iter()
            .map(|i| {
                let state = i.state.as_ref().map(|s| s.name.as_str()).unwrap_or("?");
                format!("{} {} [{}]", i.identifier, i.title, state)
            })
            .collect();
        Ok(format!("Updated {} issues:\n\n{}", issues.len(), lines.join("\n")))
    }

    async fn handle_search_documents(
        &self,
        params: search_documents::SearchDocumentsParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(20).max(1).min(100);
        let include_comments = params.include_comments.unwrap_or(false);
        let vars = serde_json::json!({
            "term": params.term,
            "first": limit,
            "includeComments": include_comments,
        });
        let data: response::SearchDocumentsData = self
            .client
            .execute_json(queries::SEARCH_DOCUMENTS, vars)
            .await?;

        let docs = &data.search_documents.nodes;
        if docs.is_empty() {
            return Ok(format!("No documents found for \"{}\".", params.term));
        }

        let total = data.search_documents.total_count.unwrap_or(docs.len() as i64);
        let lines: Vec<String> = docs.iter().map(format::format_document_search_result).collect();
        Ok(format!(
            "Document search for \"{}\" ({} results):\n\n{}",
            params.term, total, lines.join("\n")
        ))
    }

    async fn handle_create_initiative(
        &self,
        params: create_initiative::CreateInitiativeParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({ "name": params.name });

        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(ref status) = params.status {
            input["status"] = serde_json::Value::String(status.clone());
        }
        if let Some(ref owner) = params.owner {
            let owner_id = self.resolve_user_id(owner).await?;
            input["ownerId"] = serde_json::Value::String(owner_id);
        }
        if let Some(ref target_date) = params.target_date {
            input["targetDate"] = serde_json::Value::String(target_date.clone());
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateInitiativeData = self
            .client
            .execute_json(queries::CREATE_INITIATIVE, vars)
            .await?;

        match data.initiative_create.initiative {
            Some(initiative) => Ok(format!(
                "Initiative created:\n\n{}",
                format::format_initiative_detail(&initiative)
            )),
            None => Err(Error::GraphQL("Initiative creation failed".into())),
        }
    }

    async fn handle_update_initiative(
        &self,
        params: update_initiative::UpdateInitiativeParams,
    ) -> Result<String, Error> {
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
        if let Some(ref status) = params.status {
            input.insert("status".into(), serde_json::Value::String(status.clone()));
            has_fields = true;
        }
        if let Some(ref owner) = params.owner {
            if owner.eq_ignore_ascii_case("none") {
                input.insert("ownerId".into(), serde_json::Value::Null);
            } else {
                let owner_id = self.resolve_user_id(owner).await?;
                input.insert("ownerId".into(), serde_json::Value::String(owner_id));
            }
            has_fields = true;
        }
        if let Some(ref target_date) = params.target_date {
            input.insert("targetDate".into(), serde_json::Value::String(target_date.clone()));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput("No update fields provided.".into()));
        }

        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateInitiativeData = self
            .client
            .execute_json(queries::UPDATE_INITIATIVE, vars)
            .await?;

        match data.initiative_update.initiative {
            Some(initiative) => Ok(format!(
                "Initiative updated:\n\n{}",
                format::format_initiative_detail(&initiative)
            )),
            None => Err(Error::GraphQL("Initiative update failed".into())),
        }
    }

    async fn handle_delete_initiative(
        &self,
        params: delete_initiative::DeleteInitiativeParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteInitiativeData = self
            .client
            .execute_json(queries::DELETE_INITIATIVE, vars)
            .await?;

        if data.initiative_delete.success {
            Ok(format!("Initiative {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Initiative deletion failed".into()))
        }
    }

    async fn handle_get_view_issues(
        &self,
        params: get_view_issues::GetViewIssuesParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).max(1).min(100);
        let vars = serde_json::json!({ "id": params.id, "first": limit });
        let data: response::ViewIssuesData = self
            .client
            .execute_json(queries::GET_VIEW_ISSUES, vars)
            .await?;

        let view = data.custom_view.as_ref().ok_or_else(|| {
            Error::NotFound(format!("Custom view '{}' not found or not accessible.", params.id))
        })?;
        let issues = &view.issues.nodes;
        if issues.is_empty() {
            return Ok(format!("No issues in view \"{}\".", view.name));
        }

        let lines: Vec<String> = issues.iter().map(format::format_issue_summary).collect();
        Ok(format!(
            "Issues in \"{}\" ({} results):\n\n{}",
            view.name,
            issues.len(),
            lines.join("\n")
        ))
    }

    async fn handle_list_triage_issues(
        &self,
        params: list_triage_issues::ListTriageIssuesParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(50).max(1).min(100);
        let team_id = self.resolve_team_id(&params.team).await?;
        let filter = serde_json::json!({
            "state": { "type": { "eq": "triage" } },
            "team": { "id": { "eq": team_id } },
        });
        let vars = serde_json::json!({ "first": limit, "filter": filter });
        let data: response::IssuesData = self
            .client
            .execute_json(queries::LIST_TRIAGE_ISSUES, vars)
            .await?;

        let issues = &data.issues.nodes;
        if issues.is_empty() {
            return Ok(format!("No triage issues for team {}.", params.team));
        }

        let lines: Vec<String> = issues.iter().map(format::format_issue_summary).collect();
        Ok(format!(
            "Triage issues for {} ({}):\n\n{}",
            params.team,
            issues.len(),
            lines.join("\n")
        ))
    }

    async fn handle_triage_issue(
        &self,
        params: triage_issue::TriageIssueParams,
    ) -> Result<String, Error> {
        let issue_id = self.resolve_issue_id(&params.id).await?;
        let team_key = self.resolve_team_key_from_issue(&issue_id).await?;
        let state_id = self.resolve_state_id(&params.state, &team_key).await?;

        let mut input = serde_json::json!({ "stateId": state_id });

        if let Some(ref assignee) = params.assignee {
            let user_id = self.resolve_user_id(assignee).await?;
            input["assigneeId"] = serde_json::Value::String(user_id);
        }
        if let Some(ref priority) = params.priority {
            let p = match priority.to_lowercase().as_str() {
                "urgent" => 1,
                "high" => 2,
                "normal" | "medium" => 3,
                "low" => 4,
                "none" => 0,
                _ => return Err(Error::InvalidInput(format!("Unknown priority: {}", priority))),
            };
            input["priority"] = serde_json::json!(p);
        }

        let vars = serde_json::json!({ "id": issue_id, "input": input });
        let data: response::UpdateIssueData = self
            .client
            .execute_json(queries::UPDATE_ISSUE, vars)
            .await?;

        match data.issue_update.issue {
            Some(issue) => {
                let state_name = issue.state.as_ref().map(|s| s.name.as_str()).unwrap_or("?");
                Ok(format!(
                    "Issue {} triaged → {} [{}]",
                    issue.identifier, state_name, issue.title
                ))
            }
            None => Err(Error::GraphQL("Triage update failed".into())),
        }
    }

    async fn handle_create_issue_from_template(
        &self,
        params: create_issue_from_template::CreateIssueFromTemplateParams,
    ) -> Result<String, Error> {
        let team_id = self.resolve_team_id(&params.team).await?;
        let mut input = serde_json::json!({
            "teamId": team_id,
            "templateId": params.template_id,
        });

        if let Some(ref title) = params.title {
            input["title"] = serde_json::Value::String(title.clone());
        }
        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(ref assignee) = params.assignee {
            let user_id = self.resolve_user_id(assignee).await?;
            input["assigneeId"] = serde_json::Value::String(user_id);
        }
        if let Some(ref priority) = params.priority {
            let p = match priority.to_lowercase().as_str() {
                "urgent" => 1,
                "high" => 2,
                "normal" | "medium" => 3,
                "low" => 4,
                "none" => 0,
                _ => return Err(Error::InvalidInput(format!("Unknown priority: {}", priority))),
            };
            input["priority"] = serde_json::json!(p);
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateIssueData = self
            .client
            .execute_json(queries::CREATE_ISSUE, vars)
            .await?;

        match data.issue_create.issue {
            Some(issue) => Ok(format!(
                "Issue created from template:\n\n{}",
                format::format_issue_detail(&issue)
            )),
            None => Err(Error::GraphQL("Issue creation from template failed".into())),
        }
    }

    // ---- New resolvers ----

    /// Resolve a project milestone name to a UUID within a given project.
    async fn resolve_project_milestone_id(&self, name: &str, project_id: &str) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": project_id });
        let data: response::ProjectMilestonesData = self
            .client
            .execute_json(queries::LIST_PROJECT_MILESTONES, vars)
            .await?;

        data.project.project_milestones.nodes.iter()
            .find(|m| m.name.eq_ignore_ascii_case(name))
            .map(|m| m.id.clone())
            .ok_or_else(|| Error::NotFound(format!("Milestone '{}' not found in project", name)))
    }

    /// Resolve an initiative name or UUID to a UUID.
    async fn resolve_initiative_id_or_uuid(&self, id_or_name: &str) -> Result<String, Error> {
        let is_uuid = id_or_name.len() == 36
            && id_or_name.chars().all(|c| c.is_ascii_hexdigit() || c == '-');
        if is_uuid {
            return Ok(id_or_name.to_string());
        }

        let vars = serde_json::json!({ "first": 100 });
        let data: response::InitiativesData = self
            .client
            .execute_json(queries::LIST_INITIATIVES, vars)
            .await?;

        data.initiatives.nodes.iter()
            .find(|i| i.name.eq_ignore_ascii_case(id_or_name))
            .map(|i| i.id.clone())
            .ok_or_else(|| Error::NotFound(format!("Initiative '{}' not found", id_or_name)))
    }

    // ---- Phase 2: Delete/Archive handlers ----

    async fn handle_delete_document(
        &self,
        params: delete_document::DeleteDocumentParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteDocumentData = self
            .client
            .execute_json(queries::DELETE_DOCUMENT, vars)
            .await?;
        if data.document_delete.success {
            Ok(format!("Document {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Document deletion failed".into()))
        }
    }

    async fn handle_delete_project_milestone(
        &self,
        params: delete_project_milestone::DeleteProjectMilestoneParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteProjectMilestoneData = self
            .client
            .execute_json(queries::DELETE_PROJECT_MILESTONE, vars)
            .await?;
        if data.project_milestone_delete.success {
            Ok(format!("Project milestone {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Project milestone deletion failed".into()))
        }
    }

    async fn handle_delete_project_update(
        &self,
        params: delete_project_update::DeleteProjectUpdateParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteProjectUpdateData = self
            .client
            .execute_json(queries::DELETE_PROJECT_UPDATE, vars)
            .await?;
        if data.project_update_delete.success {
            Ok(format!("Project update {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Project update deletion failed".into()))
        }
    }

    async fn handle_delete_attachment(
        &self,
        params: delete_attachment::DeleteAttachmentParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteAttachmentData = self
            .client
            .execute_json(queries::DELETE_ATTACHMENT, vars)
            .await?;
        if data.attachment_delete.success {
            Ok(format!("Attachment {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Attachment deletion failed".into()))
        }
    }

    async fn handle_delete_issue(
        &self,
        params: delete_issue::DeleteIssueParams,
    ) -> Result<String, Error> {
        let uuid = self.resolve_issue_id(&params.id).await?;
        let vars = serde_json::json!({ "id": uuid });
        let data: response::DeleteIssueData = self
            .client
            .execute_json(queries::DELETE_ISSUE, vars)
            .await?;
        if data.issue_delete.success {
            Ok(format!("Issue '{}' permanently deleted.", params.id))
        } else {
            Err(Error::GraphQL("Issue deletion failed".into()))
        }
    }

    async fn handle_delete_view(
        &self,
        params: delete_view::DeleteViewParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteViewData = self
            .client
            .execute_json(queries::DELETE_VIEW, vars)
            .await?;
        if data.custom_view_delete.success {
            Ok(format!("Custom view {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Custom view deletion failed".into()))
        }
    }

    async fn handle_archive_cycle(
        &self,
        params: archive_cycle::ArchiveCycleParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::ArchiveCycleData = self
            .client
            .execute_json(queries::ARCHIVE_CYCLE, vars)
            .await?;
        if data.cycle_archive.success {
            Ok(format!("Cycle {} archived.", params.id))
        } else {
            Err(Error::GraphQL("Cycle archive failed".into()))
        }
    }

    // ---- Phase 3: Update handlers ----

    async fn handle_update_cycle(
        &self,
        params: update_cycle::UpdateCycleParams,
    ) -> Result<String, Error> {
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
        if let Some(ref starts_at) = params.starts_at {
            input.insert("startsAt".into(), serde_json::Value::String(starts_at.clone()));
            has_fields = true;
        }
        if let Some(ref ends_at) = params.ends_at {
            input.insert("endsAt".into(), serde_json::Value::String(ends_at.clone()));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }

        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateCycleData = self
            .client
            .execute_json(queries::UPDATE_CYCLE, vars)
            .await?;

        match data.cycle_update.cycle {
            Some(cycle) => Ok(format!("Cycle updated:\n\n{}", format::format_cycle_created(&cycle))),
            None => Err(Error::GraphQL("Cycle update failed".into())),
        }
    }

    async fn handle_update_project_milestone(
        &self,
        params: update_project_milestone::UpdateProjectMilestoneParams,
    ) -> Result<String, Error> {
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
        if let Some(ref target_date) = params.target_date {
            input.insert("targetDate".into(), serde_json::Value::String(target_date.clone()));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }

        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateProjectMilestoneData = self
            .client
            .execute_json(queries::UPDATE_PROJECT_MILESTONE, vars)
            .await?;

        match data.project_milestone_update.project_milestone {
            Some(milestone) => Ok(format!("Milestone updated:\n\n{}", format::format_project_milestone(&milestone))),
            None => Err(Error::GraphQL("Milestone update failed".into())),
        }
    }

    async fn handle_update_project_update(
        &self,
        params: update_project_update::UpdateProjectUpdateParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;

        if let Some(ref body) = params.body {
            input.insert("body".into(), serde_json::Value::String(body.clone()));
            has_fields = true;
        }
        if let Some(ref health) = params.health {
            input.insert("health".into(), serde_json::Value::String(health.clone()));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }

        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateProjectUpdateData = self
            .client
            .execute_json(queries::UPDATE_PROJECT_UPDATE, vars)
            .await?;

        match data.project_update_update.project_update {
            Some(update) => Ok(format!("Project update updated:\n\n{}", format::format_project_update(&update))),
            None => Err(Error::GraphQL("Project update update failed".into())),
        }
    }

    async fn handle_update_webhook(
        &self,
        params: update_webhook::UpdateWebhookParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;

        if let Some(ref url) = params.url {
            input.insert("url".into(), serde_json::Value::String(url.clone()));
            has_fields = true;
        }
        if let Some(ref label) = params.label {
            input.insert("label".into(), serde_json::Value::String(label.clone()));
            has_fields = true;
        }
        if let Some(enabled) = params.enabled {
            input.insert("enabled".into(), serde_json::Value::Bool(enabled));
            has_fields = true;
        }
        if let Some(ref resource_types) = params.resource_types {
            let types: Vec<&str> = resource_types.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            input.insert("resourceTypes".into(), serde_json::json!(types));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }

        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateWebhookData = self
            .client
            .execute_json(queries::UPDATE_WEBHOOK, vars)
            .await?;

        match data.webhook_update.webhook {
            Some(webhook) => Ok(format!("Webhook updated:\n\n{}", format::format_webhook(&webhook))),
            None => Err(Error::GraphQL("Webhook update failed".into())),
        }
    }

    async fn handle_update_attachment(
        &self,
        params: update_attachment::UpdateAttachmentParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({ "title": params.title });

        if let Some(ref subtitle) = params.subtitle {
            input["subtitle"] = serde_json::Value::String(subtitle.clone());
        }

        let vars = serde_json::json!({ "id": params.id, "input": input });
        let data: response::UpdateAttachmentData = self
            .client
            .execute_json(queries::UPDATE_ATTACHMENT, vars)
            .await?;

        match data.attachment_update.attachment {
            Some(attachment) => Ok(format!("Attachment updated:\n\n{}", format::format_attachment(&attachment))),
            None => Err(Error::GraphQL("Attachment update failed".into())),
        }
    }

    async fn handle_update_view(
        &self,
        params: update_view::UpdateViewParams,
    ) -> Result<String, Error> {
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
        if let Some(ref color) = params.color {
            input.insert("color".into(), serde_json::Value::String(color.clone()));
            has_fields = true;
        }
        if let Some(ref icon) = params.icon {
            input.insert("icon".into(), serde_json::Value::String(icon.clone()));
            has_fields = true;
        }
        if let Some(shared) = params.shared {
            input.insert("shared".into(), serde_json::Value::Bool(shared));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }

        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateViewData = self
            .client
            .execute_json(queries::UPDATE_VIEW, vars)
            .await?;

        match data.custom_view_update.custom_view {
            Some(view) => Ok(format!("View updated:\n\n{}", format::format_custom_view(&view))),
            None => Err(Error::GraphQL("View update failed".into())),
        }
    }

    // ---- Phase 4: Comment handlers ----

    async fn handle_list_comments(
        &self,
        params: list_comments::ListCommentsParams,
    ) -> Result<String, Error> {
        let uuid = self.resolve_issue_id(&params.issue).await?;
        let limit = params.limit.unwrap_or(25).min(100);
        let vars = serde_json::json!({ "id": uuid, "first": limit });
        let data: response::ListCommentsData = self
            .client
            .execute_json(queries::LIST_COMMENTS, vars)
            .await?;

        let comments = &data.issue.comments.nodes;
        if comments.is_empty() {
            return Ok("No comments on this issue.".to_string());
        }

        let lines: Vec<String> = comments.iter().map(format::format_comment_detail).collect();
        Ok(format!("Comments:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_resolve_comment(
        &self,
        params: resolve_comment::ResolveCommentParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::ResolveCommentData = self
            .client
            .execute_json(queries::RESOLVE_COMMENT, vars)
            .await?;

        match data.comment_resolve.comment {
            Some(_) => Ok(format!("Comment {} resolved.", params.id)),
            None => Err(Error::GraphQL("Comment resolve failed".into())),
        }
    }

    async fn handle_unresolve_comment(
        &self,
        params: unresolve_comment::UnresolveCommentParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::UnresolveCommentData = self
            .client
            .execute_json(queries::UNRESOLVE_COMMENT, vars)
            .await?;

        match data.comment_unresolve.comment {
            Some(_) => Ok(format!("Comment {} unresolved.", params.id)),
            None => Err(Error::GraphQL("Comment unresolve failed".into())),
        }
    }

    // ---- Phase 5: Subscription handlers ----

    async fn handle_subscribe_to_issue(
        &self,
        params: subscribe_to_issue::SubscribeToIssueParams,
    ) -> Result<String, Error> {
        let uuid = self.resolve_issue_id(&params.issue).await?;
        let mut vars = serde_json::json!({ "id": uuid });

        if let Some(ref email) = params.user {
            let user_id = self.resolve_user_id(email).await?;
            vars["userId"] = serde_json::Value::String(user_id);
        }

        let data: response::SubscribeToIssueData = self
            .client
            .execute_json(queries::SUBSCRIBE_TO_ISSUE, vars)
            .await?;

        if data.issue_subscribe.success {
            Ok(format!("Subscribed to issue '{}'.", params.issue))
        } else {
            Err(Error::GraphQL("Issue subscribe failed".into()))
        }
    }

    async fn handle_unsubscribe_from_issue(
        &self,
        params: unsubscribe_from_issue::UnsubscribeFromIssueParams,
    ) -> Result<String, Error> {
        let uuid = self.resolve_issue_id(&params.issue).await?;
        let mut vars = serde_json::json!({ "id": uuid });

        if let Some(ref email) = params.user {
            let user_id = self.resolve_user_id(email).await?;
            vars["userId"] = serde_json::Value::String(user_id);
        }

        let data: response::UnsubscribeFromIssueData = self
            .client
            .execute_json(queries::UNSUBSCRIBE_FROM_ISSUE, vars)
            .await?;

        if data.issue_unsubscribe.success {
            Ok(format!("Unsubscribed from issue '{}'.", params.issue))
        } else {
            Err(Error::GraphQL("Issue unsubscribe failed".into()))
        }
    }

    // ---- Phase 6: Create handlers ----

    async fn handle_create_view(
        &self,
        params: create_view::CreateViewParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({ "name": params.name });

        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(ref color) = params.color {
            input["color"] = serde_json::Value::String(color.clone());
        }
        if let Some(ref icon) = params.icon {
            input["icon"] = serde_json::Value::String(icon.clone());
        }
        if let Some(shared) = params.shared {
            input["shared"] = serde_json::Value::Bool(shared);
        }
        if let Some(ref team_key) = params.team {
            let team_id = self.resolve_team_id(team_key).await?;
            input["teamId"] = serde_json::Value::String(team_id);
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateViewData = self
            .client
            .execute_json(queries::CREATE_VIEW, vars)
            .await?;

        match data.custom_view_create.custom_view {
            Some(view) => Ok(format!("View created:\n\n{}", format::format_custom_view(&view))),
            None => Err(Error::GraphQL("View creation failed".into())),
        }
    }

    // ---- Phase 7: Search handlers ----

    async fn handle_search_projects(
        &self,
        params: search_projects::SearchProjectsParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(10).min(50);
        let vars = serde_json::json!({ "term": params.query, "first": limit });
        let data: response::SearchProjectsData = self
            .client
            .execute_json(queries::SEARCH_PROJECTS, vars)
            .await?;

        let results = &data.search_projects.nodes;
        if results.is_empty() {
            return Ok(format!("No projects found for \"{}\".", params.query));
        }

        let lines: Vec<String> = results.iter().map(format::format_project_search_result).collect();
        Ok(format!(
            "Project search for \"{}\":\n\n{}",
            params.query, lines.join("\n\n")
        ))
    }

    async fn handle_issue_vcs_branch_search(
        &self,
        params: issue_vcs_branch_search::IssueVcsBranchSearchParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "branchName": params.branch_name });
        let data: response::IssueVcsBranchSearchData = self
            .client
            .execute_json(queries::ISSUE_VCS_BRANCH_SEARCH, vars)
            .await?;

        match data.issue_vcs_branch_search {
            Some(issue) => Ok(format::format_issue_detail(&issue)),
            None => Ok(format!("No issue found for branch '{}'.", params.branch_name)),
        }
    }

    // ---- Phase 8: Agent Session handlers ----

    async fn handle_create_agent_session(
        &self,
        params: create_agent_session::CreateAgentSessionParams,
    ) -> Result<String, Error> {
        if params.issue.is_none() && params.comment.is_none() {
            return Err(Error::InvalidInput("Provide either 'issue' or 'comment'.".into()));
        }
        if params.issue.is_some() && params.comment.is_some() {
            return Err(Error::InvalidInput("Provide either 'issue' or 'comment', not both.".into()));
        }

        if let Some(ref issue_id) = params.issue {
            let uuid = self.resolve_issue_id(issue_id).await?;
            let mut input = serde_json::json!({ "issueId": uuid });
            if let Some(ref link) = params.external_link {
                input["externalLink"] = serde_json::Value::String(link.clone());
            }
            let vars = serde_json::json!({ "input": input });
            let data: response::AgentSessionCreateOnIssueData = self
                .client
                .execute_json(queries::AGENT_SESSION_CREATE_ON_ISSUE, vars)
                .await?;
            match data.agent_session_create_on_issue.agent_session {
                Some(session) => Ok(format!("Agent session created:\n\n{}", format::format_agent_session_summary(&session))),
                None => Err(Error::GraphQL("Agent session creation failed".into())),
            }
        } else {
            let comment_id = params.comment.as_ref().unwrap();
            let mut input = serde_json::json!({ "commentId": comment_id });
            if let Some(ref link) = params.external_link {
                input["externalLink"] = serde_json::Value::String(link.clone());
            }
            let vars = serde_json::json!({ "input": input });
            let data: response::AgentSessionCreateOnCommentData = self
                .client
                .execute_json(queries::AGENT_SESSION_CREATE_ON_COMMENT, vars)
                .await?;
            match data.agent_session_create_on_comment.agent_session {
                Some(session) => Ok(format!("Agent session created:\n\n{}", format::format_agent_session_summary(&session))),
                None => Err(Error::GraphQL("Agent session creation failed".into())),
            }
        }
    }

    async fn handle_update_agent_session(
        &self,
        params: update_agent_session::UpdateAgentSessionParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;

        if let Some(ref plan) = params.plan {
            let plan_value: serde_json::Value = serde_json::from_str(plan)
                .map_err(|e| Error::InvalidInput(format!("Invalid plan JSON: {}", e)))?;
            input.insert("plan".into(), plan_value);
            has_fields = true;
        }
        if let Some(ref link) = params.external_link {
            input.insert("externalLink".into(), serde_json::Value::String(link.clone()));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }

        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateAgentSessionData = self
            .client
            .execute_json(queries::UPDATE_AGENT_SESSION, vars)
            .await?;

        match data.agent_session_update.agent_session {
            Some(session) => Ok(format!("Agent session updated:\n\n{}", format::format_agent_session_summary(&session))),
            None => Err(Error::GraphQL("Agent session update failed".into())),
        }
    }

    async fn handle_create_agent_activity(
        &self,
        params: create_agent_activity::CreateAgentActivityParams,
    ) -> Result<String, Error> {
        let mut content = serde_json::Map::new();
        if let Some(ref body) = params.body {
            content.insert("body".into(), serde_json::Value::String(body.clone()));
        }
        if let Some(ref action) = params.action {
            content.insert("action".into(), serde_json::Value::String(action.clone()));
        }
        if let Some(ref parameter) = params.parameter {
            content.insert("parameter".into(), serde_json::Value::String(parameter.clone()));
        }
        if let Some(ref result) = params.result {
            content.insert("result".into(), serde_json::Value::String(result.clone()));
        }

        // Store activity type in content so it's never lost
        content.insert(
            "type".into(),
            serde_json::Value::String(params.activity_type.clone()),
        );

        let mut input = serde_json::json!({
            "agentSessionId": params.session,
            "content": serde_json::Value::Object(content),
        });

        // Map to signal enum when the activity type matches a control signal
        match params.activity_type.to_lowercase().as_str() {
            "stop" | "continue" | "auth" | "select" => {
                input["signal"] =
                    serde_json::Value::String(params.activity_type.to_lowercase());
            }
            _ => {} // Non-signal types are carried in content.type
        }

        if let Some(ephemeral) = params.ephemeral {
            input["ephemeral"] = serde_json::Value::Bool(ephemeral);
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateAgentActivityData = self
            .client
            .execute_json(queries::CREATE_AGENT_ACTIVITY, vars)
            .await?;

        if data.agent_activity_create.success {
            Ok("Agent activity logged.".to_string())
        } else {
            Err(Error::GraphQL("Agent activity creation failed".into()))
        }
    }

    async fn handle_list_agent_sessions(
        &self,
        params: list_agent_sessions::ListAgentSessionsParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(25).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::AgentSessionsData = self
            .client
            .execute_json(queries::LIST_AGENT_SESSIONS, vars)
            .await?;

        let sessions = &data.agent_sessions.nodes;
        if sessions.is_empty() {
            return Ok("No agent sessions found.".to_string());
        }

        let lines: Vec<String> = sessions.iter().map(format::format_agent_session_summary).collect();
        Ok(format!("Agent Sessions:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_get_agent_session(
        &self,
        params: get_agent_session::GetAgentSessionParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::AgentSessionData = self
            .client
            .execute_json(queries::GET_AGENT_SESSION, vars)
            .await?;
        Ok(format::format_agent_session_detail(&data.agent_session))
    }

    // ---- Phase 9: Customer handlers ----

    async fn handle_list_customers(
        &self,
        params: list_customers::ListCustomersParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(25).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::CustomersData = self
            .client
            .execute_json(queries::LIST_CUSTOMERS, vars)
            .await?;

        let customers = &data.customers.nodes;
        if customers.is_empty() {
            return Ok("No customers found.".to_string());
        }

        let lines: Vec<String> = customers.iter().map(format::format_customer_summary).collect();
        Ok(format!("Customers:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_get_customer(
        &self,
        params: get_customer::GetCustomerParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::CustomerData = self
            .client
            .execute_json(queries::GET_CUSTOMER, vars)
            .await?;
        Ok(format::format_customer_detail(&data.customer))
    }

    async fn handle_create_customer(
        &self,
        params: create_customer::CreateCustomerParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({ "name": params.name });

        if let Some(ref domains) = params.domains {
            let domain_list: Vec<&str> = domains.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            input["domains"] = serde_json::json!(domain_list);
        }
        if let Some(ref owner) = params.owner {
            let owner_id = self.resolve_user_id(owner).await?;
            input["ownerId"] = serde_json::Value::String(owner_id);
        }
        if let Some(revenue) = params.revenue {
            input["revenue"] = serde_json::json!(revenue);
        }
        if let Some(size) = params.size {
            input["size"] = serde_json::json!(size);
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateCustomerData = self
            .client
            .execute_json(queries::CREATE_CUSTOMER, vars)
            .await?;

        match data.customer_create.customer {
            Some(customer) => Ok(format!("Customer created:\n\n{}", format::format_customer_detail(&customer))),
            None => Err(Error::GraphQL("Customer creation failed".into())),
        }
    }

    async fn handle_update_customer(
        &self,
        params: update_customer::UpdateCustomerParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;

        if let Some(ref name) = params.name {
            input.insert("name".into(), serde_json::Value::String(name.clone()));
            has_fields = true;
        }
        if let Some(ref domains) = params.domains {
            let domain_list: Vec<&str> = domains.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            input.insert("domains".into(), serde_json::json!(domain_list));
            has_fields = true;
        }
        if let Some(ref owner) = params.owner {
            let owner_id = self.resolve_user_id(owner).await?;
            input.insert("ownerId".into(), serde_json::Value::String(owner_id));
            has_fields = true;
        }
        if let Some(revenue) = params.revenue {
            input.insert("revenue".into(), serde_json::json!(revenue));
            has_fields = true;
        }
        if let Some(size) = params.size {
            input.insert("size".into(), serde_json::json!(size));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }

        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateCustomerData = self
            .client
            .execute_json(queries::UPDATE_CUSTOMER, vars)
            .await?;

        match data.customer_update.customer {
            Some(customer) => Ok(format!("Customer updated:\n\n{}", format::format_customer_detail(&customer))),
            None => Err(Error::GraphQL("Customer update failed".into())),
        }
    }

    async fn handle_delete_customer(
        &self,
        params: delete_customer::DeleteCustomerParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteCustomerData = self
            .client
            .execute_json(queries::DELETE_CUSTOMER, vars)
            .await?;
        if data.customer_delete.success {
            Ok(format!("Customer {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Customer deletion failed".into()))
        }
    }

    async fn handle_list_customer_needs(
        &self,
        params: list_customer_needs::ListCustomerNeedsParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(25).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::CustomerNeedsData = self
            .client
            .execute_json(queries::LIST_CUSTOMER_NEEDS, vars)
            .await?;

        let needs = &data.customer_needs.nodes;
        if needs.is_empty() {
            return Ok("No customer needs found.".to_string());
        }

        let lines: Vec<String> = needs.iter().map(format::format_customer_need).collect();
        Ok(format!("Customer Needs:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_create_customer_need(
        &self,
        params: create_customer_need::CreateCustomerNeedParams,
    ) -> Result<String, Error> {
        let issue_uuid = self.resolve_issue_id(&params.issue).await?;
        let mut input = serde_json::json!({
            "issueId": issue_uuid,
            "customerId": params.customer,
        });

        if let Some(ref body) = params.body {
            input["body"] = serde_json::Value::String(body.clone());
        }
        if let Some(priority) = params.priority {
            input["priority"] = serde_json::json!(priority);
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateCustomerNeedData = self
            .client
            .execute_json(queries::CREATE_CUSTOMER_NEED, vars)
            .await?;

        match data.customer_need_create.need {
            Some(need) => Ok(format!("Customer need created:\n\n{}", format::format_customer_need(&need))),
            None => Err(Error::GraphQL("Customer need creation failed".into())),
        }
    }

    async fn handle_update_customer_need(
        &self,
        params: update_customer_need::UpdateCustomerNeedParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;

        if let Some(ref issue) = params.issue {
            let issue_uuid = self.resolve_issue_id(issue).await?;
            input.insert("issueId".into(), serde_json::Value::String(issue_uuid));
            has_fields = true;
        }
        if let Some(ref customer) = params.customer {
            input.insert("customerId".into(), serde_json::Value::String(customer.clone()));
            has_fields = true;
        }
        if let Some(ref body) = params.body {
            input.insert("body".into(), serde_json::Value::String(body.clone()));
            has_fields = true;
        }
        if let Some(priority) = params.priority {
            input.insert("priority".into(), serde_json::json!(priority));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }

        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateCustomerNeedData = self
            .client
            .execute_json(queries::UPDATE_CUSTOMER_NEED, vars)
            .await?;

        match data.customer_need_update.need {
            Some(need) => Ok(format!("Customer need updated:\n\n{}", format::format_customer_need(&need))),
            None => Err(Error::GraphQL("Customer need update failed".into())),
        }
    }

    // ---- Phase 10: Initiative extras handlers ----

    async fn handle_list_initiative_updates(
        &self,
        params: list_initiative_updates::ListInitiativeUpdatesParams,
    ) -> Result<String, Error> {
        let initiative_id = self.resolve_initiative_id_or_uuid(&params.initiative).await?;
        let limit = params.limit.unwrap_or(10).min(50);
        let vars = serde_json::json!({ "id": initiative_id, "first": limit });
        let data: response::InitiativeUpdatesData = self
            .client
            .execute_json(queries::LIST_INITIATIVE_UPDATES, vars)
            .await?;

        let updates = &data.initiative.initiative_updates.nodes;
        if updates.is_empty() {
            return Ok("No initiative updates found.".to_string());
        }

        let lines: Vec<String> = updates.iter().map(format::format_initiative_update).collect();
        Ok(format!("Initiative Updates:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_create_initiative_update(
        &self,
        params: create_initiative_update::CreateInitiativeUpdateParams,
    ) -> Result<String, Error> {
        let initiative_id = self.resolve_initiative_id_or_uuid(&params.initiative).await?;
        let mut input = serde_json::json!({
            "initiativeId": initiative_id,
            "body": params.body,
        });

        if let Some(ref health) = params.health {
            input["health"] = serde_json::Value::String(health.clone());
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateInitiativeUpdateData = self
            .client
            .execute_json(queries::CREATE_INITIATIVE_UPDATE, vars)
            .await?;

        match data.initiative_update_create.initiative_update {
            Some(update) => Ok(format!("Initiative update created:\n\n{}", format::format_initiative_update(&update))),
            None => Err(Error::GraphQL("Initiative update creation failed".into())),
        }
    }

    async fn handle_add_project_to_initiative(
        &self,
        params: add_project_to_initiative::AddProjectToInitiativeParams,
    ) -> Result<String, Error> {
        let initiative_id = self.resolve_initiative_id_or_uuid(&params.initiative).await?;
        let project_id = self.resolve_project_id_or_uuid(&params.project).await?;

        let input = serde_json::json!({
            "initiativeId": initiative_id,
            "projectId": project_id,
        });

        let vars = serde_json::json!({ "input": input });
        let data: response::AddProjectToInitiativeData = self
            .client
            .execute_json(queries::ADD_PROJECT_TO_INITIATIVE, vars)
            .await?;

        match data.initiative_to_project_create.initiative_to_project {
            Some(link) => Ok(format!("Project linked to initiative:\n\n{}", format::format_initiative_to_project(&link))),
            None => Err(Error::GraphQL("Failed to link project to initiative".into())),
        }
    }

    async fn handle_remove_project_from_initiative(
        &self,
        params: remove_project_from_initiative::RemoveProjectFromInitiativeParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::RemoveProjectFromInitiativeData = self
            .client
            .execute_json(queries::REMOVE_PROJECT_FROM_INITIATIVE, vars)
            .await?;

        if data.initiative_to_project_delete.success {
            Ok(format!("Initiative-project link {} removed.", params.id))
        } else {
            Err(Error::GraphQL("Failed to remove initiative-project link".into()))
        }
    }

    // ---- Phase 11: Project relation handlers ----

    async fn handle_create_project_relation(
        &self,
        params: create_project_relation::CreateProjectRelationParams,
    ) -> Result<String, Error> {
        let project_id = self.resolve_project_id_or_uuid(&params.project).await?;
        let related_id = self.resolve_project_id_or_uuid(&params.related_project).await?;

        // Linear project relations use type="dependency" with anchor types:
        //   "start", "end", or "milestone"
        // "blocks" means: project's end blocks related project's start
        // "dependsOn" means: project's start depends on related project's end
        let (anchor, related_anchor) = match params.relation_type.to_lowercase().as_str() {
            "blocks" => ("end", "start"),
            "dependson" | "depends_on" => ("start", "end"),
            "related" => ("end", "end"),
            _ => return Err(Error::InvalidInput(format!("Unknown relation type: {}. Use 'blocks', 'dependsOn', or 'related'.", params.relation_type))),
        };

        let input = serde_json::json!({
            "projectId": project_id,
            "relatedProjectId": related_id,
            "type": "dependency",
            "anchorType": anchor,
            "relatedAnchorType": related_anchor,
        });

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateProjectRelationData = self
            .client
            .execute_json(queries::CREATE_PROJECT_RELATION, vars)
            .await?;

        match data.project_relation_create.project_relation {
            Some(relation) => Ok(format!("Project relation created:\n\n{}", format::format_project_relation(&relation))),
            None => Err(Error::GraphQL("Project relation creation failed".into())),
        }
    }

    async fn handle_delete_project_relation(
        &self,
        params: delete_project_relation::DeleteProjectRelationParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteProjectRelationData = self
            .client
            .execute_json(queries::DELETE_PROJECT_RELATION, vars)
            .await?;

        if data.project_relation_delete.success {
            Ok(format!("Project relation {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Project relation deletion failed".into()))
        }
    }

    async fn handle_list_project_relations(
        &self,
        params: list_project_relations::ListProjectRelationsParams,
    ) -> Result<String, Error> {
        let project_id = self.resolve_project_id_or_uuid(&params.project).await?;
        let vars = serde_json::json!({ "id": project_id });
        let data: response::ProjectRelationsData = self
            .client
            .execute_json(queries::LIST_PROJECT_RELATIONS, vars)
            .await?;

        let relations = &data.project.relations.nodes;
        if relations.is_empty() {
            return Ok("No project relations found.".to_string());
        }

        let lines: Vec<String> = relations.iter().map(format::format_project_relation).collect();
        Ok(format!("Project Relations:\n\n{}", lines.join("\n\n")))
    }

    // ---- Phase 12: Release handlers ----

    async fn handle_list_releases(
        &self,
        params: list_releases::ListReleasesParams,
    ) -> Result<String, Error> {
        let limit = params.limit.unwrap_or(25).min(100);
        let vars = serde_json::json!({ "first": limit });
        let data: response::ReleasesData = self
            .client
            .execute_json(queries::LIST_RELEASES, vars)
            .await?;

        let releases = &data.releases.nodes;
        if releases.is_empty() {
            return Ok("No releases found.".to_string());
        }

        let lines: Vec<String> = releases.iter().map(format::format_release_summary).collect();
        Ok(format!("Releases:\n\n{}", lines.join("\n\n")))
    }

    async fn handle_create_release(
        &self,
        params: create_release::CreateReleaseParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({
            "name": params.name,
            "pipelineId": params.pipeline,
        });

        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(ref version) = params.version {
            input["version"] = serde_json::Value::String(version.clone());
        }
        if let Some(ref sha) = params.commit_sha {
            input["commitSha"] = serde_json::Value::String(sha.clone());
        }
        if let Some(ref start) = params.start_date {
            input["startDate"] = serde_json::Value::String(start.clone());
        }
        if let Some(ref target) = params.target_date {
            input["targetDate"] = serde_json::Value::String(target.clone());
        }

        let vars = serde_json::json!({ "input": input });
        let data: response::CreateReleaseData = self
            .client
            .execute_json(queries::CREATE_RELEASE, vars)
            .await?;

        match data.release_create.release {
            Some(release) => Ok(format!("Release created:\n\n{}", format::format_release_detail(&release))),
            None => Err(Error::GraphQL("Release creation failed".into())),
        }
    }

    async fn handle_update_release(
        &self,
        params: update_release::UpdateReleaseParams,
    ) -> Result<String, Error> {
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
        if let Some(ref version) = params.version {
            input.insert("version".into(), serde_json::Value::String(version.clone()));
            has_fields = true;
        }
        if let Some(ref sha) = params.commit_sha {
            input.insert("commitSha".into(), serde_json::Value::String(sha.clone()));
            has_fields = true;
        }
        if let Some(ref stage) = params.stage {
            input.insert("stageId".into(), serde_json::Value::String(stage.clone()));
            has_fields = true;
        }

        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }

        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateReleaseData = self
            .client
            .execute_json(queries::UPDATE_RELEASE, vars)
            .await?;

        match data.release_update.release {
            Some(release) => Ok(format!("Release updated:\n\n{}", format::format_release_detail(&release))),
            None => Err(Error::GraphQL("Release update failed".into())),
        }
    }

    // ---- Phase 1A: Workflow State CRUD handlers ----

    async fn handle_get_workflow_state(
        &self,
        params: get_workflow_state::GetWorkflowStateParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::WorkflowStateData = self
            .client
            .execute_json(queries::GET_WORKFLOW_STATE, vars)
            .await?;
        Ok(format!("**{}** [{}] color:{} [id: {}]", data.workflow_state.name, data.workflow_state.state_type, data.workflow_state.color, data.workflow_state.id))
    }

    async fn handle_create_workflow_state(
        &self,
        params: create_workflow_state::CreateWorkflowStateParams,
    ) -> Result<String, Error> {
        let team_id = self.resolve_team_id(&params.team).await?;
        let mut input = serde_json::json!({
            "teamId": team_id,
            "name": params.name,
            "color": params.color,
            "type": params.state_type,
        });
        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(pos) = params.position {
            input["position"] = serde_json::json!(pos);
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateWorkflowStateData = self
            .client
            .execute_json(queries::CREATE_WORKFLOW_STATE, vars)
            .await?;
        match data.workflow_state_create.workflow_state {
            Some(state) => Ok(format!("Workflow state created:\n\n{}", format!("**{}** [{}] color:{} [id: {}]", state.name, state.state_type, state.color, state.id))),
            None => Err(Error::GraphQL("Workflow state creation failed".into())),
        }
    }

    async fn handle_update_workflow_state(
        &self,
        params: update_workflow_state::UpdateWorkflowStateParams,
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
        if let Some(ref desc) = params.description {
            input.insert("description".into(), serde_json::Value::String(desc.clone()));
            has_fields = true;
        }
        if let Some(pos) = params.position {
            input.insert("position".into(), serde_json::json!(pos));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateWorkflowStateData = self
            .client
            .execute_json(queries::UPDATE_WORKFLOW_STATE, vars)
            .await?;
        match data.workflow_state_update.workflow_state {
            Some(state) => Ok(format!("Workflow state updated:\n\n{}", format!("**{}** [{}] color:{} [id: {}]", state.name, state.state_type, state.color, state.id))),
            None => Err(Error::GraphQL("Workflow state update failed".into())),
        }
    }

    async fn handle_archive_workflow_state(
        &self,
        params: archive_workflow_state::ArchiveWorkflowStateParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::ArchiveWorkflowStateData = self
            .client
            .execute_json(queries::ARCHIVE_WORKFLOW_STATE, vars)
            .await?;
        if data.workflow_state_archive.success {
            Ok(format!("Workflow state {} archived.", params.id))
        } else {
            Err(Error::GraphQL("Workflow state archive failed".into()))
        }
    }

    // ---- Phase 1B: Issue Extras handlers ----

    /// Resolve the team key for an issue, whether given as identifier ("ENG-123") or UUID.
    async fn resolve_issue_team_key(&self, issue_input: &str, issue_id: &str) -> Result<Option<String>, Error> {
        let is_uuid = issue_input.len() == 36
            && issue_input.chars().all(|c| c.is_ascii_hexdigit() || c == '-');
        if !is_uuid {
            // Extract team key from identifier like "ENG-123"
            let key = issue_input.split('-').next()
                .filter(|k| !k.is_empty() && k.chars().all(|c| c.is_ascii_alphabetic()));
            return Ok(key.map(|k| k.to_string()));
        }
        // UUID input — fetch the issue's team
        let vars = serde_json::json!({ "id": issue_id });
        let data: response::IssueTeamData = self
            .client
            .execute_json(queries::GET_ISSUE_TEAM, vars)
            .await?;
        Ok(Some(data.issue.team.key))
    }

    async fn handle_add_issue_label(
        &self,
        params: add_issue_label::AddIssueLabelParams,
    ) -> Result<String, Error> {
        let issue_id = self.resolve_issue_id(&params.issue).await?;
        let team_key = self.resolve_issue_team_key(&params.issue, &issue_id).await?;
        let label_ids = self.resolve_label_ids(&params.label, team_key.as_deref()).await?;
        let label_id = label_ids.first().ok_or_else(|| Error::NotFound(format!("Label '{}' not found", params.label)))?;
        let vars = serde_json::json!({ "id": issue_id, "labelId": label_id });
        let data: response::IssueAddLabelData = self
            .client
            .execute_json(queries::ISSUE_ADD_LABEL, vars)
            .await?;
        if data.issue_add_label.success {
            Ok(format!("Label '{}' added to issue {}.", params.label, params.issue))
        } else {
            Err(Error::GraphQL("Add label failed".into()))
        }
    }

    async fn handle_remove_issue_label(
        &self,
        params: remove_issue_label::RemoveIssueLabelParams,
    ) -> Result<String, Error> {
        let issue_id = self.resolve_issue_id(&params.issue).await?;
        let team_key = self.resolve_issue_team_key(&params.issue, &issue_id).await?;
        let label_ids = self.resolve_label_ids(&params.label, team_key.as_deref()).await?;
        let label_id = label_ids.first().ok_or_else(|| Error::NotFound(format!("Label '{}' not found", params.label)))?;
        let vars = serde_json::json!({ "id": issue_id, "labelId": label_id });
        let data: response::IssueRemoveLabelData = self
            .client
            .execute_json(queries::ISSUE_REMOVE_LABEL, vars)
            .await?;
        if data.issue_remove_label.success {
            Ok(format!("Label '{}' removed from issue {}.", params.label, params.issue))
        } else {
            Err(Error::GraphQL("Remove label failed".into()))
        }
    }

    async fn handle_batch_create_issues(
        &self,
        params: batch_create_issues::BatchCreateIssuesParams,
    ) -> Result<String, Error> {
        let team_id = self.resolve_team_id(&params.team).await?;
        let issues: Vec<serde_json::Value> = serde_json::from_str(&params.issues)
            .map_err(|e| Error::InvalidInput(format!("Invalid JSON: {}", e)))?;
        let mut inputs = Vec::with_capacity(issues.len());
        for issue in &issues {
            let mut input = issue.as_object()
                .ok_or_else(|| Error::InvalidInput("Each issue must be a JSON object".into()))?
                .clone();
            input.insert("teamId".into(), serde_json::Value::String(team_id.clone()));
            inputs.push(serde_json::Value::Object(input));
        }
        let vars = serde_json::json!({ "input": { "issues": inputs } });
        let data: response::BatchCreateIssuesData = self
            .client
            .execute_json(queries::BATCH_CREATE_ISSUES, vars)
            .await?;
        if data.issue_batch_create.success {
            let count = data.issue_batch_create.issues.len();
            let summaries: Vec<String> = data.issue_batch_create.issues.iter()
                .map(|i| format!("- {} {}", i.identifier, i.title))
                .collect();
            Ok(format!("{} issues created:\n{}", count, summaries.join("\n")))
        } else {
            Err(Error::GraphQL("Batch create issues failed".into()))
        }
    }

    async fn handle_update_issue_relation(
        &self,
        params: update_issue_relation::UpdateIssueRelationParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id, "input": { "type": params.relation_type } });
        let data: response::UpdateIssueRelationData = self
            .client
            .execute_json(queries::UPDATE_ISSUE_RELATION, vars)
            .await?;
        if data.issue_relation_update.success {
            Ok(format!("Issue relation {} updated to type '{}'.", params.id, params.relation_type))
        } else {
            Err(Error::GraphQL("Issue relation update failed".into()))
        }
    }

    async fn handle_get_issue_priority_values(
        &self,
        _params: get_issue_priority_values::GetIssuePriorityValuesParams,
    ) -> Result<String, Error> {
        let data: response::IssuePriorityValuesData = self
            .client
            .execute::<(), _>(queries::GET_ISSUE_PRIORITY_VALUES, None)
            .await?;
        let lines: Vec<String> = data.issue_priority_values.iter()
            .map(|p| format::format_priority_value(p))
            .collect();
        Ok(format!("Priority values:\n{}", lines.join("\n")))
    }

    // ---- Phase 1C: Project Extras handlers ----

    async fn handle_delete_project(
        &self,
        params: delete_project::DeleteProjectParams,
    ) -> Result<String, Error> {
        let project_id = self.resolve_project_id_or_uuid(&params.id).await?;
        let vars = serde_json::json!({ "id": project_id });
        let data: response::DeleteProjectData = self
            .client
            .execute_json(queries::DELETE_PROJECT, vars)
            .await?;
        if data.project_delete.success {
            Ok(format!("Project '{}' permanently deleted.", params.id))
        } else {
            Err(Error::GraphQL("Project deletion failed".into()))
        }
    }

    async fn handle_unarchive_project(
        &self,
        params: unarchive_project::UnarchiveProjectParams,
    ) -> Result<String, Error> {
        let project_id = self.resolve_project_id_or_uuid(&params.id).await?;
        let vars = serde_json::json!({ "id": project_id });
        let data: response::UnarchiveProjectData = self
            .client
            .execute_json(queries::UNARCHIVE_PROJECT, vars)
            .await?;
        if data.project_unarchive.success {
            Ok(format!("Project '{}' unarchived.", params.id))
        } else {
            Err(Error::GraphQL("Project unarchive failed".into()))
        }
    }

    async fn handle_update_project_relation(
        &self,
        params: update_project_relation::UpdateProjectRelationParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;
        if let Some(ref at) = params.anchor_type {
            input.insert("anchorType".into(), serde_json::Value::String(at.clone()));
            has_fields = true;
        }
        if let Some(ref rat) = params.related_anchor_type {
            input.insert("relatedAnchorType".into(), serde_json::Value::String(rat.clone()));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateProjectRelationData = self
            .client
            .execute_json(queries::UPDATE_PROJECT_RELATION, vars)
            .await?;
        if data.project_relation_update.success {
            Ok(format!("Project relation {} updated.", params.id))
        } else {
            Err(Error::GraphQL("Project relation update failed".into()))
        }
    }

    async fn handle_get_project_milestone(
        &self,
        params: get_project_milestone::GetProjectMilestoneParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::GetProjectMilestoneData = self
            .client
            .execute_json(queries::GET_PROJECT_MILESTONE, vars)
            .await?;
        Ok(format::format_project_milestone(&data.project_milestone))
    }

    // ---- Phase 1D: Team Extras handlers ----

    async fn handle_delete_team(
        &self,
        params: delete_team::DeleteTeamParams,
    ) -> Result<String, Error> {
        let team_id = self.resolve_team_id(&params.id).await?;
        let vars = serde_json::json!({ "id": team_id });
        let data: response::DeleteTeamData = self
            .client
            .execute_json(queries::DELETE_TEAM, vars)
            .await?;
        if data.team_delete.success {
            Ok(format!("Team '{}' permanently deleted.", params.id))
        } else {
            Err(Error::GraphQL("Team deletion failed".into()))
        }
    }

    async fn handle_unarchive_team(
        &self,
        params: unarchive_team::UnarchiveTeamParams,
    ) -> Result<String, Error> {
        let team_id = self.resolve_team_id(&params.id).await?;
        let vars = serde_json::json!({ "id": team_id });
        let data: response::UnarchiveTeamData = self
            .client
            .execute_json(queries::UNARCHIVE_TEAM, vars)
            .await?;
        if data.team_unarchive.success {
            Ok(format!("Team '{}' unarchived.", params.id))
        } else {
            Err(Error::GraphQL("Team unarchive failed".into()))
        }
    }

    async fn handle_get_team(
        &self,
        params: get_team::GetTeamParams,
    ) -> Result<String, Error> {
        let team_id = self.resolve_team_id(&params.id).await?;
        let vars = serde_json::json!({ "id": team_id });
        let data: response::GetTeamData = self
            .client
            .execute_json(queries::GET_TEAM, vars)
            .await?;
        let t = &data.team;
        let mut parts = vec![format!("{} | {}", t.key, t.name)];
        let mut meta = Vec::new();
        if let Some(ref desc) = t.description {
            if !desc.is_empty() {
                meta.push(desc.clone());
            }
        }
        if let Some(ref tz) = t.timezone {
            meta.push(format!("timezone: {}", tz));
        }
        if let Some(triage) = t.triage_enabled {
            if triage {
                meta.push("triage enabled".into());
            }
        }
        meta.push(format!("id: {}", t.id));
        if !meta.is_empty() {
            parts.push(meta.join(" | "));
        }
        Ok(parts.join("\n"))
    }

    // ---- Phase 1E: Document Extras handlers ----

    async fn handle_unarchive_document(
        &self,
        params: unarchive_document::UnarchiveDocumentParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::UnarchiveDocumentData = self
            .client
            .execute_json(queries::UNARCHIVE_DOCUMENT, vars)
            .await?;
        if data.document_unarchive.success {
            Ok(format!("Document {} unarchived.", params.id))
        } else {
            Err(Error::GraphQL("Document unarchive failed".into()))
        }
    }

    async fn handle_get_document_content_history(
        &self,
        params: get_document_content_history::GetDocumentContentHistoryParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DocumentContentHistoryData = self
            .client
            .execute_json(queries::GET_DOCUMENT_CONTENT_HISTORY, vars)
            .await?;
        if !data.document_content_history.success {
            return Err(Error::GraphQL("Failed to retrieve document content history.".into()));
        }
        let history = &data.document_content_history.history;
        if history.is_empty() {
            return Ok("No content history found.".into());
        }
        let limit = params.limit.unwrap_or(50).max(1) as usize;
        let lines: Vec<String> = history.iter()
            .take(limit)
            .map(|e| format::format_document_content_history_entry(e))
            .collect();
        Ok(lines.join("\n"))
    }

    // ---- Phase 1F: Misc High-Value handlers ----

    async fn handle_get_viewer_tool(&self) -> Result<String, Error> {
        let viewer = self.get_viewer().await?;
        Ok(format::format_user(&types::User {
            id: viewer.id,
            display_name: viewer.display_name,
            email: viewer.email,
            admin: None,
            guest: None,
            active: None,
        }))
    }

    async fn handle_get_user(
        &self,
        params: get_user::GetUserParams,
    ) -> Result<String, Error> {
        // If it looks like an email, resolve first
        let user_id = if params.id.contains('@') {
            self.resolve_user_id(&params.id).await?
        } else {
            params.id.clone()
        };
        let vars = serde_json::json!({ "id": user_id });
        let data: response::GetUserData = self
            .client
            .execute_json(queries::GET_USER, vars)
            .await?;
        Ok(format::format_user(&data.user))
    }

    async fn handle_update_user(
        &self,
        params: update_user::UpdateUserParams,
    ) -> Result<String, Error> {
        let user_id = if params.id.contains('@') {
            self.resolve_user_id(&params.id).await?
        } else {
            params.id.clone()
        };
        let mut input = serde_json::Map::new();
        let mut has_fields = false;
        if let Some(ref name) = params.display_name {
            input.insert("displayName".into(), serde_json::Value::String(name.clone()));
            has_fields = true;
        }
        if let Some(ref desc) = params.description {
            input.insert("description".into(), serde_json::Value::String(desc.clone()));
            has_fields = true;
        }
        if let Some(ref emoji) = params.status_emoji {
            input.insert("statusEmoji".into(), serde_json::Value::String(emoji.clone()));
            has_fields = true;
        }
        if let Some(ref label) = params.status_label {
            input.insert("statusLabel".into(), serde_json::Value::String(label.clone()));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": user_id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateUserData = self
            .client
            .execute_json(queries::UPDATE_USER, vars)
            .await?;
        match data.user_update.user {
            Some(user) => Ok(format!("User updated:\n\n{}", format::format_user(&user))),
            None => Err(Error::GraphQL("User update failed".into())),
        }
    }

    async fn handle_get_attachment(
        &self,
        params: get_attachment::GetAttachmentParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::GetAttachmentData = self
            .client
            .execute_json(queries::GET_ATTACHMENT, vars)
            .await?;
        Ok(format::format_attachment(&data.attachment))
    }

    async fn handle_get_comment(
        &self,
        params: get_comment::GetCommentParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::GetCommentData = self
            .client
            .execute_json(queries::GET_COMMENT, vars)
            .await?;
        Ok(format::format_comment_detail(&data.comment))
    }

    async fn handle_get_favorite(
        &self,
        params: get_favorite::GetFavoriteParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::GetFavoriteData = self
            .client
            .execute_json(queries::GET_FAVORITE, vars)
            .await?;
        Ok(format::format_favorite(&data.favorite))
    }

    async fn handle_update_favorite(
        &self,
        params: update_favorite::UpdateFavoriteParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;
        if let Some(order) = params.sort_order {
            input.insert("sortOrder".into(), serde_json::json!(order));
            has_fields = true;
        }
        if let Some(ref pid) = params.parent_id {
            input.insert("parentId".into(), serde_json::Value::String(pid.clone()));
            has_fields = true;
        }
        if let Some(ref name) = params.folder_name {
            input.insert("folderName".into(), serde_json::Value::String(name.clone()));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateFavoriteData = self
            .client
            .execute_json(queries::UPDATE_FAVORITE, vars)
            .await?;
        match data.favorite_update.favorite {
            Some(fav) => Ok(format!("Favorite updated:\n\n{}", format::format_favorite(&fav))),
            None => Err(Error::GraphQL("Favorite update failed".into())),
        }
    }

    async fn handle_get_notification(
        &self,
        params: get_notification::GetNotificationParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::GetNotificationData = self
            .client
            .execute_json(queries::GET_NOTIFICATION, vars)
            .await?;
        Ok(format::format_notification(&data.notification))
    }

    // ---- Phase 2A: Customer Status CRUD handlers ----

    async fn handle_list_customer_statuses(
        &self,
        params: list_customer_statuses::ListCustomerStatusesParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::CustomerStatusesData = self
            .client
            .execute_json(queries::LIST_CUSTOMER_STATUSES, vars)
            .await?;
        if data.customer_statuses.nodes.is_empty() {
            return Ok("No customer statuses found.".into());
        }
        let lines: Vec<String> = data.customer_statuses.nodes.iter()
            .map(|s| format::format_customer_status(s))
            .collect();
        Ok(lines.join("\n\n"))
    }

    async fn handle_get_customer_status(
        &self,
        params: get_customer_status::GetCustomerStatusParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::CustomerStatusData = self
            .client
            .execute_json(queries::GET_CUSTOMER_STATUS, vars)
            .await?;
        Ok(format::format_customer_status(&data.customer_status))
    }

    async fn handle_create_customer_status(
        &self,
        params: create_customer_status::CreateCustomerStatusParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({
            "name": params.name,
            "color": params.color,
        });
        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(pos) = params.position {
            input["position"] = serde_json::json!(pos);
        }
        if let Some(ref dn) = params.display_name {
            input["displayName"] = serde_json::Value::String(dn.clone());
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateCustomerStatusData = self
            .client
            .execute_json(queries::CREATE_CUSTOMER_STATUS, vars)
            .await?;
        match data.customer_status_create.status {
            Some(s) => Ok(format!("Customer status created:\n\n{}", format::format_customer_status(&s))),
            None => Err(Error::GraphQL("Customer status creation failed".into())),
        }
    }

    async fn handle_update_customer_status(
        &self,
        params: update_customer_status::UpdateCustomerStatusParams,
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
        if let Some(ref desc) = params.description {
            input.insert("description".into(), serde_json::Value::String(desc.clone()));
            has_fields = true;
        }
        if let Some(pos) = params.position {
            input.insert("position".into(), serde_json::json!(pos));
            has_fields = true;
        }
        if let Some(ref dn) = params.display_name {
            input.insert("displayName".into(), serde_json::Value::String(dn.clone()));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateCustomerStatusData = self
            .client
            .execute_json(queries::UPDATE_CUSTOMER_STATUS, vars)
            .await?;
        match data.customer_status_update.status {
            Some(s) => Ok(format!("Customer status updated:\n\n{}", format::format_customer_status(&s))),
            None => Err(Error::GraphQL("Customer status update failed".into())),
        }
    }

    async fn handle_delete_customer_status(
        &self,
        params: delete_customer_status::DeleteCustomerStatusParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteCustomerStatusData = self
            .client
            .execute_json(queries::DELETE_CUSTOMER_STATUS, vars)
            .await?;
        if data.customer_status_delete.success {
            Ok(format!("Customer status {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Customer status deletion failed".into()))
        }
    }

    // ---- Phase 2B: Customer Tier CRUD handlers ----

    async fn handle_list_customer_tiers(
        &self,
        params: list_customer_tiers::ListCustomerTiersParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::CustomerTiersData = self
            .client
            .execute_json(queries::LIST_CUSTOMER_TIERS, vars)
            .await?;
        if data.customer_tiers.nodes.is_empty() {
            return Ok("No customer tiers found.".into());
        }
        let lines: Vec<String> = data.customer_tiers.nodes.iter()
            .map(|t| format::format_customer_tier(t))
            .collect();
        Ok(lines.join("\n\n"))
    }

    async fn handle_get_customer_tier(
        &self,
        params: get_customer_tier::GetCustomerTierParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::CustomerTierData = self
            .client
            .execute_json(queries::GET_CUSTOMER_TIER, vars)
            .await?;
        Ok(format::format_customer_tier(&data.customer_tier))
    }

    async fn handle_create_customer_tier(
        &self,
        params: create_customer_tier::CreateCustomerTierParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({
            "name": params.name,
            "color": params.color,
        });
        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(pos) = params.position {
            input["position"] = serde_json::json!(pos);
        }
        if let Some(ref dn) = params.display_name {
            input["displayName"] = serde_json::Value::String(dn.clone());
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateCustomerTierData = self
            .client
            .execute_json(queries::CREATE_CUSTOMER_TIER, vars)
            .await?;
        match data.customer_tier_create.tier {
            Some(t) => Ok(format!("Customer tier created:\n\n{}", format::format_customer_tier(&t))),
            None => Err(Error::GraphQL("Customer tier creation failed".into())),
        }
    }

    async fn handle_update_customer_tier(
        &self,
        params: update_customer_tier::UpdateCustomerTierParams,
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
        if let Some(ref desc) = params.description {
            input.insert("description".into(), serde_json::Value::String(desc.clone()));
            has_fields = true;
        }
        if let Some(pos) = params.position {
            input.insert("position".into(), serde_json::json!(pos));
            has_fields = true;
        }
        if let Some(ref dn) = params.display_name {
            input.insert("displayName".into(), serde_json::Value::String(dn.clone()));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateCustomerTierData = self
            .client
            .execute_json(queries::UPDATE_CUSTOMER_TIER, vars)
            .await?;
        match data.customer_tier_update.tier {
            Some(t) => Ok(format!("Customer tier updated:\n\n{}", format::format_customer_tier(&t))),
            None => Err(Error::GraphQL("Customer tier update failed".into())),
        }
    }

    async fn handle_delete_customer_tier(
        &self,
        params: delete_customer_tier::DeleteCustomerTierParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteCustomerTierData = self
            .client
            .execute_json(queries::DELETE_CUSTOMER_TIER, vars)
            .await?;
        if data.customer_tier_delete.success {
            Ok(format!("Customer tier {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Customer tier deletion failed".into()))
        }
    }

    // ---- Phase 2C: Customer Extras handlers ----

    async fn handle_merge_customers(
        &self,
        params: merge_customers::MergeCustomersParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "sourceCustomerId": params.source_id, "targetCustomerId": params.target_id });
        let data: response::MergeCustomersData = self
            .client
            .execute_json(queries::MERGE_CUSTOMERS, vars)
            .await?;
        if data.customer_merge.success {
            Ok(format!("Customer {} merged into {}.", params.source_id, params.target_id))
        } else {
            Err(Error::GraphQL("Customer merge failed".into()))
        }
    }

    async fn handle_get_customer_need(
        &self,
        params: get_customer_need::GetCustomerNeedParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::GetCustomerNeedData = self
            .client
            .execute_json(queries::GET_CUSTOMER_NEED, vars)
            .await?;
        Ok(format::format_customer_need(&data.customer_need))
    }

    async fn handle_archive_customer_need(
        &self,
        params: archive_customer_need::ArchiveCustomerNeedParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::ArchiveCustomerNeedData = self
            .client
            .execute_json(queries::ARCHIVE_CUSTOMER_NEED, vars)
            .await?;
        if data.customer_need_archive.success {
            Ok(format!("Customer need {} archived.", params.id))
        } else {
            Err(Error::GraphQL("Customer need archive failed".into()))
        }
    }

    async fn handle_unarchive_customer_need(
        &self,
        params: unarchive_customer_need::UnarchiveCustomerNeedParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::UnarchiveCustomerNeedData = self
            .client
            .execute_json(queries::UNARCHIVE_CUSTOMER_NEED, vars)
            .await?;
        if data.customer_need_unarchive.success {
            Ok(format!("Customer need {} unarchived.", params.id))
        } else {
            Err(Error::GraphQL("Customer need unarchive failed".into()))
        }
    }

    async fn handle_delete_customer_need(
        &self,
        params: delete_customer_need::DeleteCustomerNeedParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteCustomerNeedData = self
            .client
            .execute_json(queries::DELETE_CUSTOMER_NEED, vars)
            .await?;
        if data.customer_need_delete.success {
            Ok(format!("Customer need {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Customer need deletion failed".into()))
        }
    }

    // ---- Phase 2D: Initiative Extras handlers ----

    async fn handle_archive_initiative(
        &self,
        params: archive_initiative::ArchiveInitiativeParams,
    ) -> Result<String, Error> {
        let initiative_id = self.resolve_initiative_id_or_uuid(&params.id).await?;
        let vars = serde_json::json!({ "id": initiative_id });
        let data: response::ArchiveInitiativeData = self
            .client
            .execute_json(queries::ARCHIVE_INITIATIVE, vars)
            .await?;
        if data.initiative_archive.success {
            Ok(format!("Initiative '{}' archived.", params.id))
        } else {
            Err(Error::GraphQL("Initiative archive failed".into()))
        }
    }

    async fn handle_unarchive_initiative(
        &self,
        params: unarchive_initiative::UnarchiveInitiativeParams,
    ) -> Result<String, Error> {
        let initiative_id = self.resolve_initiative_id_or_uuid(&params.id).await?;
        let vars = serde_json::json!({ "id": initiative_id });
        let data: response::UnarchiveInitiativeData = self
            .client
            .execute_json(queries::UNARCHIVE_INITIATIVE, vars)
            .await?;
        if data.initiative_unarchive.success {
            Ok(format!("Initiative '{}' unarchived.", params.id))
        } else {
            Err(Error::GraphQL("Initiative unarchive failed".into()))
        }
    }

    async fn handle_update_initiative_to_project(
        &self,
        params: update_initiative_to_project::UpdateInitiativeToProjectParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        if let Some(order) = params.sort_order {
            input.insert("sortOrder".into(), serde_json::json!(order));
        }
        if input.is_empty() {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateInitiativeToProjectData = self
            .client
            .execute_json(queries::UPDATE_INITIATIVE_TO_PROJECT, vars)
            .await?;
        if data.initiative_to_project_update.success {
            Ok(format!("Initiative-to-project link {} updated.", params.id))
        } else {
            Err(Error::GraphQL("Initiative-to-project update failed".into()))
        }
    }

    async fn handle_archive_initiative_update(
        &self,
        params: archive_initiative_update::ArchiveInitiativeUpdateParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::ArchiveInitiativeUpdateData = self
            .client
            .execute_json(queries::ARCHIVE_INITIATIVE_UPDATE, vars)
            .await?;
        if data.initiative_update_archive.success {
            Ok(format!("Initiative update {} archived.", params.id))
        } else {
            Err(Error::GraphQL("Initiative update archive failed".into()))
        }
    }

    async fn handle_unarchive_initiative_update(
        &self,
        params: unarchive_initiative_update::UnarchiveInitiativeUpdateParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::UnarchiveInitiativeUpdateData = self
            .client
            .execute_json(queries::UNARCHIVE_INITIATIVE_UPDATE, vars)
            .await?;
        if data.initiative_update_unarchive.success {
            Ok(format!("Initiative update {} unarchived.", params.id))
        } else {
            Err(Error::GraphQL("Initiative update unarchive failed".into()))
        }
    }

    // ---- Phase 3A: Release Extras handlers ----

    async fn handle_get_release(
        &self,
        params: get_release::GetReleaseParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::GetReleaseData = self
            .client
            .execute_json(queries::GET_RELEASE, vars)
            .await?;
        Ok(format::format_release_detail(&data.release))
    }

    async fn handle_archive_release(
        &self,
        params: archive_release::ArchiveReleaseParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::ArchiveReleaseData = self
            .client
            .execute_json(queries::ARCHIVE_RELEASE, vars)
            .await?;
        if data.release_archive.success {
            Ok(format!("Release {} archived.", params.id))
        } else {
            Err(Error::GraphQL("Release archive failed".into()))
        }
    }

    async fn handle_delete_release(
        &self,
        params: delete_release::DeleteReleaseParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteReleaseData = self
            .client
            .execute_json(queries::DELETE_RELEASE, vars)
            .await?;
        if data.release_delete.success {
            Ok(format!("Release {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Release deletion failed".into()))
        }
    }

    async fn handle_unarchive_release(
        &self,
        params: unarchive_release::UnarchiveReleaseParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::UnarchiveReleaseData = self
            .client
            .execute_json(queries::UNARCHIVE_RELEASE, vars)
            .await?;
        if data.release_unarchive.success {
            Ok(format!("Release {} unarchived.", params.id))
        } else {
            Err(Error::GraphQL("Release unarchive failed".into()))
        }
    }

    async fn handle_search_releases(
        &self,
        params: search_releases::SearchReleasesParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(25);
        let vars = serde_json::json!({ "term": params.query, "first": first });
        let data: response::SearchReleasesData = self
            .client
            .execute_json(queries::SEARCH_RELEASES, vars)
            .await?;
        if data.release_search.nodes.is_empty() {
            return Ok("No releases found.".into());
        }
        let lines: Vec<String> = data.release_search.nodes.iter()
            .map(|r| format::format_release_summary(r))
            .collect();
        Ok(lines.join("\n"))
    }

    // ---- Phase 3B: Release Pipeline CRUD handlers ----

    async fn handle_list_release_pipelines(
        &self,
        params: list_release_pipelines::ListReleasePipelinesParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::ReleasePipelinesData = self
            .client
            .execute_json(queries::LIST_RELEASE_PIPELINES, vars)
            .await?;
        if data.release_pipelines.nodes.is_empty() {
            return Ok("No release pipelines found.".into());
        }
        let lines: Vec<String> = data.release_pipelines.nodes.iter()
            .map(|p| format::format_release_pipeline(p))
            .collect();
        Ok(lines.join("\n\n"))
    }

    async fn handle_get_release_pipeline(
        &self,
        params: get_release_pipeline::GetReleasePipelineParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::ReleasePipelineData = self
            .client
            .execute_json(queries::GET_RELEASE_PIPELINE, vars)
            .await?;
        Ok(format::format_release_pipeline(&data.release_pipeline))
    }

    async fn handle_create_release_pipeline(
        &self,
        params: create_release_pipeline::CreateReleasePipelineParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({ "name": params.name });
        if let Some(ref t) = params.pipeline_type {
            input["type"] = serde_json::Value::String(t.clone());
        }
        if let Some(ref patterns) = params.include_path_patterns {
            let patterns: Vec<&str> = patterns.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            input["includePathPatterns"] = serde_json::json!(patterns);
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateReleasePipelineData = self
            .client
            .execute_json(queries::CREATE_RELEASE_PIPELINE, vars)
            .await?;
        match data.release_pipeline_create.release_pipeline {
            Some(p) => Ok(format!("Release pipeline created:\n\n{}", format::format_release_pipeline(&p))),
            None => Err(Error::GraphQL("Release pipeline creation failed".into())),
        }
    }

    async fn handle_update_release_pipeline(
        &self,
        params: update_release_pipeline::UpdateReleasePipelineParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;
        if let Some(ref name) = params.name {
            input.insert("name".into(), serde_json::Value::String(name.clone()));
            has_fields = true;
        }
        if let Some(ref t) = params.pipeline_type {
            input.insert("type".into(), serde_json::Value::String(t.clone()));
            has_fields = true;
        }
        if let Some(ref patterns) = params.include_path_patterns {
            let patterns: Vec<&str> = patterns.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            input.insert("includePathPatterns".into(), serde_json::json!(patterns));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateReleasePipelineData = self
            .client
            .execute_json(queries::UPDATE_RELEASE_PIPELINE, vars)
            .await?;
        match data.release_pipeline_update.release_pipeline {
            Some(p) => Ok(format!("Release pipeline updated:\n\n{}", format::format_release_pipeline(&p))),
            None => Err(Error::GraphQL("Release pipeline update failed".into())),
        }
    }

    async fn handle_delete_release_pipeline(
        &self,
        params: delete_release_pipeline::DeleteReleasePipelineParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteReleasePipelineData = self
            .client
            .execute_json(queries::DELETE_RELEASE_PIPELINE, vars)
            .await?;
        if data.release_pipeline_delete.success {
            Ok(format!("Release pipeline {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Release pipeline deletion failed".into()))
        }
    }

    // ---- Phase 3C: Release Stage CRUD handlers ----

    async fn handle_list_release_stages(
        &self,
        params: list_release_stages::ListReleaseStagesParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::ReleaseStagesData = self
            .client
            .execute_json(queries::LIST_RELEASE_STAGES, vars)
            .await?;
        if data.release_stages.nodes.is_empty() {
            return Ok("No release stages found.".into());
        }
        let lines: Vec<String> = data.release_stages.nodes.iter()
            .map(|s| format::format_release_stage(s))
            .collect();
        Ok(lines.join("\n\n"))
    }

    async fn handle_get_release_stage(
        &self,
        params: get_release_stage::GetReleaseStageParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::ReleaseStageData = self
            .client
            .execute_json(queries::GET_RELEASE_STAGE, vars)
            .await?;
        Ok(format::format_release_stage(&data.release_stage))
    }

    async fn handle_create_release_stage(
        &self,
        params: create_release_stage::CreateReleaseStageParams,
    ) -> Result<String, Error> {
        let input = serde_json::json!({
            "name": params.name,
            "color": params.color,
            "type": params.stage_type,
            "position": params.position,
            "pipelineId": params.pipeline_id,
            "frozen": params.frozen.unwrap_or(false),
        });
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateReleaseStageData = self
            .client
            .execute_json(queries::CREATE_RELEASE_STAGE, vars)
            .await?;
        match data.release_stage_create.release_stage {
            Some(s) => Ok(format!("Release stage created:\n\n{}", format::format_release_stage(&s))),
            None => Err(Error::GraphQL("Release stage creation failed".into())),
        }
    }

    async fn handle_update_release_stage(
        &self,
        params: update_release_stage::UpdateReleaseStageParams,
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
        if let Some(pos) = params.position {
            input.insert("position".into(), serde_json::json!(pos));
            has_fields = true;
        }
        if let Some(frozen) = params.frozen {
            input.insert("frozen".into(), serde_json::json!(frozen));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateReleaseStageData = self
            .client
            .execute_json(queries::UPDATE_RELEASE_STAGE, vars)
            .await?;
        match data.release_stage_update.release_stage {
            Some(s) => Ok(format!("Release stage updated:\n\n{}", format::format_release_stage(&s))),
            None => Err(Error::GraphQL("Release stage update failed".into())),
        }
    }

    // ---- Phase 3D: Issue-to-Release handlers ----

    async fn handle_list_issue_to_releases(
        &self,
        params: list_issue_to_releases::ListIssueToReleasesParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::IssueToReleasesData = self
            .client
            .execute_json(queries::LIST_ISSUE_TO_RELEASES, vars)
            .await?;
        if data.issue_to_releases.nodes.is_empty() {
            return Ok("No issue-to-release links found.".into());
        }
        let lines: Vec<String> = data.issue_to_releases.nodes.iter()
            .map(|l| format::format_issue_to_release(l))
            .collect();
        Ok(lines.join("\n"))
    }

    async fn handle_get_issue_to_release(
        &self,
        params: get_issue_to_release::GetIssueToReleaseParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::IssueToReleaseData = self
            .client
            .execute_json(queries::GET_ISSUE_TO_RELEASE, vars)
            .await?;
        Ok(format::format_issue_to_release(&data.issue_to_release))
    }

    async fn handle_add_issue_to_release(
        &self,
        params: add_issue_to_release::AddIssueToReleaseParams,
    ) -> Result<String, Error> {
        let issue_id = self.resolve_issue_id(&params.issue).await?;
        let vars = serde_json::json!({ "input": { "issueId": issue_id, "releaseId": params.release } });
        let data: response::AddIssueToReleaseData = self
            .client
            .execute_json(queries::ADD_ISSUE_TO_RELEASE, vars)
            .await?;
        if data.issue_to_release_create.success {
            Ok(format!("Issue {} added to release {}.", params.issue, params.release))
        } else {
            Err(Error::GraphQL("Add issue to release failed".into()))
        }
    }

    async fn handle_remove_issue_from_release(
        &self,
        params: remove_issue_from_release::RemoveIssueFromReleaseParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::RemoveIssueFromReleaseData = self
            .client
            .execute_json(queries::REMOVE_ISSUE_FROM_RELEASE, vars)
            .await?;
        if data.issue_to_release_delete.success {
            Ok(format!("Issue-to-release link {} removed.", params.id))
        } else {
            Err(Error::GraphQL("Remove issue from release failed".into()))
        }
    }

    // ---- Phase 4A: Project Status CRUD handlers ----

    async fn handle_list_project_statuses(
        &self,
        params: list_project_statuses::ListProjectStatusesParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::ProjectStatusesData = self
            .client
            .execute_json(queries::LIST_PROJECT_STATUSES, vars)
            .await?;
        if data.project_statuses.nodes.is_empty() {
            return Ok("No project statuses found.".into());
        }
        let lines: Vec<String> = data.project_statuses.nodes.iter()
            .map(|s| format::format_project_status(s))
            .collect();
        Ok(lines.join("\n\n"))
    }

    async fn handle_get_project_status(
        &self,
        params: get_project_status::GetProjectStatusParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::ProjectStatusData = self
            .client
            .execute_json(queries::GET_PROJECT_STATUS, vars)
            .await?;
        Ok(format::format_project_status(&data.project_status))
    }

    async fn handle_create_project_status(
        &self,
        params: create_project_status::CreateProjectStatusParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({
            "name": params.name,
            "color": params.color,
            "type": params.status_type,
            "position": params.position,
        });
        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(indef) = params.indefinite {
            input["indefinite"] = serde_json::json!(indef);
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateProjectStatusData = self
            .client
            .execute_json(queries::CREATE_PROJECT_STATUS, vars)
            .await?;
        match data.project_status_create.status {
            Some(s) => Ok(format!("Project status created:\n\n{}", format::format_project_status(&s))),
            None => Err(Error::GraphQL("Project status creation failed".into())),
        }
    }

    async fn handle_update_project_status(
        &self,
        params: update_project_status::UpdateProjectStatusParams,
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
        if let Some(ref desc) = params.description {
            input.insert("description".into(), serde_json::Value::String(desc.clone()));
            has_fields = true;
        }
        if let Some(pos) = params.position {
            input.insert("position".into(), serde_json::json!(pos));
            has_fields = true;
        }
        if let Some(ref t) = params.status_type {
            input.insert("type".into(), serde_json::Value::String(t.clone()));
            has_fields = true;
        }
        if let Some(indef) = params.indefinite {
            input.insert("indefinite".into(), serde_json::json!(indef));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateProjectStatusData = self
            .client
            .execute_json(queries::UPDATE_PROJECT_STATUS, vars)
            .await?;
        match data.project_status_update.status {
            Some(s) => Ok(format!("Project status updated:\n\n{}", format::format_project_status(&s))),
            None => Err(Error::GraphQL("Project status update failed".into())),
        }
    }

    async fn handle_archive_project_status(
        &self,
        params: archive_project_status::ArchiveProjectStatusParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::ArchiveProjectStatusData = self
            .client
            .execute_json(queries::ARCHIVE_PROJECT_STATUS, vars)
            .await?;
        if data.project_status_archive.success {
            Ok(format!("Project status {} archived.", params.id))
        } else {
            Err(Error::GraphQL("Project status archive failed".into()))
        }
    }

    async fn handle_unarchive_project_status(
        &self,
        params: unarchive_project_status::UnarchiveProjectStatusParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::UnarchiveProjectStatusData = self
            .client
            .execute_json(queries::UNARCHIVE_PROJECT_STATUS, vars)
            .await?;
        if data.project_status_unarchive.success {
            Ok(format!("Project status {} unarchived.", params.id))
        } else {
            Err(Error::GraphQL("Project status unarchive failed".into()))
        }
    }

    // ---- Phase 4B: Project Labels CRUD handlers ----

    async fn handle_list_project_labels(
        &self,
        params: list_project_labels::ListProjectLabelsParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::ProjectLabelsData = self
            .client
            .execute_json(queries::LIST_PROJECT_LABELS, vars)
            .await?;
        if data.project_labels.nodes.is_empty() {
            return Ok("No project labels found.".into());
        }
        let lines: Vec<String> = data.project_labels.nodes.iter()
            .map(|l| format::format_project_label(l))
            .collect();
        Ok(lines.join("\n\n"))
    }

    async fn handle_get_project_label(
        &self,
        params: get_project_label::GetProjectLabelParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::ProjectLabelData = self
            .client
            .execute_json(queries::GET_PROJECT_LABEL, vars)
            .await?;
        Ok(format::format_project_label(&data.project_label))
    }

    async fn handle_create_project_label(
        &self,
        params: create_project_label::CreateProjectLabelParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({ "name": params.name });
        if let Some(ref color) = params.color {
            input["color"] = serde_json::Value::String(color.clone());
        }
        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        if let Some(ref pid) = params.parent_id {
            input["parentId"] = serde_json::Value::String(pid.clone());
        }
        if let Some(is_group) = params.is_group {
            input["isGroup"] = serde_json::json!(is_group);
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateProjectLabelData = self
            .client
            .execute_json(queries::CREATE_PROJECT_LABEL, vars)
            .await?;
        match data.project_label_create.project_label {
            Some(l) => Ok(format!("Project label created:\n\n{}", format::format_project_label(&l))),
            None => Err(Error::GraphQL("Project label creation failed".into())),
        }
    }

    async fn handle_update_project_label(
        &self,
        params: update_project_label::UpdateProjectLabelParams,
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
        if let Some(ref desc) = params.description {
            input.insert("description".into(), serde_json::Value::String(desc.clone()));
            has_fields = true;
        }
        if let Some(ref pid) = params.parent_id {
            input.insert("parentId".into(), serde_json::Value::String(pid.clone()));
            has_fields = true;
        }
        if let Some(is_group) = params.is_group {
            input.insert("isGroup".into(), serde_json::json!(is_group));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateProjectLabelData = self
            .client
            .execute_json(queries::UPDATE_PROJECT_LABEL, vars)
            .await?;
        match data.project_label_update.project_label {
            Some(l) => Ok(format!("Project label updated:\n\n{}", format::format_project_label(&l))),
            None => Err(Error::GraphQL("Project label update failed".into())),
        }
    }

    async fn handle_delete_project_label(
        &self,
        params: delete_project_label::DeleteProjectLabelParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteProjectLabelData = self
            .client
            .execute_json(queries::DELETE_PROJECT_LABEL, vars)
            .await?;
        if data.project_label_delete.success {
            Ok(format!("Project label {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Project label deletion failed".into()))
        }
    }

    // ---- Phase 5A: Team Membership CRUD handlers ----

    async fn handle_list_team_memberships(
        &self,
        params: list_team_memberships::ListTeamMembershipsParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let nodes = if let Some(ref team_filter) = params.team {
            let team_id = self.resolve_team_id(team_filter).await?;
            let vars = serde_json::json!({ "teamId": team_id, "first": first });
            let data: response::TeamMembershipsByTeamData = self
                .client
                .execute_json(queries::LIST_TEAM_MEMBERSHIPS_BY_TEAM, vars)
                .await?;
            data.team.memberships.nodes
        } else {
            let vars = serde_json::json!({ "first": first });
            let data: response::TeamMembershipsData = self
                .client
                .execute_json(queries::LIST_TEAM_MEMBERSHIPS, vars)
                .await?;
            data.team_memberships.nodes
        };
        if nodes.is_empty() {
            return Ok("No team memberships found.".into());
        }
        let lines: Vec<String> = nodes.iter()
            .map(|m| format::format_team_membership(m))
            .collect();
        Ok(lines.join("\n"))
    }

    async fn handle_get_team_membership(
        &self,
        params: get_team_membership::GetTeamMembershipParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::TeamMembershipData = self
            .client
            .execute_json(queries::GET_TEAM_MEMBERSHIP, vars)
            .await?;
        Ok(format::format_team_membership(&data.team_membership))
    }

    async fn handle_create_team_membership(
        &self,
        params: create_team_membership::CreateTeamMembershipParams,
    ) -> Result<String, Error> {
        let user_id = self.resolve_user_id(&params.user).await?;
        let team_id = self.resolve_team_id(&params.team).await?;
        let mut input = serde_json::json!({
            "userId": user_id,
            "teamId": team_id,
        });
        if let Some(owner) = params.owner {
            input["owner"] = serde_json::json!(owner);
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateTeamMembershipData = self
            .client
            .execute_json(queries::CREATE_TEAM_MEMBERSHIP, vars)
            .await?;
        match data.team_membership_create.team_membership {
            Some(m) => Ok(format!("Team membership created:\n\n{}", format::format_team_membership(&m))),
            None => Err(Error::GraphQL("Team membership creation failed".into())),
        }
    }

    async fn handle_update_team_membership(
        &self,
        params: update_team_membership::UpdateTeamMembershipParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        if let Some(owner) = params.owner {
            input.insert("owner".into(), serde_json::json!(owner));
        }
        if input.is_empty() {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateTeamMembershipData = self
            .client
            .execute_json(queries::UPDATE_TEAM_MEMBERSHIP, vars)
            .await?;
        match data.team_membership_update.team_membership {
            Some(m) => Ok(format!("Team membership updated:\n\n{}", format::format_team_membership(&m))),
            None => Err(Error::GraphQL("Team membership update failed".into())),
        }
    }

    async fn handle_delete_team_membership(
        &self,
        params: delete_team_membership::DeleteTeamMembershipParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteTeamMembershipData = self
            .client
            .execute_json(queries::DELETE_TEAM_MEMBERSHIP, vars)
            .await?;
        if data.team_membership_delete.success {
            Ok(format!("Team membership {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Team membership deletion failed".into()))
        }
    }

    // ---- Phase 5B: Notification Subscriptions handlers ----

    async fn handle_list_notification_subscriptions(
        &self,
        params: list_notification_subscriptions::ListNotificationSubscriptionsParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::NotificationSubscriptionsData = self
            .client
            .execute_json(queries::LIST_NOTIFICATION_SUBSCRIPTIONS, vars)
            .await?;
        if data.notification_subscriptions.nodes.is_empty() {
            return Ok("No notification subscriptions found.".into());
        }
        let lines: Vec<String> = data.notification_subscriptions.nodes.iter()
            .map(|s| format::format_notification_subscription(s))
            .collect();
        Ok(lines.join("\n"))
    }

    async fn handle_get_notification_subscription(
        &self,
        params: get_notification_subscription::GetNotificationSubscriptionParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::NotificationSubscriptionData = self
            .client
            .execute_json(queries::GET_NOTIFICATION_SUBSCRIPTION, vars)
            .await?;
        Ok(format::format_notification_subscription(&data.notification_subscription))
    }

    async fn handle_create_notification_subscription(
        &self,
        params: create_notification_subscription::CreateNotificationSubscriptionParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        if let Some(ref types) = params.types {
            let type_list: Vec<&str> = types.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            input.insert("notificationSubscriptionTypes".into(), serde_json::json!(type_list));
        }
        if let Some(ref team) = params.team {
            let team_id = self.resolve_team_id(team).await?;
            input.insert("teamId".into(), serde_json::Value::String(team_id));
        }
        if let Some(ref project) = params.project {
            let project_id = self.resolve_project_id_or_uuid(project).await?;
            input.insert("projectId".into(), serde_json::Value::String(project_id));
        }
        if let Some(ref label) = params.label {
            let is_uuid = label.len() == 36
                && label.chars().all(|c| c.is_ascii_hexdigit() || c == '-');
            let label_id = if is_uuid {
                label.clone()
            } else {
                let label_ids = self.resolve_label_ids(label, params.team.as_deref()).await?;
                label_ids.into_iter().next()
                    .ok_or_else(|| Error::NotFound(format!("Label '{}' not found", label)))?
            };
            input.insert("labelId".into(), serde_json::Value::String(label_id));
        }
        if let Some(active) = params.active {
            input.insert("active".into(), serde_json::json!(active));
        }
        let vars = serde_json::json!({ "input": serde_json::Value::Object(input) });
        let data: response::CreateNotificationSubscriptionData = self
            .client
            .execute_json(queries::CREATE_NOTIFICATION_SUBSCRIPTION, vars)
            .await?;
        match data.notification_subscription_create.notification_subscription {
            Some(s) => Ok(format!("Notification subscription created:\n\n{}", format::format_notification_subscription(&s))),
            None => Err(Error::GraphQL("Notification subscription creation failed".into())),
        }
    }

    async fn handle_update_notification_subscription(
        &self,
        params: update_notification_subscription::UpdateNotificationSubscriptionParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;
        if let Some(ref types) = params.types {
            let type_list: Vec<&str> = types.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            input.insert("notificationSubscriptionTypes".into(), serde_json::json!(type_list));
            has_fields = true;
        }
        if let Some(active) = params.active {
            input.insert("active".into(), serde_json::json!(active));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateNotificationSubscriptionData = self
            .client
            .execute_json(queries::UPDATE_NOTIFICATION_SUBSCRIPTION, vars)
            .await?;
        match data.notification_subscription_update.notification_subscription {
            Some(s) => Ok(format!("Notification subscription updated:\n\n{}", format::format_notification_subscription(&s))),
            None => Err(Error::GraphQL("Notification subscription update failed".into())),
        }
    }

    async fn handle_get_notifications_unread_count(&self) -> Result<String, Error> {
        let data: response::NotificationsUnreadCountData = self
            .client
            .execute::<(), _>(queries::GET_NOTIFICATIONS_UNREAD_COUNT, None)
            .await?;
        Ok(format!("Unread notifications: {}", data.notifications_unread_count))
    }

    // ---- Phase 6A: Template CRUD handlers ----

    async fn handle_get_template(
        &self,
        params: get_template::GetTemplateParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::TemplateData = self
            .client
            .execute_json(queries::GET_TEMPLATE, vars)
            .await?;
        Ok(format::format_template(&data.template))
    }

    async fn handle_create_template(
        &self,
        params: create_template::CreateTemplateParams,
    ) -> Result<String, Error> {
        let template_data: serde_json::Value = serde_json::from_str(&params.template_data)
            .map_err(|e| Error::InvalidInput(format!("Invalid template data JSON: {}", e)))?;
        let mut input = serde_json::json!({
            "name": params.name,
            "type": params.template_type,
            "templateData": template_data,
        });
        if let Some(ref team) = params.team {
            let team_id = self.resolve_team_id(team).await?;
            input["teamId"] = serde_json::Value::String(team_id);
        }
        if let Some(ref desc) = params.description {
            input["description"] = serde_json::Value::String(desc.clone());
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateTemplateData = self
            .client
            .execute_json(queries::CREATE_TEMPLATE, vars)
            .await?;
        match data.template_create.template {
            Some(t) => Ok(format!("Template created:\n\n{}", format::format_template(&t))),
            None => Err(Error::GraphQL("Template creation failed".into())),
        }
    }

    async fn handle_update_template(
        &self,
        params: update_template::UpdateTemplateParams,
    ) -> Result<String, Error> {
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
        if let Some(ref td) = params.template_data {
            let template_data: serde_json::Value = serde_json::from_str(td)
                .map_err(|e| Error::InvalidInput(format!("Invalid template data JSON: {}", e)))?;
            input.insert("templateData".into(), template_data);
            has_fields = true;
        }
        if let Some(ref team) = params.team {
            let team_id = self.resolve_team_id(team).await?;
            input.insert("teamId".into(), serde_json::Value::String(team_id));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateTemplateData = self
            .client
            .execute_json(queries::UPDATE_TEMPLATE, vars)
            .await?;
        match data.template_update.template {
            Some(t) => Ok(format!("Template updated:\n\n{}", format::format_template(&t))),
            None => Err(Error::GraphQL("Template update failed".into())),
        }
    }

    async fn handle_delete_template(
        &self,
        params: delete_template::DeleteTemplateParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteTemplateData = self
            .client
            .execute_json(queries::DELETE_TEMPLATE, vars)
            .await?;
        if data.template_delete.success {
            Ok(format!("Template {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Template deletion failed".into()))
        }
    }

    // ---- Phase 6B: Entity External Links handlers ----

    async fn handle_get_entity_external_link(
        &self,
        params: get_entity_external_link::GetEntityExternalLinkParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::EntityExternalLinkData = self
            .client
            .execute_json(queries::GET_ENTITY_EXTERNAL_LINK, vars)
            .await?;
        Ok(format::format_entity_external_link(&data.entity_external_link))
    }

    async fn handle_create_entity_external_link(
        &self,
        params: create_entity_external_link::CreateEntityExternalLinkParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::json!({
            "url": params.url,
            "label": params.label,
        });
        if let Some(ref initiative) = params.initiative {
            let init_id = self.resolve_initiative_id_or_uuid(initiative).await?;
            input["initiativeId"] = serde_json::Value::String(init_id);
        }
        if let Some(ref project) = params.project {
            let project_id = self.resolve_project_id_or_uuid(project).await?;
            input["projectId"] = serde_json::Value::String(project_id);
        }
        if let Some(ref team) = params.team {
            let team_id = self.resolve_team_id(team).await?;
            input["teamId"] = serde_json::Value::String(team_id);
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateEntityExternalLinkData = self
            .client
            .execute_json(queries::CREATE_ENTITY_EXTERNAL_LINK, vars)
            .await?;
        match data.entity_external_link_create.entity_external_link {
            Some(l) => Ok(format!("Entity external link created:\n\n{}", format::format_entity_external_link(&l))),
            None => Err(Error::GraphQL("Entity external link creation failed".into())),
        }
    }

    async fn handle_update_entity_external_link(
        &self,
        params: update_entity_external_link::UpdateEntityExternalLinkParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;
        if let Some(ref url) = params.url {
            input.insert("url".into(), serde_json::Value::String(url.clone()));
            has_fields = true;
        }
        if let Some(ref label) = params.label {
            input.insert("label".into(), serde_json::Value::String(label.clone()));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateEntityExternalLinkData = self
            .client
            .execute_json(queries::UPDATE_ENTITY_EXTERNAL_LINK, vars)
            .await?;
        match data.entity_external_link_update.entity_external_link {
            Some(l) => Ok(format!("Entity external link updated:\n\n{}", format::format_entity_external_link(&l))),
            None => Err(Error::GraphQL("Entity external link update failed".into())),
        }
    }

    async fn handle_delete_entity_external_link(
        &self,
        params: delete_entity_external_link::DeleteEntityExternalLinkParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteEntityExternalLinkData = self
            .client
            .execute_json(queries::DELETE_ENTITY_EXTERNAL_LINK, vars)
            .await?;
        if data.entity_external_link_delete.success {
            Ok(format!("Entity external link {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Entity external link deletion failed".into()))
        }
    }

    // ---- Phase 6C: Emoji CRUD handlers ----

    async fn handle_list_emojis(
        &self,
        params: list_emojis::ListEmojisParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::EmojisData = self
            .client
            .execute_json(queries::LIST_EMOJIS, vars)
            .await?;
        if data.emojis.nodes.is_empty() {
            return Ok("No custom emojis found.".into());
        }
        let lines: Vec<String> = data.emojis.nodes.iter()
            .map(|e| format::format_emoji(e))
            .collect();
        Ok(lines.join("\n\n"))
    }

    async fn handle_get_emoji(
        &self,
        params: get_emoji::GetEmojiParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::EmojiData = self
            .client
            .execute_json(queries::GET_EMOJI, vars)
            .await?;
        Ok(format::format_emoji(&data.emoji))
    }

    async fn handle_create_emoji(
        &self,
        params: create_emoji::CreateEmojiParams,
    ) -> Result<String, Error> {
        let input = serde_json::json!({
            "name": params.name,
            "url": params.url,
        });
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateEmojiData = self
            .client
            .execute_json(queries::CREATE_EMOJI, vars)
            .await?;
        match data.emoji_create.emoji {
            Some(e) => Ok(format!("Emoji created:\n\n{}", format::format_emoji(&e))),
            None => Err(Error::GraphQL("Emoji creation failed".into())),
        }
    }

    async fn handle_delete_emoji(
        &self,
        params: delete_emoji::DeleteEmojiParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteEmojiData = self
            .client
            .execute_json(queries::DELETE_EMOJI, vars)
            .await?;
        if data.emoji_delete.success {
            Ok(format!("Emoji {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Emoji deletion failed".into()))
        }
    }

    // ---- Phase 6D: Initiative Relations handlers ----

    async fn handle_list_initiative_relations(
        &self,
        params: list_initiative_relations::ListInitiativeRelationsParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::InitiativeRelationsData = self
            .client
            .execute_json(queries::LIST_INITIATIVE_RELATIONS, vars)
            .await?;
        if data.initiative_relations.nodes.is_empty() {
            return Ok("No initiative relations found.".into());
        }
        let lines: Vec<String> = data.initiative_relations.nodes.iter()
            .map(|r| format::format_initiative_relation(r))
            .collect();
        Ok(lines.join("\n"))
    }

    async fn handle_get_initiative_relation(
        &self,
        params: get_initiative_relation::GetInitiativeRelationParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::InitiativeRelationData = self
            .client
            .execute_json(queries::GET_INITIATIVE_RELATION, vars)
            .await?;
        Ok(format::format_initiative_relation(&data.initiative_relation))
    }

    async fn handle_create_initiative_relation(
        &self,
        params: create_initiative_relation::CreateInitiativeRelationParams,
    ) -> Result<String, Error> {
        let init_id = self.resolve_initiative_id_or_uuid(&params.initiative).await?;
        let related_id = self.resolve_initiative_id_or_uuid(&params.related_initiative).await?;
        let input = serde_json::json!({
            "initiativeId": init_id,
            "relatedInitiativeId": related_id,
        });
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateInitiativeRelationData = self
            .client
            .execute_json(queries::CREATE_INITIATIVE_RELATION, vars)
            .await?;
        match data.initiative_relation_create.initiative_relation {
            Some(r) => Ok(format!("Initiative relation created:\n\n{}", format::format_initiative_relation(&r))),
            None => Err(Error::GraphQL("Initiative relation creation failed".into())),
        }
    }

    async fn handle_update_initiative_relation(
        &self,
        params: update_initiative_relation::UpdateInitiativeRelationParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        if let Some(order) = params.sort_order {
            input.insert("sortOrder".into(), serde_json::json!(order));
        }
        if input.is_empty() {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateInitiativeRelationData = self
            .client
            .execute_json(queries::UPDATE_INITIATIVE_RELATION, vars)
            .await?;
        match data.initiative_relation_update.initiative_relation {
            Some(r) => Ok(format!("Initiative relation updated:\n\n{}", format::format_initiative_relation(&r))),
            None => Err(Error::GraphQL("Initiative relation update failed".into())),
        }
    }

    async fn handle_delete_initiative_relation(
        &self,
        params: delete_initiative_relation::DeleteInitiativeRelationParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteInitiativeRelationData = self
            .client
            .execute_json(queries::DELETE_INITIATIVE_RELATION, vars)
            .await?;
        if data.initiative_relation_delete.success {
            Ok(format!("Initiative relation {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Initiative relation deletion failed".into()))
        }
    }

    // ---- Phase 7A: Time Schedule CRUD handlers ----

    async fn handle_list_time_schedules(
        &self,
        params: list_time_schedules::ListTimeSchedulesParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::TimeSchedulesData = self
            .client
            .execute_json(queries::LIST_TIME_SCHEDULES, vars)
            .await?;
        if data.time_schedules.nodes.is_empty() {
            return Ok("No time schedules found.".into());
        }
        let lines: Vec<String> = data.time_schedules.nodes.iter()
            .map(|s| format::format_time_schedule(s))
            .collect();
        Ok(lines.join("\n\n"))
    }

    async fn handle_get_time_schedule(
        &self,
        params: get_time_schedule::GetTimeScheduleParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::TimeScheduleData = self
            .client
            .execute_json(queries::GET_TIME_SCHEDULE, vars)
            .await?;
        Ok(format::format_time_schedule(&data.time_schedule))
    }

    async fn handle_create_time_schedule(
        &self,
        params: create_time_schedule::CreateTimeScheduleParams,
    ) -> Result<String, Error> {
        let entries: serde_json::Value = serde_json::from_str(&params.entries)
            .map_err(|e| Error::InvalidInput(format!("Invalid entries JSON: {}", e)))?;
        let mut input = serde_json::json!({
            "name": params.name,
            "entries": entries,
        });
        if let Some(ref ext_id) = params.external_id {
            input["externalId"] = serde_json::Value::String(ext_id.clone());
        }
        if let Some(ref ext_url) = params.external_url {
            input["externalUrl"] = serde_json::Value::String(ext_url.clone());
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateTimeScheduleData = self
            .client
            .execute_json(queries::CREATE_TIME_SCHEDULE, vars)
            .await?;
        match data.time_schedule_create.time_schedule {
            Some(s) => Ok(format!("Time schedule created:\n\n{}", format::format_time_schedule(&s))),
            None => Err(Error::GraphQL("Time schedule creation failed".into())),
        }
    }

    async fn handle_update_time_schedule(
        &self,
        params: update_time_schedule::UpdateTimeScheduleParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;
        if let Some(ref name) = params.name {
            input.insert("name".into(), serde_json::Value::String(name.clone()));
            has_fields = true;
        }
        if let Some(ref entries) = params.entries {
            let entries_val: serde_json::Value = serde_json::from_str(entries)
                .map_err(|e| Error::InvalidInput(format!("Invalid entries JSON: {}", e)))?;
            input.insert("entries".into(), entries_val);
            has_fields = true;
        }
        if let Some(ref ext_id) = params.external_id {
            input.insert("externalId".into(), serde_json::Value::String(ext_id.clone()));
            has_fields = true;
        }
        if let Some(ref ext_url) = params.external_url {
            input.insert("externalUrl".into(), serde_json::Value::String(ext_url.clone()));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateTimeScheduleData = self
            .client
            .execute_json(queries::UPDATE_TIME_SCHEDULE, vars)
            .await?;
        match data.time_schedule_update.time_schedule {
            Some(s) => Ok(format!("Time schedule updated:\n\n{}", format::format_time_schedule(&s))),
            None => Err(Error::GraphQL("Time schedule update failed".into())),
        }
    }

    async fn handle_delete_time_schedule(
        &self,
        params: delete_time_schedule::DeleteTimeScheduleParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteTimeScheduleData = self
            .client
            .execute_json(queries::DELETE_TIME_SCHEDULE, vars)
            .await?;
        if data.time_schedule_delete.success {
            Ok(format!("Time schedule {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Time schedule deletion failed".into()))
        }
    }

    // ---- Phase 7B: Triage Responsibility CRUD handlers ----

    async fn handle_list_triage_responsibilities(
        &self,
        params: list_triage_responsibilities::ListTriageResponsibilitiesParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::TriageResponsibilitiesData = self
            .client
            .execute_json(queries::LIST_TRIAGE_RESPONSIBILITIES, vars)
            .await?;
        if data.triage_responsibilities.nodes.is_empty() {
            return Ok("No triage responsibilities found.".into());
        }
        let lines: Vec<String> = data.triage_responsibilities.nodes.iter()
            .map(|r| format::format_triage_responsibility(r))
            .collect();
        Ok(lines.join("\n"))
    }

    async fn handle_get_triage_responsibility(
        &self,
        params: get_triage_responsibility::GetTriageResponsibilityParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::TriageResponsibilityData = self
            .client
            .execute_json(queries::GET_TRIAGE_RESPONSIBILITY, vars)
            .await?;
        Ok(format::format_triage_responsibility(&data.triage_responsibility))
    }

    async fn handle_create_triage_responsibility(
        &self,
        params: create_triage_responsibility::CreateTriageResponsibilityParams,
    ) -> Result<String, Error> {
        let team_id = self.resolve_team_id(&params.team).await?;
        let mut input = serde_json::json!({
            "teamId": team_id,
            "action": params.action,
        });
        if let Some(ref ts_id) = params.time_schedule_id {
            input["timeScheduleId"] = serde_json::Value::String(ts_id.clone());
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateTriageResponsibilityData = self
            .client
            .execute_json(queries::CREATE_TRIAGE_RESPONSIBILITY, vars)
            .await?;
        match data.triage_responsibility_create.triage_responsibility {
            Some(r) => Ok(format!("Triage responsibility created:\n\n{}", format::format_triage_responsibility(&r))),
            None => Err(Error::GraphQL("Triage responsibility creation failed".into())),
        }
    }

    async fn handle_update_triage_responsibility(
        &self,
        params: update_triage_responsibility::UpdateTriageResponsibilityParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;
        if let Some(ref action) = params.action {
            input.insert("action".into(), serde_json::Value::String(action.clone()));
            has_fields = true;
        }
        if let Some(ref ts_id) = params.time_schedule_id {
            input.insert("timeScheduleId".into(), serde_json::Value::String(ts_id.clone()));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateTriageResponsibilityData = self
            .client
            .execute_json(queries::UPDATE_TRIAGE_RESPONSIBILITY, vars)
            .await?;
        match data.triage_responsibility_update.triage_responsibility {
            Some(r) => Ok(format!("Triage responsibility updated:\n\n{}", format::format_triage_responsibility(&r))),
            None => Err(Error::GraphQL("Triage responsibility update failed".into())),
        }
    }

    async fn handle_delete_triage_responsibility(
        &self,
        params: delete_triage_responsibility::DeleteTriageResponsibilityParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteTriageResponsibilityData = self
            .client
            .execute_json(queries::DELETE_TRIAGE_RESPONSIBILITY, vars)
            .await?;
        if data.triage_responsibility_delete.success {
            Ok(format!("Triage responsibility {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Triage responsibility deletion failed".into()))
        }
    }

    // ---- Phase 7C: Git Automation handlers ----

    async fn handle_create_git_automation_state(
        &self,
        params: create_git_automation_state::CreateGitAutomationStateParams,
    ) -> Result<String, Error> {
        let team_id = self.resolve_team_id(&params.team).await?;
        let mut input = serde_json::json!({
            "teamId": team_id,
            "event": params.event,
        });
        if let Some(ref state_id) = params.state_id {
            input["stateId"] = serde_json::Value::String(state_id.clone());
        }
        if let Some(ref tb_id) = params.target_branch_id {
            input["targetBranchId"] = serde_json::Value::String(tb_id.clone());
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateGitAutomationStateData = self
            .client
            .execute_json(queries::CREATE_GIT_AUTOMATION_STATE, vars)
            .await?;
        match data.git_automation_state_create.git_automation_state {
            Some(s) => Ok(format!("Git automation state created:\n\n{}", format::format_git_automation_state(&s))),
            None => Err(Error::GraphQL("Git automation state creation failed".into())),
        }
    }

    async fn handle_update_git_automation_state(
        &self,
        params: update_git_automation_state::UpdateGitAutomationStateParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;
        if let Some(ref state_id) = params.state_id {
            input.insert("stateId".into(), serde_json::Value::String(state_id.clone()));
            has_fields = true;
        }
        if let Some(ref tb_id) = params.target_branch_id {
            input.insert("targetBranchId".into(), serde_json::Value::String(tb_id.clone()));
            has_fields = true;
        }
        if let Some(ref event) = params.event {
            input.insert("event".into(), serde_json::Value::String(event.clone()));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateGitAutomationStateData = self
            .client
            .execute_json(queries::UPDATE_GIT_AUTOMATION_STATE, vars)
            .await?;
        match data.git_automation_state_update.git_automation_state {
            Some(s) => Ok(format!("Git automation state updated:\n\n{}", format::format_git_automation_state(&s))),
            None => Err(Error::GraphQL("Git automation state update failed".into())),
        }
    }

    async fn handle_delete_git_automation_state(
        &self,
        params: delete_git_automation_state::DeleteGitAutomationStateParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteGitAutomationStateData = self
            .client
            .execute_json(queries::DELETE_GIT_AUTOMATION_STATE, vars)
            .await?;
        if data.git_automation_state_delete.success {
            Ok(format!("Git automation state {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Git automation state deletion failed".into()))
        }
    }

    async fn handle_create_git_automation_target_branch(
        &self,
        params: create_git_automation_target_branch::CreateGitAutomationTargetBranchParams,
    ) -> Result<String, Error> {
        let team_id = self.resolve_team_id(&params.team).await?;
        let mut input = serde_json::json!({
            "teamId": team_id,
            "branchPattern": params.branch_pattern,
        });
        if let Some(is_regex) = params.is_regex {
            input["isRegex"] = serde_json::json!(is_regex);
        }
        let vars = serde_json::json!({ "input": input });
        let data: response::CreateGitAutomationTargetBranchData = self
            .client
            .execute_json(queries::CREATE_GIT_AUTOMATION_TARGET_BRANCH, vars)
            .await?;
        match data.git_automation_target_branch_create.target_branch {
            Some(b) => Ok(format!("Git automation target branch created:\n\n{}", format::format_git_automation_target_branch(&b))),
            None => Err(Error::GraphQL("Git automation target branch creation failed".into())),
        }
    }

    async fn handle_update_git_automation_target_branch(
        &self,
        params: update_git_automation_target_branch::UpdateGitAutomationTargetBranchParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;
        if let Some(ref pattern) = params.branch_pattern {
            input.insert("branchPattern".into(), serde_json::Value::String(pattern.clone()));
            has_fields = true;
        }
        if let Some(is_regex) = params.is_regex {
            input.insert("isRegex".into(), serde_json::json!(is_regex));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateGitAutomationTargetBranchData = self
            .client
            .execute_json(queries::UPDATE_GIT_AUTOMATION_TARGET_BRANCH, vars)
            .await?;
        match data.git_automation_target_branch_update.target_branch {
            Some(b) => Ok(format!("Git automation target branch updated:\n\n{}", format::format_git_automation_target_branch(&b))),
            None => Err(Error::GraphQL("Git automation target branch update failed".into())),
        }
    }

    async fn handle_delete_git_automation_target_branch(
        &self,
        params: delete_git_automation_target_branch::DeleteGitAutomationTargetBranchParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteGitAutomationTargetBranchData = self
            .client
            .execute_json(queries::DELETE_GIT_AUTOMATION_TARGET_BRANCH, vars)
            .await?;
        if data.git_automation_target_branch_delete.success {
            Ok(format!("Git automation target branch {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Git automation target branch deletion failed".into()))
        }
    }

    // ---- Phase 8A: Email Intake handlers ----

    async fn handle_get_email_intake_address(
        &self,
        params: get_email_intake_address::GetEmailIntakeAddressParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::EmailIntakeAddressData = self
            .client
            .execute_json(queries::GET_EMAIL_INTAKE_ADDRESS, vars)
            .await?;
        Ok(format::format_email_intake_address(&data.email_intake_address))
    }

    async fn handle_create_email_intake_address(
        &self,
        params: create_email_intake_address::CreateEmailIntakeAddressParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        if let Some(ref team) = params.team {
            let team_id = self.resolve_team_id(team).await?;
            input.insert("teamId".into(), serde_json::Value::String(team_id));
        }
        if let Some(ref template) = params.template {
            input.insert("templateId".into(), serde_json::Value::String(template.clone()));
        }
        if let Some(ref name) = params.sender_name {
            input.insert("senderName".into(), serde_json::Value::String(name.clone()));
        }
        if let Some(replies) = params.replies_enabled {
            input.insert("repliesEnabled".into(), serde_json::json!(replies));
        }
        if let Some(customer) = params.customer_requests_enabled {
            input.insert("customerRequestsEnabled".into(), serde_json::json!(customer));
        }
        let vars = serde_json::json!({ "input": serde_json::Value::Object(input) });
        let data: response::CreateEmailIntakeAddressData = self
            .client
            .execute_json(queries::CREATE_EMAIL_INTAKE_ADDRESS, vars)
            .await?;
        match data.email_intake_address_create.email_intake_address {
            Some(a) => Ok(format!("Email intake address created:\n\n{}", format::format_email_intake_address(&a))),
            None => Err(Error::GraphQL("Email intake address creation failed".into())),
        }
    }

    async fn handle_update_email_intake_address(
        &self,
        params: update_email_intake_address::UpdateEmailIntakeAddressParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;
        if let Some(enabled) = params.enabled {
            input.insert("enabled".into(), serde_json::json!(enabled));
            has_fields = true;
        }
        if let Some(ref name) = params.sender_name {
            input.insert("senderName".into(), serde_json::Value::String(name.clone()));
            has_fields = true;
        }
        if let Some(ref team) = params.team {
            let team_id = self.resolve_team_id(team).await?;
            input.insert("teamId".into(), serde_json::Value::String(team_id));
            has_fields = true;
        }
        if let Some(ref template) = params.template {
            input.insert("templateId".into(), serde_json::Value::String(template.clone()));
            has_fields = true;
        }
        if let Some(replies) = params.replies_enabled {
            input.insert("repliesEnabled".into(), serde_json::json!(replies));
            has_fields = true;
        }
        if let Some(customer) = params.customer_requests_enabled {
            input.insert("customerRequestsEnabled".into(), serde_json::json!(customer));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateEmailIntakeAddressData = self
            .client
            .execute_json(queries::UPDATE_EMAIL_INTAKE_ADDRESS, vars)
            .await?;
        match data.email_intake_address_update.email_intake_address {
            Some(a) => Ok(format!("Email intake address updated:\n\n{}", format::format_email_intake_address(&a))),
            None => Err(Error::GraphQL("Email intake address update failed".into())),
        }
    }

    async fn handle_delete_email_intake_address(
        &self,
        params: delete_email_intake_address::DeleteEmailIntakeAddressParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::DeleteEmailIntakeAddressData = self
            .client
            .execute_json(queries::DELETE_EMAIL_INTAKE_ADDRESS, vars)
            .await?;
        if data.email_intake_address_delete.success {
            Ok(format!("Email intake address {} deleted.", params.id))
        } else {
            Err(Error::GraphQL("Email intake address deletion failed".into()))
        }
    }

    // ---- Phase 8B: Remaining Misc handlers ----

    async fn handle_list_archived_teams(
        &self,
        params: list_archived_teams::ListArchivedTeamsParams,
    ) -> Result<String, Error> {
        let data: response::ArchivedTeamsData = self
            .client
            .execute::<(), _>(queries::LIST_ARCHIVED_TEAMS, None)
            .await?;
        if data.archived_teams.is_empty() {
            return Ok("No archived teams found.".into());
        }
        let limit = params.limit.unwrap_or(50).max(1) as usize;
        let lines: Vec<String> = data.archived_teams.iter()
            .take(limit)
            .map(|t| format::format_team_detail(t))
            .collect();
        Ok(lines.join("\n"))
    }

    async fn handle_get_rate_limit_status(&self) -> Result<String, Error> {
        let data: response::RateLimitStatusData = self
            .client
            .execute::<(), _>(queries::GET_RATE_LIMIT_STATUS, None)
            .await?;
        Ok(format::format_rate_limit_status(&data.rate_limit_status))
    }

    async fn handle_get_organization(&self) -> Result<String, Error> {
        let data: response::OrganizationData = self
            .client
            .execute::<(), _>(queries::GET_ORGANIZATION, None)
            .await?;
        Ok(format::format_organization(&data.organization))
    }

    async fn handle_get_application_info(
        &self,
        params: get_application_info::GetApplicationInfoParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "clientId": params.client_id });
        let data: response::ApplicationInfoData = self
            .client
            .execute_json(queries::GET_APPLICATION_INFO, vars)
            .await?;
        Ok(format::format_application_info(&data.application_info))
    }

    async fn handle_semantic_search(
        &self,
        params: semantic_search::SemanticSearchParams,
    ) -> Result<String, Error> {
        let max_results = params.limit.unwrap_or(25);
        let vars = serde_json::json!({ "query": params.query, "maxResults": max_results });
        let data: response::SemanticSearchData = self
            .client
            .execute_json(queries::SEMANTIC_SEARCH, vars)
            .await?;
        if data.semantic_search.results.is_empty() {
            return Ok("No results found.".into());
        }
        let lines: Vec<String> = data.semantic_search.results.iter()
            .map(|r| {
                if let Some(ref issue) = r.issue {
                    format!("[{}] {} {} [id: {}]", r.result_type, issue.identifier, issue.title, issue.id)
                } else if let Some(ref project) = r.project {
                    format!("[{}] {} [id: {}]", r.result_type, project.name, project.id)
                } else if let Some(ref doc) = r.document {
                    format!("[{}] {} [id: {}]", r.result_type, doc.title, doc.id)
                } else {
                    format!("[{}] [id: {}]", r.result_type, r.id)
                }
            })
            .collect();
        Ok(lines.join("\n"))
    }

    async fn handle_attach_link_url(
        &self,
        params: attach_link_url::AttachLinkUrlParams,
    ) -> Result<String, Error> {
        let issue_id = self.resolve_issue_id(&params.issue).await?;
        let mut vars = serde_json::json!({
            "issueId": issue_id,
            "url": params.url,
        });
        if let Some(ref title) = params.title {
            vars["title"] = serde_json::Value::String(title.clone());
        }
        let data: response::AttachLinkUrlData = self
            .client
            .execute_json(queries::ATTACH_LINK_URL, vars)
            .await?;
        if data.attachment_link_url.success {
            Ok(format!("URL attached to issue {}.", params.issue))
        } else {
            Err(Error::GraphQL("Attach link URL failed".into()))
        }
    }

    async fn handle_get_attachments_for_url(
        &self,
        params: get_attachments_for_url::GetAttachmentsForUrlParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "url": params.url });
        let data: response::AttachmentsForUrlData = self
            .client
            .execute_json(queries::GET_ATTACHMENTS_FOR_URL, vars)
            .await?;
        if data.attachments_for_url.nodes.is_empty() {
            return Ok("No attachments found for this URL.".into());
        }
        let lines: Vec<String> = data.attachments_for_url.nodes.iter()
            .map(|a| format::format_attachment(a))
            .collect();
        Ok(lines.join("\n\n"))
    }

    async fn handle_get_issue_filter_suggestion(
        &self,
        params: get_issue_filter_suggestion::GetIssueFilterSuggestionParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "prompt": params.prompt });
        let data: response::IssueFilterSuggestionData = self
            .client
            .execute_json(queries::GET_ISSUE_FILTER_SUGGESTION, vars)
            .await?;
        Ok(format!("Filter: {}", serde_json::to_string_pretty(&data.issue_filter_suggestion.filter).unwrap_or_default()))
    }

    async fn handle_get_project_filter_suggestion(
        &self,
        params: get_project_filter_suggestion::GetProjectFilterSuggestionParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "prompt": params.prompt });
        let data: response::ProjectFilterSuggestionData = self
            .client
            .execute_json(queries::GET_PROJECT_FILTER_SUGGESTION, vars)
            .await?;
        Ok(format!("Filter: {}", serde_json::to_string_pretty(&data.project_filter_suggestion.filter).unwrap_or_default()))
    }

    async fn handle_get_custom_view_suggestion(
        &self,
        params: get_custom_view_suggestion::GetCustomViewSuggestionParams,
    ) -> Result<String, Error> {
        let filter = params.filter.unwrap_or(serde_json::json!({}));
        if !filter.is_object() {
            return Err(Error::InvalidInput("filter must be a JSON object".into()));
        }
        let vars = serde_json::json!({ "modelName": params.model_name, "filter": filter });
        let data: response::CustomViewSuggestionData = self
            .client
            .execute_json(queries::GET_CUSTOM_VIEW_SUGGESTION, vars)
            .await?;
        let s = &data.custom_view_details_suggestion;
        let mut lines = Vec::new();
        if let Some(ref name) = s.name {
            lines.push(format!("Name: {}", name));
        }
        if let Some(ref desc) = s.description {
            lines.push(format!("Description: {}", desc));
        }
        if let Some(ref icon) = s.icon {
            lines.push(format!("Icon: {}", icon));
        }
        Ok(lines.join("\n"))
    }

    async fn handle_check_custom_view_has_subscribers(
        &self,
        params: check_custom_view_has_subscribers::CheckCustomViewHasSubscribersParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::CustomViewHasSubscribersData = self
            .client
            .execute_json(queries::CHECK_CUSTOM_VIEW_HAS_SUBSCRIBERS, vars)
            .await?;
        Ok(format!("Has subscribers: {}", data.custom_view_has_subscribers.has_subscribers))
    }

    async fn handle_search_issue_figma_file_key(
        &self,
        params: search_issue_figma_file_key::SearchIssueFigmaFileKeyParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "fileKey": params.file_key });
        let data: response::SearchIssueFigmaFileKeyData = self
            .client
            .execute_json(queries::SEARCH_ISSUE_FIGMA_FILE_KEY, vars)
            .await?;
        match data.issue_figma_file_key_search {
            Some(issue) => Ok(format::format_issue_summary(&issue)),
            None => Ok("No issues found for this Figma file key.".into()),
        }
    }

    async fn handle_update_initiative_update(
        &self,
        params: update_initiative_update::UpdateInitiativeUpdateParams,
    ) -> Result<String, Error> {
        let mut input = serde_json::Map::new();
        let mut has_fields = false;
        if let Some(ref body) = params.body {
            input.insert("body".into(), serde_json::Value::String(body.clone()));
            has_fields = true;
        }
        if let Some(ref health) = params.health {
            input.insert("health".into(), serde_json::Value::String(health.clone()));
            has_fields = true;
        }
        if !has_fields {
            return Err(Error::InvalidInput("No fields to update.".into()));
        }
        let vars = serde_json::json!({ "id": params.id, "input": serde_json::Value::Object(input) });
        let data: response::UpdateInitiativeUpdateData = self
            .client
            .execute_json(queries::UPDATE_INITIATIVE_UPDATE_MUTATION, vars)
            .await?;
        if data.initiative_update_update.success {
            Ok(format!("Initiative update {} updated.", params.id))
        } else {
            Err(Error::GraphQL("Initiative update update failed".into()))
        }
    }

    async fn handle_list_comments_all(
        &self,
        params: list_comments_all::ListCommentsAllParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::ListCommentsAllData = self
            .client
            .execute_json(queries::LIST_COMMENTS_ALL, vars)
            .await?;
        if data.comments.nodes.is_empty() {
            return Ok("No comments found.".into());
        }
        let lines: Vec<String> = data.comments.nodes.iter()
            .map(|c| format::format_comment_detail(c))
            .collect();
        Ok(lines.join("\n\n"))
    }

    async fn handle_get_issue_label(
        &self,
        params: get_issue_label::GetIssueLabelParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::GetIssueLabelData = self
            .client
            .execute_json(queries::GET_ISSUE_LABEL, vars)
            .await?;
        Ok(format::format_label(&data.issue_label))
    }

    async fn handle_get_issue_relation(
        &self,
        params: get_issue_relation::GetIssueRelationParams,
    ) -> Result<String, Error> {
        let vars = serde_json::json!({ "id": params.id });
        let data: response::GetIssueRelationData = self
            .client
            .execute_json(queries::GET_ISSUE_RELATION, vars)
            .await?;
        Ok(format::format_issue_relation(&data.issue_relation))
    }

    async fn handle_list_issue_relations(
        &self,
        params: list_issue_relations::ListIssueRelationsParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::ListIssueRelationsData = self
            .client
            .execute_json(queries::LIST_ISSUE_RELATIONS, vars)
            .await?;
        if data.issue_relations.nodes.is_empty() {
            return Ok("No issue relations found.".into());
        }
        let lines: Vec<String> = data.issue_relations.nodes.iter()
            .map(|r| format::format_issue_relation(r))
            .collect();
        Ok(lines.join("\n"))
    }

    async fn handle_list_external_users(
        &self,
        params: list_external_users::ListExternalUsersParams,
    ) -> Result<String, Error> {
        let first = params.limit.unwrap_or(50);
        let vars = serde_json::json!({ "first": first });
        let data: response::ExternalUsersData = self
            .client
            .execute_json(queries::LIST_EXTERNAL_USERS, vars)
            .await?;
        if data.external_users.nodes.is_empty() {
            return Ok("No external users found.".into());
        }
        let lines: Vec<String> = data.external_users.nodes.iter()
            .map(|u| format::format_external_user(u))
            .collect();
        Ok(lines.join("\n"))
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
