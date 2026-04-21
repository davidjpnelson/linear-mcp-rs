# linear-mcp-rs

Rust MCP server for Linear. Single ~4MB binary, no runtime dependencies.

## Install

```bash
gh release download --repo Integral-Healthcare/linear-mcp-rs -p install.sh -O- | bash
```

Requires [GitHub CLI](https://cli.github.com). Downloads the right binary for your OS/arch to `~/.local/bin/linear-mcp`.

## Setup

### 1. Linear API key

Create an API key at [https://linear.app](https://linear.app) → **Settings** → **Security & access** → Personal API Keys

The server checks these in order:

1. `LINEAR_API_KEY` environment variable
2. macOS Keychain entry with service name `linear-api-key`

**Option A — env var** (add to `~/.zshrc`):

```bash
export LINEAR_API_KEY="lin_api_your_key_here"
```

**Option B — macOS Keychain**:

```bash
security add-generic-password -s linear-api-key -a "$USER" -w "lin_api_your_key_here"
```

### 2. Add to your coding agent

<details>
<summary><b>Claude Code</b></summary>

```bash
claude mcp add linear-mcp ~/.local/bin/linear-mcp
```

Then restart Claude Code.

</details>

<details>
<summary><b>OpenAI Codex CLI</b></summary>

```bash
codex mcp add linear-mcp -- ~/.local/bin/linear-mcp
```

Or add to `~/.codex/config.toml` (use your full home path):

```toml
[mcp_servers.linear-mcp]
command = "/Users/YOUR_USER/.local/bin/linear-mcp"
```

</details>

<details>
<summary><b>Gemini CLI</b></summary>

```bash
gemini mcp add --name linear-mcp -- ~/.local/bin/linear-mcp
```

Or add to `~/.gemini/settings.json` (use your full home path):

```json
{
  "mcpServers": {
    "linear-mcp": {
      "command": "/Users/YOUR_USER/.local/bin/linear-mcp"
    }
  }
}
```

</details>

<details>
<summary><b>Open Code</b></summary>

Add to `~/.config/opencode/opencode.json` (or `opencode.json` in your project root, use your full home path):

```json
{
  "mcp": {
    "linear-mcp": {
      "type": "local",
      "command": ["/Users/YOUR_USER/.local/bin/linear-mcp"]
    }
  }
}
```

</details>

<details>
<summary><b>Jules</b></summary>

Jules has built-in Linear MCP support — no binary needed. Go to **Jules Settings → MCP**, enter your Linear API key, and start a session.

</details>

## Tools

253 tools with full CRUD coverage of the Linear API. All accept human-friendly inputs (team keys like `ENG`, emails, state names, project names, issue identifiers) and resolve them to IDs automatically.

### Issues

| Tool | Description |
|------|-------------|
| `list_issues` | Filter by team, assignee, status, project, label, priority. Paginated. |
| `search_issues` | Full-text search across titles, descriptions, and comments |
| `semantic_search` | AI-powered semantic search across issues |
| `get_issue` | Full detail by identifier (e.g. `ENG-123`) or UUID. Includes comments, labels, relations. |
| `my_issues` | Your assigned issues grouped by status |
| `create_issue` | Create issue with team key, assignee email, state name, labels, priority |
| `create_issue_from_template` | Create issue from a saved template |
| `batch_create_issues` | Create multiple issues at once in a team |
| `update_issue` | Update any field. Use `"none"` to clear assignee/dueDate. |
| `bulk_update_issues` | Update multiple issues at once (status, priority, assignee, labels) |
| `archive_issue` | Archive an issue |
| `unarchive_issue` | Restore an archived issue |
| `add_issue_label` | Add a label to an issue |
| `remove_issue_label` | Remove a label from an issue |
| `get_issue_priority_values` | List all priority levels and their values |
| `search_issue_figma_file_key` | Find issues linked to a Figma file |
| `get_issue_filter_suggestion` | AI-suggested filters for issue queries |

### Issue Relations

| Tool | Description |
|------|-------------|
| `list_issue_relations` | List relations for an issue |
| `get_issue_relation` | Get a specific issue relation |
| `create_issue_relation` | Create a relation between issues (related, blocks, duplicate) |
| `update_issue_relation` | Update a relation type |
| `delete_issue_relation` | Delete a relation |

### Triage

| Tool | Description |
|------|-------------|
| `list_triage_issues` | List issues in a team's triage state |
| `triage_issue` | Move a triage issue to a target workflow state |
| `list_triage_responsibilities` | List triage responsibility assignments |
| `get_triage_responsibility` | Get a triage responsibility by ID |
| `create_triage_responsibility` | Create a triage responsibility assignment |
| `update_triage_responsibility` | Update a triage responsibility |
| `delete_triage_responsibility` | Delete a triage responsibility |

### Workflow States

| Tool | Description |
|------|-------------|
| `list_states` | Workflow states grouped by team |
| `get_workflow_state` | Get a workflow state by ID |
| `create_workflow_state` | Create a new workflow state for a team |
| `update_workflow_state` | Update a workflow state's name, color, or position |
| `archive_workflow_state` | Archive a workflow state |

### Comments

| Tool | Description |
|------|-------------|
| `list_comments_all` | List comments across all issues |
| `get_comment` | Get a specific comment by ID |
| `add_comment` | Add a markdown comment (supports threaded replies) |
| `update_comment` | Edit an existing comment |
| `delete_comment` | Delete a comment |

### Reactions

| Tool | Description |
|------|-------------|
| `add_reaction` | Add an emoji reaction to a comment |
| `remove_reaction` | Remove a reaction by UUID |

### Labels

| Tool | Description |
|------|-------------|
| `list_labels` | List workspace and team-scoped labels |
| `get_issue_label` | Get a specific label by ID |
| `create_label` | Create a label (workspace or team-scoped, with color) |
| `update_label` | Update a label's name or color |
| `archive_label` | Archive a label |

### Projects

| Tool | Description |
|------|-------------|
| `list_projects` | Projects with status, progress, lead, and teams |
| `get_project` | Full project details |
| `create_project` | Create project with team associations, lead, and dates |
| `update_project` | Update project details |
| `archive_project` | Archive a project |
| `unarchive_project` | Restore an archived project |
| `delete_project` | Permanently delete a project |
| `update_project_relation` | Update a project relation's anchor types |
| `get_project_filter_suggestion` | AI-suggested filters for project queries |

### Project Milestones & Updates

| Tool | Description |
|------|-------------|
| `list_project_milestones` | List milestones for a project |
| `get_project_milestone` | Get a specific project milestone |
| `create_project_milestone` | Create a milestone with target date |
| `list_project_updates` | List status updates for a project |
| `create_project_update` | Post a status update to a project |

### Project Statuses

| Tool | Description |
|------|-------------|
| `list_project_statuses` | List all project statuses |
| `get_project_status` | Get a project status by ID |
| `create_project_status` | Create a new project status |
| `update_project_status` | Update a project status |
| `archive_project_status` | Archive a project status |
| `unarchive_project_status` | Restore an archived project status |

### Project Labels

| Tool | Description |
|------|-------------|
| `list_project_labels` | List project-level labels |
| `get_project_label` | Get a project label by ID |
| `create_project_label` | Create a project label |
| `update_project_label` | Update a project label |
| `delete_project_label` | Delete a project label |

### Cycles

| Tool | Description |
|------|-------------|
| `list_cycles` | List cycles for a team |
| `get_cycle` | Get cycle details |
| `create_cycle` | Create a new cycle with start/end dates |
| `add_issue_to_cycle` | Add an issue to a cycle |
| `remove_issue_from_cycle` | Remove an issue from a cycle |

### Initiatives

| Tool | Description |
|------|-------------|
| `list_initiatives` | List all workspace initiatives |
| `create_initiative` | Create an initiative with status, owner, target date |
| `update_initiative` | Update an initiative |
| `archive_initiative` | Archive an initiative |
| `unarchive_initiative` | Restore an archived initiative |
| `delete_initiative` | Permanently delete an initiative |
| `update_initiative_to_project` | Update an initiative-to-project link |
| `archive_initiative_update` | Archive an initiative update |
| `unarchive_initiative_update` | Restore an archived initiative update |
| `update_initiative_update` | Update an initiative update |

### Initiative Relations

| Tool | Description |
|------|-------------|
| `list_initiative_relations` | List initiative relations |
| `get_initiative_relation` | Get an initiative relation by ID |
| `create_initiative_relation` | Create an initiative relation |
| `update_initiative_relation` | Update an initiative relation |
| `delete_initiative_relation` | Delete an initiative relation |

### Customers

| Tool | Description |
|------|-------------|
| `get_customer_need` | Get a customer need by ID |
| `archive_customer_need` | Archive a customer need |
| `unarchive_customer_need` | Restore an archived customer need |
| `delete_customer_need` | Delete a customer need |
| `merge_customers` | Merge two customer records |

### Customer Statuses

| Tool | Description |
|------|-------------|
| `list_customer_statuses` | List all customer statuses |
| `get_customer_status` | Get a customer status by ID |
| `create_customer_status` | Create a customer status |
| `update_customer_status` | Update a customer status |
| `delete_customer_status` | Delete a customer status |

### Customer Tiers

| Tool | Description |
|------|-------------|
| `list_customer_tiers` | List all customer tiers |
| `get_customer_tier` | Get a customer tier by ID |
| `create_customer_tier` | Create a customer tier |
| `update_customer_tier` | Update a customer tier |
| `delete_customer_tier` | Delete a customer tier |

### Releases

| Tool | Description |
|------|-------------|
| `get_release` | Get a release by ID |
| `search_releases` | Search releases by query string |
| `archive_release` | Archive a release |
| `unarchive_release` | Restore an archived release |
| `delete_release` | Delete a release |

### Release Pipelines

| Tool | Description |
|------|-------------|
| `list_release_pipelines` | List release pipelines |
| `get_release_pipeline` | Get a release pipeline by ID |
| `create_release_pipeline` | Create a release pipeline |
| `update_release_pipeline` | Update a release pipeline |
| `delete_release_pipeline` | Delete a release pipeline |

### Release Stages

| Tool | Description |
|------|-------------|
| `list_release_stages` | List release stages |
| `get_release_stage` | Get a release stage by ID |
| `create_release_stage` | Create a release stage |
| `update_release_stage` | Update a release stage |

### Issue-to-Release

| Tool | Description |
|------|-------------|
| `list_issue_to_releases` | List issue-to-release associations |
| `get_issue_to_release` | Get an issue-to-release link by ID |
| `add_issue_to_release` | Add an issue to a release |
| `remove_issue_from_release` | Remove an issue from a release |

### Documents

| Tool | Description |
|------|-------------|
| `list_documents` | List documents, optionally filtered by project |
| `get_document` | Get full document content |
| `get_document_content_history` | Get content revision history for a document |
| `search_documents` | Full-text search across documents |
| `create_document` | Create a document, optionally linked to a project |
| `update_document` | Update document title or content |
| `unarchive_document` | Restore an archived document |

### Teams

| Tool | Description |
|------|-------------|
| `list_teams` | All teams with keys |
| `get_team` | Get team details by key or UUID |
| `create_team` | Create a new team |
| `update_team` | Update team name, description, or timezone |
| `delete_team` | Delete a team |
| `unarchive_team` | Restore an archived team |
| `list_archived_teams` | List archived teams |

### Team Memberships

| Tool | Description |
|------|-------------|
| `list_team_memberships` | List team memberships, optionally filtered by team |
| `get_team_membership` | Get a team membership by ID |
| `create_team_membership` | Add a user to a team |
| `update_team_membership` | Update membership (e.g. owner flag) |
| `delete_team_membership` | Remove a user from a team |

### Users

| Tool | Description |
|------|-------------|
| `list_users` | Workspace members with roles |
| `get_user` | Get a user by email or UUID |
| `get_viewer` | Get the authenticated user's info |
| `update_user` | Update user display name, status, or description |
| `list_external_users` | List external (guest) users |

### Notifications

| Tool | Description |
|------|-------------|
| `list_notifications` | Your recent notifications |
| `get_notification` | Get a specific notification by ID |
| `get_notifications_unread_count` | Get count of unread notifications |
| `mark_notification_read` | Mark a notification as read |

### Notification Subscriptions

| Tool | Description |
|------|-------------|
| `list_notification_subscriptions` | List notification subscriptions |
| `get_notification_subscription` | Get a subscription by ID |
| `create_notification_subscription` | Create a notification subscription (team, project, or label scoped) |
| `update_notification_subscription` | Update a subscription |

### Views

| Tool | Description |
|------|-------------|
| `list_views` | List saved custom views |
| `get_view_issues` | Get issues matching a custom view's filters |
| `get_custom_view_suggestion` | AI-suggested custom view from a prompt |
| `check_custom_view_has_subscribers` | Check if a custom view has subscribers |

### Attachments

| Tool | Description |
|------|-------------|
| `list_attachments` | List attachments on an issue |
| `get_attachment` | Get an attachment by ID |
| `add_attachment` | Add a URL attachment to an issue |
| `attach_link_url` | Attach a URL link to an issue |
| `get_attachments_for_url` | Find attachments matching a URL |

### Favorites

| Tool | Description |
|------|-------------|
| `list_favorites` | List your favorited items |
| `get_favorite` | Get a favorite by ID |
| `add_favorite` | Favorite an issue or project |
| `update_favorite` | Update a favorite's sort order or folder |
| `remove_favorite` | Remove a favorite by UUID |

### Templates

| Tool | Description |
|------|-------------|
| `list_templates` | List issue templates |
| `get_template` | Get a template by ID |
| `create_template` | Create a new template |
| `update_template` | Update a template |
| `delete_template` | Delete a template |

### Entity External Links

| Tool | Description |
|------|-------------|
| `get_entity_external_link` | Get an external link by ID |
| `create_entity_external_link` | Create an external link on an entity |
| `update_entity_external_link` | Update an external link |
| `delete_entity_external_link` | Delete an external link |

### Emojis

| Tool | Description |
|------|-------------|
| `list_emojis` | List custom emojis |
| `get_emoji` | Get an emoji by ID |
| `create_emoji` | Create a custom emoji |
| `delete_emoji` | Delete a custom emoji |

### Time Schedules

| Tool | Description |
|------|-------------|
| `list_time_schedules` | List time schedules |
| `get_time_schedule` | Get a time schedule by ID |
| `create_time_schedule` | Create a time schedule |
| `update_time_schedule` | Update a time schedule |
| `delete_time_schedule` | Delete a time schedule |

### Git Automation

| Tool | Description |
|------|-------------|
| `create_git_automation_state` | Create a git automation state mapping |
| `update_git_automation_state` | Update a git automation state mapping |
| `delete_git_automation_state` | Delete a git automation state mapping |
| `create_git_automation_target_branch` | Create a git automation target branch |
| `update_git_automation_target_branch` | Update a git automation target branch |
| `delete_git_automation_target_branch` | Delete a git automation target branch |

### Email Intake

| Tool | Description |
|------|-------------|
| `get_email_intake_address` | Get an email intake address by ID |
| `create_email_intake_address` | Create an email intake address for a team |
| `update_email_intake_address` | Update an email intake address |
| `delete_email_intake_address` | Delete an email intake address |

### History & Audit

| Tool | Description |
|------|-------------|
| `get_issue_history` | Audit trail of changes for an issue |
| `query_audit_log` | Workspace audit log (admin only) |

### Integrations & Webhooks

| Tool | Description |
|------|-------------|
| `list_integrations` | List workspace integrations |
| `list_webhooks` | List webhooks (admin only) |
| `create_webhook` | Create a webhook (admin only) |
| `delete_webhook` | Delete a webhook (admin only) |

### Organization & System

| Tool | Description |
|------|-------------|
| `get_organization` | Get workspace organization info |
| `get_application_info` | Get API application info |
| `get_rate_limit_status` | Get current API rate limit status |
| `list_roadmaps` | List roadmaps |

## Architecture

- **MCP SDK**: [rmcp](https://crates.io/crates/rmcp) v0.15 with stdio transport
- **Linear API**: Raw GraphQL via `reqwest` (no Linear SDK)
- **TLS**: rustls (no OpenSSL dependency)
- **Caching**: In-memory cache for entity resolution (team key → ID, issue identifier → UUID)
- **Inline enrichment**: Related entities (state, assignee, labels) fetched inline in queries, no N+1

## Build from source

Requires [Rust toolchain](https://rustup.rs).

```bash
git clone git@github.com:Integral-Healthcare/linear-mcp-rs.git
cd linear-mcp-rs
cargo build --release
# Binary: target/release/linear-mcp
```
