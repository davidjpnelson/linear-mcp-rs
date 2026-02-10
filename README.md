# linear-mcp-rs

A Rust MCP (Model Context Protocol) server for Linear. Single binary, no runtime dependencies.

## Setup

### 1. Set your Linear API key

Create a personal API key at https://linear.app/settings/api and set it:

```bash
export LINEAR_API_KEY=lin_api_xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

Or store it in macOS Keychain:

```bash
security add-generic-password -s linear-api-key -a "$USER" -w "lin_api_xxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
```

### 2. Build

```bash
cargo build --release
```

The binary is at `target/release/linear-mcp`.

### 3. Add to Claude Code

Add to `~/.claude/settings.json`:

```json
{
  "mcpServers": {
    "linear-mcp": {
      "command": "/path/to/linear-mcp-rs/target/release/linear-mcp",
      "env": {
        "LINEAR_API_KEY": "lin_api_xxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
      }
    }
  }
}
```

## Tools

| Tool | Description |
|------|-------------|
| `list_issues` | Filter issues by team, assignee, status, project, label, priority |
| `search_issues` | Full-text search across issues |
| `get_issue` | Full detail by identifier (e.g. `ENG-123`) or UUID |
| `list_teams` | All teams with optional member counts |
| `list_projects` | Projects with status and progress |
| `list_users` | Workspace members |
| `list_states` | Workflow states, optionally by team |
| `my_issues` | Your issues grouped by status |
| `create_issue` | Create issue with human-friendly inputs |
| `update_issue` | Update issue, use `"none"` to clear fields |
| `add_comment` | Add markdown comment to an issue |
