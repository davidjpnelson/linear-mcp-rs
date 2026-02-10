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
        description = "Add a comment to a Linear issue. Supports markdown."
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
        let pagination =
            format::format_pagination(data.issues.page_info.has_next_page, issues.len());

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
        let pagination = format::format_pagination(
            data.search_issues.page_info.has_next_page,
            issues.len(),
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
        let limit = params.limit.unwrap_or(50).min(100);

        let mut vars = serde_json::json!({ "first": limit });
        if let Some(ref status) = params.status {
            let filter = filters::ProjectFilter {
                state: filters::StringFilter::eq_exact(status.as_str()),
            };
            vars["filter"] = serde_json::to_value(filter).unwrap();
        }

        let data: response::ProjectsData = self
            .client
            .execute_json(queries::LIST_PROJECTS, vars)
            .await?;

        let lines: Vec<String> = data
            .projects
            .nodes
            .iter()
            .map(format::format_project)
            .collect();

        Ok(format!("Projects:\n\n{}", lines.join("\n")))
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

        let filter = filters::IssueFilter::combine(issue_filters);

        let mut vars = serde_json::json!({ "first": limit });
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

        Ok(format!(
            "Issues assigned to {}:\n\n{}",
            viewer.display_name,
            sections.join("\n\n")
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

    // ---- add_comment ----

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

        let vars = serde_json::json!({
            "input": {
                "issueId": uuid,
                "body": params.body,
            }
        });
        let data: response::AddCommentData = self
            .client
            .execute_json(queries::ADD_COMMENT, vars)
            .await?;

        match data.comment_create.comment {
            Some(comment) => {
                let formatted = format::format_comment(&comment);
                Ok(format!("Comment added to {}:\n\n{}", identifier, formatted))
            }
            None => Ok(format!(
                "Comment added to {} but could not fetch details.",
                identifier
            )),
        }
    }
}

fn error_result(err: &Error) -> CallToolResult {
    CallToolResult::error(vec![Content::text(format!("Error: {}", err))])
}
