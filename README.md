# linear-mcp-rs

Rust MCP server for Linear. Single ~4MB binary, no runtime dependencies.

## Install

Requires [GitHub CLI](https://cli.github.com) authenticated with access to this repo.

```bash
gh release download --repo davidjpnelson/linear-mcp-rs -p install.sh -O- | bash
```

This downloads the right binary for your OS/arch to `~/.local/bin/linear-mcp`.

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

### 2. Add to Claude Code

```bash
claude mcp add linear-mcp-rs ~/.local/bin/linear-mcp
```

Then restart Claude Code.

## Tools

All 11 tools accept human-friendly inputs (team keys, emails, state names) and resolve them to IDs automatically.

### Read

| Tool | Description |
|------|-------------|
| `list_issues` | Filter by team, assignee, status, project, label, priority. Paginated. |
| `search_issues` | Full-text search across titles, descriptions, and comments |
| `get_issue` | Full detail by identifier (e.g. `ENG-123`) or UUID. Includes comments, labels, sub-issues. |
| `list_teams` | All teams with optional member counts |
| `list_projects` | Projects with status and completion percentage |
| `list_users` | Workspace members with roles |
| `list_states` | Workflow states grouped by team |
| `my_issues` | Your assigned issues grouped by status |

### Write

| Tool | Description |
|------|-------------|
| `create_issue` | Create issue. Pass team key, assignee email, state name — resolved automatically. |
| `update_issue` | Update issue. Use `"none"` for assignee/dueDate to clear them. |
| `add_comment` | Add a markdown comment to an issue |

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
