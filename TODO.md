# linear-mcp-rs - Full API Coverage TODO

## Phase 0: Cleanup & Integration (from crashed session)
- [x] Merge types2.rs into types.rs
- [x] Merge format2.rs into format.rs
- [x] Merge queries2.rs into queries.rs
- [x] Merge response2.rs into response.rs
- [x] Create individual tool files from tools2.rs param structs
- [x] Convert server2.rs standalone handlers into LinearMcp methods
- [x] Wire all new tools into #[tool_router] in server.rs
- [x] Update tools/mod.rs exports
- [x] Delete all *2.rs files
- [x] Verify compilation (cargo check)

## Phase 1: Core Issue Tools (existing - 22 tools)
- [x] list_issues - Filter by team, assignee, status, project, label, priority, dates
- [x] search_issues - Full-text search with filters
- [x] get_issue - Full issue detail with comments, relations, labels
- [x] create_issue - Human-friendly inputs (team key, email, state name)
- [x] update_issue - With label, project, parent support
- [x] archive_issue - Archive by identifier
- [x] my_issues - Viewer's assigned issues grouped by status
- [x] add_comment - With threaded reply support (parentId)
- [x] update_comment - Edit comment by UUID
- [x] delete_comment - Delete comment by UUID
- [x] list_teams - With optional member counts
- [x] list_projects - Filter by status, lead, team
- [x] list_users - Workspace members
- [x] list_states - Workflow states grouped by team
- [x] list_cycles - Cycles for a team
- [x] get_cycle - Cycle detail by UUID
- [x] add_issue_to_cycle - Assign issue to cycle
- [x] remove_issue_from_cycle - Unassign issue from cycle
- [x] list_labels - Workspace/team labels
- [x] create_label - With optional team scope and color
- [x] create_issue_relation - blocks, blocked_by, related, duplicate
- [x] delete_issue_relation - Remove relation by UUID

## Phase 2: Project Tools (integrated)
- [x] get_project - Full project detail (teams, members, lead, dates)
- [x] create_project - With team keys, lead email, dates
- [x] update_project - Name, description, state, lead, dates
- [x] archive_project - Archive project
- [x] list_project_updates - Status updates for a project
- [x] create_project_update - Post status update with health
- [x] list_project_milestones - Milestones for a project
- [x] create_project_milestone - With name, description, target date

## Phase 3: Document Tools (integrated)
- [x] list_documents - All documents with project/creator info
- [x] get_document - Full document with content
- [x] create_document - With title, content, project association
- [x] update_document - Edit document title/content

## Phase 4: Planning Tools (integrated)
- [x] list_roadmaps - All roadmaps
- [x] list_initiatives - All initiatives
- [x] create_initiative - Create initiative
- [x] update_initiative - Update initiative
- [x] delete_initiative - Delete initiative

## Phase 5: Notification & View Tools (integrated)
- [x] list_notifications - Inbox notifications
- [x] mark_notification_read - Mark as read
- [x] list_views - Custom saved views
- [x] get_view_issues - Execute a custom view's filter

## Phase 6: Attachment & Reaction Tools (integrated)
- [x] list_attachments - Attachments on an issue
- [x] add_attachment - Link URL/resource to issue
- [x] add_reaction - Emoji reaction on comment
- [x] remove_reaction - Remove reaction

## Phase 7: Favorites & Triage (integrated)
- [x] list_favorites - User's favorites
- [x] add_favorite - Favorite issue or project
- [x] remove_favorite - Remove favorite
- [x] list_triage_issues - Issues in triage state for a team
- [x] triage_issue - Move issue out of triage

## Phase 8: Templates & History (integrated)
- [x] list_templates - Available templates
- [x] create_issue_from_template - Create issue using template data
- [x] get_issue_history - Audit trail of changes

## Phase 9: Webhook & Admin Tools (integrated)
- [x] list_webhooks - Active webhooks
- [x] create_webhook - Create webhook endpoint
- [x] delete_webhook - Remove webhook
- [x] list_integrations - Active integrations
- [x] query_audit_log - Audit log entries

## Phase 10: Team Management (integrated)
- [x] create_team - Create new team
- [x] update_team - Update team settings

## Phase 11: Additional Tools
- [x] bulk_update_issues - Batch update multiple issues
- [x] create_cycle - Create new cycle for a team
- [x] update_label - Update label name/color
- [x] archive_label - Archive/delete label
- [x] unarchive_issue - Restore archived issue
- [x] archive_project - Archive a project
- [x] update_document - Edit document title/content
- [x] search_documents - Full-text search across documents

## Final
- [x] Remove all dead code warnings
- [x] cargo check passes clean
- [x] Update README with full tool list
- [x] Build release binary
- [x] v0.2.0 released with all 68 tools
