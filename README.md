# linear-mcp-rs

Rust MCP server for Linear. Single ~4MB binary, no runtime dependencies.

## Install

```bash
gh release download --repo davidjpnelson/linear-mcp-rs -p install.sh -O- | bash
```

Requires [GitHub CLI](https://cli.github.com). Downloads the right binary for your OS/arch to `~/.local/bin/linear-mcp`.

## Setup

### 1. Linear API key

Create a personal API key at [https://linear.app](https://linear.app) → **Settings** → **Security & access** → Personal API Keys

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

All 68 tools accept human-friendly inputs (team keys like `ENG`, emails, state names, project names) and resolve them to IDs automatically.

### Issues

| Tool | Description |
|------|-------------|
| `list_issues` | Filter by team, assignee, status, project, label, priority. Paginated. |
| `search_issues` | Full-text search across titles, descriptions, and comments |
| `get_issue` | Full detail by identifier (e.g. `ENG-123`) or UUID. Includes comments, labels, relations. |
| `my_issues` | Your assigned issues grouped by status |
| `create_issue` | Create issue with team key, assignee email, state name, labels, priority. |
| `create_issue_from_template` | Create issue from a saved template |
| `update_issue` | Update any field. Use `"none"` to clear assignee/dueDate. |
| `bulk_update_issues` | Update multiple issues at once (status, priority, assignee, labels). |
| `archive_issue` | Archive an issue |
| `unarchive_issue` | Restore an archived issue |

### Triage

| Tool | Description |
|------|-------------|
| `list_triage_issues` | List issues in a team's triage state |
| `triage_issue` | Move a triage issue to a target workflow state |

### Comments

| Tool | Description |
|------|-------------|
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

### Project Milestones & Updates

| Tool | Description |
|------|-------------|
| `list_project_milestones` | List milestones for a project |
| `create_project_milestone` | Create a milestone with target date |
| `list_project_updates` | List status updates for a project |
| `create_project_update` | Post a status update to a project |

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
| `delete_initiative` | Permanently delete an initiative |

### Documents

| Tool | Description |
|------|-------------|
| `list_documents` | List documents, optionally filtered by project |
| `get_document` | Get full document content |
| `search_documents` | Full-text search across documents |
| `create_document` | Create a document, optionally linked to a project |
| `update_document` | Update document title or content |

### Teams

| Tool | Description |
|------|-------------|
| `list_teams` | All teams with keys |
| `list_states` | Workflow states grouped by team |
| `create_team` | Create a new team |
| `update_team` | Update team name, description, or timezone |

### Users & Notifications

| Tool | Description |
|------|-------------|
| `list_users` | Workspace members with roles |
| `list_notifications` | Your recent notifications |
| `mark_notification_read` | Mark a notification as read |

### Views

| Tool | Description |
|------|-------------|
| `list_views` | List saved custom views |
| `get_view_issues` | Get issues matching a custom view's filters |

### Relations & Attachments

| Tool | Description |
|------|-------------|
| `create_issue_relation` | Create a relation between issues (related, blocks, duplicate) |
| `delete_issue_relation` | Delete a relation |
| `list_attachments` | List attachments on an issue |
| `add_attachment` | Add a URL attachment to an issue |

### Favorites

| Tool | Description |
|------|-------------|
| `list_favorites` | List your favorited items |
| `add_favorite` | Favorite an issue or project |
| `remove_favorite` | Remove a favorite by UUID |

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

### Other

| Tool | Description |
|------|-------------|
| `list_templates` | List issue templates |
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
git clone git@github.com:davidjpnelson/linear-mcp-rs.git
cd linear-mcp-rs
cargo build --release
# Binary: target/release/linear-mcp
```
