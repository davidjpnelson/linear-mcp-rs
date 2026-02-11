use crate::graphql::response::WorkflowStateWithTeam;
use crate::types::*;

/// Format an issue as a one-line summary for list views.
pub fn format_issue_summary(issue: &Issue) -> String {
    let mut parts = vec![format!("[{}]", issue.identifier), issue.title.clone()];

    let mut meta = Vec::new();
    if let Some(ref state) = issue.state {
        meta.push(state.name.clone());
    }
    meta.push(priority_label(issue.priority).to_string());
    if let Some(ref assignee) = issue.assignee {
        meta.push(format!("@{}", assignee.display_name));
    }
    if let Some(ref team) = issue.team {
        meta.push(format!("team:{}", team.key));
    }
    if let Some(ref project) = issue.project {
        meta.push(format!("project:{}", project.name));
    }
    if let Some(estimate) = issue.estimate {
        meta.push(format!("est:{}", estimate));
    }
    if let Some(ref due) = issue.due_date {
        meta.push(format!("due:{}", due));
    }
    if let Some(ref labels) = issue.labels {
        if !labels.nodes.is_empty() {
            meta.push(
                labels
                    .nodes
                    .iter()
                    .map(|l| l.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
    }

    parts.push(format!("({})", meta.join(", ")));
    parts.join(" ")
}

/// Format an issue with full detail (markdown).
pub fn format_issue_detail(issue: &Issue) -> String {
    let mut lines = Vec::new();

    lines.push(format!("# {}: {}", issue.identifier, issue.title));
    lines.push(String::new());

    if let Some(ref state) = issue.state {
        lines.push(format!("**Status:** {}", state.name));
    }
    lines.push(format!("**Priority:** {}", priority_label(issue.priority)));
    if let Some(ref assignee) = issue.assignee {
        if let Some(ref email) = assignee.email {
            lines.push(format!(
                "**Assignee:** {} <{}>",
                assignee.display_name, email
            ));
        } else {
            lines.push(format!("**Assignee:** {}", assignee.display_name));
        }
    }
    if let Some(ref team) = issue.team {
        lines.push(format!("**Team:** {} ({})", team.name, team.key));
    }
    if let Some(ref project) = issue.project {
        lines.push(format!("**Project:** {}", project.name));
    }
    if let Some(ref labels) = issue.labels {
        if !labels.nodes.is_empty() {
            let label_str = labels
                .nodes
                .iter()
                .map(|l| l.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!("**Labels:** {}", label_str));
        }
    }
    if let Some(estimate) = issue.estimate {
        lines.push(format!("**Estimate:** {}", estimate));
    }
    if let Some(ref due) = issue.due_date {
        lines.push(format!("**Due:** {}", due));
    }
    if let Some(ref branch) = issue.branch_name {
        lines.push(format!("**Branch:** {}", branch));
    }

    if let Some(ref creator) = issue.creator {
        if let Some(ref email) = creator.email {
            lines.push(format!(
                "**Creator:** {} <{}>",
                creator.display_name, email
            ));
        } else {
            lines.push(format!("**Creator:** {}", creator.display_name));
        }
    }
    if let Some(ref cycle) = issue.cycle {
        let fallback = format!("Cycle {}", cycle.number);
        let cycle_name = cycle.name.as_deref().unwrap_or(&fallback);
        lines.push(format!("**Cycle:** {} ({})", cycle_name, cycle.id));
    }

    // Dates -- extract just the date portion
    if let Some(ref created) = issue.created_at {
        lines.push(format!("**Created:** {}", format_date(created)));
    }
    if let Some(ref updated) = issue.updated_at {
        lines.push(format!("**Updated:** {}", format_date(updated)));
    }
    if let Some(ref started) = issue.started_at {
        lines.push(format!("**Started:** {}", format_date(started)));
    }
    if let Some(ref completed) = issue.completed_at {
        lines.push(format!("**Completed:** {}", format_date(completed)));
    }
    if let Some(ref canceled) = issue.canceled_at {
        lines.push(format!("**Canceled:** {}", format_date(canceled)));
    }
    lines.push(format!("**URL:** {}", issue.url));

    // Parent
    if let Some(ref parent) = issue.parent {
        lines.push(String::new());
        lines.push(format!(
            "**Parent:** [{}] {}",
            parent.identifier, parent.title
        ));
    }

    // Relations
    if let Some(ref relations) = issue.relations {
        if !relations.nodes.is_empty() {
            lines.push(String::new());
            lines.push("## Relations".to_string());
            for relation in &relations.nodes {
                let related = relation
                    .related_issue
                    .as_ref()
                    .map(|i| format!("[{}] {}", i.identifier, i.title))
                    .unwrap_or_else(|| "?".to_string());
                lines.push(format!("- {} {}", relation.relation_type, related));
            }
        }
    }

    // Subscribers
    if let Some(ref subscribers) = issue.subscribers {
        if !subscribers.nodes.is_empty() {
            lines.push(String::new());
            let sub_names: Vec<String> = subscribers
                .nodes
                .iter()
                .map(|u| {
                    if let Some(ref email) = u.email {
                        format!("{} <{}>", u.display_name, email)
                    } else {
                        u.display_name.clone()
                    }
                })
                .collect();
            lines.push(format!("**Subscribers:** {}", sub_names.join(", ")));
        }
    }

    // Description
    if let Some(ref desc) = issue.description {
        if !desc.is_empty() {
            lines.push(String::new());
            lines.push("## Description".to_string());
            lines.push(desc.clone());
        }
    }

    // Sub-issues
    if let Some(ref children) = issue.children {
        if !children.nodes.is_empty() {
            lines.push(String::new());
            lines.push("## Sub-issues".to_string());
            for child in &children.nodes {
                lines.push(format!("- [{}] {}", child.identifier, child.title));
            }
        }
    }

    // Comments
    if let Some(ref comments) = issue.comments {
        if !comments.nodes.is_empty() {
            lines.push(String::new());
            lines.push("## Comments".to_string());
            for comment in &comments.nodes {
                lines.push(format_comment(comment));
                lines.push(String::new());
            }
        }
    }

    lines.join("\n")
}

pub fn format_comment(comment: &Comment) -> String {
    let date = format_date(&comment.created_at);
    format!("**{}:** {}", date, comment.body)
}

pub fn format_team(team: &crate::types::Team, member_count: Option<usize>) -> String {
    match member_count {
        Some(count) => format!("{} | {} ({} members)", team.key, team.name, count),
        None => format!("{} | {}", team.key, team.name),
    }
}

pub fn format_project(project: &Project) -> String {
    let progress = project.progress.unwrap_or(0.0);
    let pct = (progress * 100.0).round() as i32;
    let state = project.state.as_deref().unwrap_or("unknown");
    let mut parts = vec![format!("{} [{}] - {}% complete", project.name, state, pct)];

    if let Some(ref lead) = project.lead {
        let lead_str = if let Some(ref email) = lead.email {
            format!("{} <{}>", lead.display_name, email)
        } else {
            lead.display_name.clone()
        };
        parts.push(format!("  Lead: {}", lead_str));
    }
    if let Some(ref teams) = project.teams {
        if !teams.nodes.is_empty() {
            let team_keys: Vec<&str> = teams.nodes.iter().map(|t| t.key.as_str()).collect();
            parts.push(format!("  Teams: {}", team_keys.join(", ")));
        }
    }
    if let Some(ref members) = project.members {
        if !members.nodes.is_empty() {
            let member_names: Vec<&str> = members
                .nodes
                .iter()
                .map(|m| m.display_name.as_str())
                .collect();
            parts.push(format!("  Members: {}", member_names.join(", ")));
        }
    }
    if let Some(ref start) = project.start_date {
        parts.push(format!("  Start: {}", start));
    }
    if let Some(ref target) = project.target_date {
        parts.push(format!("  Target: {}", target));
    }
    if let Some(ref url) = project.url {
        parts.push(format!("  URL: {}", url));
    }
    if let Some(ref desc) = project.description {
        if !desc.is_empty() {
            // Truncate long descriptions for list view (char-safe)
            let truncated = if desc.chars().count() > 100 {
                let t: String = desc.chars().take(100).collect();
                format!("{}...", t)
            } else {
                desc.clone()
            };
            parts.push(format!("  Description: {}", truncated));
        }
    }

    parts.join("\n")
}

pub fn format_user(user: &User) -> String {
    let role = if user.admin.unwrap_or(false) {
        "admin"
    } else if user.guest.unwrap_or(false) {
        "guest"
    } else {
        "member"
    };
    let email = user.email.as_deref().unwrap_or("no email");
    format!("{} <{}> ({})", user.display_name, email, role)
}

pub fn format_workflow_state(state: &WorkflowStateWithTeam) -> String {
    format!("{} [{}] ({})", state.name, state.state_type, state.color)
}

pub fn format_pagination_with_cursor(
    has_next_page: bool,
    count: usize,
    end_cursor: Option<&str>,
) -> String {
    let s = if count != 1 { "s" } else { "" };
    if has_next_page {
        match end_cursor {
            Some(cursor) => format!(
                "\n---\nShowing {} result{}. More available -- use cursor: \"{}\"",
                count, s, cursor
            ),
            None => format!("\n---\nShowing {} result{}. More available.", count, s),
        }
    } else {
        format!("\n---\nShowing all {} result{}.", count, s)
    }
}

/// Format a cycle as a one-line summary for list views.
pub fn format_cycle_summary(cycle: &Cycle) -> String {
    let fallback = format!("Cycle {}", cycle.number);
    let name = cycle.name.as_deref().unwrap_or(&fallback);
    let progress = cycle.progress.unwrap_or(0.0);
    let pct = (progress * 100.0).round() as i32;
    let start = cycle.starts_at.as_deref().unwrap_or("?");
    let end = cycle.ends_at.as_deref().unwrap_or("?");
    let completed = if cycle.completed_at.is_some() {
        " [completed]"
    } else {
        ""
    };
    format!(
        "{} ({} - {}) {}% complete{}  [id: {}]",
        name,
        format_date(start),
        format_date(end),
        pct,
        completed,
        cycle.id
    )
}

/// Format a cycle with full detail (markdown).
pub fn format_cycle_detail(cycle: &Cycle) -> String {
    let mut lines = Vec::new();
    let fallback = format!("Cycle {}", cycle.number);
    let name = cycle.name.as_deref().unwrap_or(&fallback);
    lines.push(format!("# {}", name));
    lines.push(String::new());
    lines.push(format!("**Number:** {}", cycle.number));
    lines.push(format!("**ID:** {}", cycle.id));
    if let Some(ref start) = cycle.starts_at {
        lines.push(format!("**Starts:** {}", format_date(start)));
    }
    if let Some(ref end) = cycle.ends_at {
        lines.push(format!("**Ends:** {}", format_date(end)));
    }
    if let Some(ref completed) = cycle.completed_at {
        lines.push(format!("**Completed:** {}", format_date(completed)));
    }
    let progress = cycle.progress.unwrap_or(0.0);
    let pct = (progress * 100.0).round() as i32;
    lines.push(format!("**Progress:** {}%", pct));
    lines.join("\n")
}

/// Format a label as a one-line summary.
pub fn format_label(label: &Label) -> String {
    format!("{} [id: {}]", label.name, label.id)
}

/// Format an issue relation as a one-line summary.
pub fn format_issue_relation(relation: &IssueRelation) -> String {
    let issue_str = relation
        .issue
        .as_ref()
        .map(|i| format!("[{}] {}", i.identifier, i.title))
        .unwrap_or_default();
    let related_str = relation
        .related_issue
        .as_ref()
        .map(|i| format!("[{}] {}", i.identifier, i.title))
        .unwrap_or_default();
    format!(
        "{} --{}-- {}  [relation id: {}]",
        issue_str, relation.relation_type, related_str, relation.id,
    )
}

fn format_date(iso: &str) -> &str {
    // Extract YYYY-MM-DD from ISO timestamp
    if iso.len() >= 10 {
        &iso[..10]
    } else {
        iso
    }
}

// ---- #17: Documents ----

pub fn format_document_summary(doc: &Document) -> String {
    let mut parts = vec![doc.title.clone()];
    let mut meta = Vec::new();
    if let Some(ref project) = doc.project {
        meta.push(format!("project: {}", project.name));
    }
    if let Some(ref creator) = doc.creator {
        meta.push(format!("by {}", creator.display_name));
    }
    if let Some(ref updated) = doc.updated_at {
        meta.push(format!("updated {}", format_date(updated)));
    }
    if !meta.is_empty() {
        parts.push(format!("({})", meta.join(", ")));
    }
    parts.join(" ")
}

pub fn format_document_detail(doc: &Document) -> String {
    let mut lines = Vec::new();
    lines.push(format!("# {}", doc.title));
    lines.push(String::new());

    if let Some(ref creator) = doc.creator {
        lines.push(format!("**Author:** {}", creator.display_name));
    }
    if let Some(ref project) = doc.project {
        lines.push(format!("**Project:** {}", project.name));
    }
    if let Some(ref created) = doc.created_at {
        lines.push(format!("**Created:** {}", format_date(created)));
    }
    if let Some(ref updated) = doc.updated_at {
        lines.push(format!("**Updated:** {}", format_date(updated)));
    }

    if let Some(ref content) = doc.content {
        if !content.is_empty() {
            lines.push(String::new());
            lines.push("## Content".to_string());
            lines.push(content.clone());
        }
    }

    lines.join("\n")
}

// ---- #18: Project detail ----

pub fn format_project_detail(project: &ProjectDetail) -> String {
    let mut lines = Vec::new();
    let progress = project.progress.unwrap_or(0.0);
    let pct = (progress * 100.0).round() as i32;

    lines.push(format!("# {}", project.name));
    lines.push(String::new());
    lines.push(format!("**State:** {}", project.state.as_deref().unwrap_or("unknown")));
    lines.push(format!("**Progress:** {}%", pct));

    if let Some(ref desc) = project.description {
        if !desc.is_empty() {
            lines.push(format!("**Description:** {}", desc));
        }
    }
    if let Some(ref lead) = project.lead {
        lines.push(format!("**Lead:** {}", lead.display_name));
    }
    if let Some(ref start) = project.start_date {
        lines.push(format!("**Start Date:** {}", format_date(start)));
    }
    if let Some(ref target) = project.target_date {
        lines.push(format!("**Target Date:** {}", format_date(target)));
    }
    if let Some(ref created) = project.created_at {
        lines.push(format!("**Created:** {}", format_date(created)));
    }
    if let Some(ref updated) = project.updated_at {
        lines.push(format!("**Updated:** {}", format_date(updated)));
    }

    if let Some(ref teams) = project.teams {
        if !teams.nodes.is_empty() {
            let team_str = teams
                .nodes
                .iter()
                .map(|t| format!("{} ({})", t.name, t.key))
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!("**Teams:** {}", team_str));
        }
    }

    if let Some(ref members) = project.members {
        if !members.nodes.is_empty() {
            let member_str = members
                .nodes
                .iter()
                .map(|m| m.display_name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!("**Members:** {}", member_str));
        }
    }

    lines.join("\n")
}

// ---- #19: Project Updates ----

pub fn format_project_update(update: &ProjectUpdate) -> String {
    let mut parts = Vec::new();

    if let Some(ref created) = update.created_at {
        parts.push(format!("**{}**", format_date(created)));
    }
    if let Some(ref user) = update.user {
        parts.push(format!("by {}", user.display_name));
    }
    if let Some(ref health) = update.health {
        parts.push(format!("[{}]", health));
    }
    parts.push(format!("\n{}", update.body));

    parts.join(" ")
}

// ---- #20: Project Milestones ----

pub fn format_project_milestone(milestone: &ProjectMilestone) -> String {
    let mut parts = vec![milestone.name.clone()];
    let mut meta = Vec::new();
    if let Some(ref target) = milestone.target_date {
        meta.push(format!("target: {}", format_date(target)));
    }
    if let Some(ref desc) = milestone.description {
        if !desc.is_empty() {
            meta.push(desc.clone());
        }
    }
    if !meta.is_empty() {
        parts.push(format!("({})", meta.join(", ")));
    }
    parts.join(" ")
}

// ---- #21: Roadmaps and Initiatives ----

pub fn format_roadmap(roadmap: &Roadmap) -> String {
    let mut parts = vec![roadmap.name.clone()];
    if let Some(ref desc) = roadmap.description {
        if !desc.is_empty() {
            parts.push(format!("- {}", desc));
        }
    }
    parts.join(" ")
}

pub fn format_initiative(initiative: &Initiative) -> String {
    let mut parts = vec![initiative.name.clone()];
    let mut meta = Vec::new();
    if let Some(ref status) = initiative.status {
        meta.push(status.clone());
    }
    if !meta.is_empty() {
        parts.push(format!("({})", meta.join(", ")));
    }
    if let Some(ref desc) = initiative.description {
        if !desc.is_empty() {
            parts.push(format!("- {}", desc));
        }
    }
    parts.join(" ")
}

// ---- #22: Notifications ----

pub fn format_notification(notification: &Notification) -> String {
    let mut parts = vec![format!("[{}]", notification.notification_type)];
    if let Some(ref issue) = notification.issue {
        parts.push(format!("{}: {}", issue.identifier, issue.title));
    }
    let mut meta = Vec::new();
    if let Some(ref created) = notification.created_at {
        meta.push(format_date(created).to_string());
    }
    if notification.read_at.is_some() {
        meta.push("read".to_string());
    } else {
        meta.push("unread".to_string());
    }
    if !meta.is_empty() {
        parts.push(format!("({})", meta.join(", ")));
    }
    parts.join(" ")
}

// ---- #23: Attachments ----

pub fn format_attachment(attachment: &Attachment) -> String {
    let title = attachment
        .title
        .as_deref()
        .unwrap_or("Untitled");
    let url = attachment.url.as_deref().unwrap_or("no URL");
    let mut parts = vec![format!("{} - {}", title, url)];
    if let Some(ref created) = attachment.created_at {
        parts.push(format!("({})", format_date(created)));
    }
    parts.join(" ")
}

// ---- #25: Custom Views ----

pub fn format_custom_view(view: &CustomView) -> String {
    let mut parts = vec![view.name.clone()];
    if let Some(ref desc) = view.description {
        if !desc.is_empty() {
            parts.push(format!("- {}", desc));
        }
    }
    parts.join(" ")
}

// ---- #26: Favorites ----

pub fn format_favorite(favorite: &Favorite) -> String {
    let fav_type = favorite
        .favorite_type
        .as_deref()
        .unwrap_or("unknown");
    let mut parts = vec![format!("[{}]", fav_type)];
    if let Some(ref issue) = favorite.issue {
        parts.push(format!("{}: {}", issue.identifier, issue.title));
    }
    if let Some(ref project) = favorite.project {
        parts.push(project.name.clone());
    }
    parts.join(" ")
}

// ---- #29: Templates ----

pub fn format_template(template: &Template) -> String {
    let mut parts = vec![template.name.clone()];
    if let Some(ref desc) = template.description {
        if !desc.is_empty() {
            parts.push(format!("- {}", desc));
        }
    }
    parts.join(" ")
}

// ---- #30: Issue History ----

pub fn format_history_entry(entry: &IssueHistoryEntry) -> String {
    let mut parts = Vec::new();

    if let Some(ref created) = entry.created_at {
        parts.push(format!("**{}**", format_date(created)));
    }

    if let Some(ref actor) = entry.actor {
        parts.push(actor.display_name.clone());
    }

    // State transition
    if let (Some(ref from), Some(ref to)) = (&entry.from_state, &entry.to_state) {
        parts.push(format!("{} -> {}", from.name, to.name));
    } else if let Some(ref to) = entry.to_state {
        parts.push(format!("-> {}", to.name));
    }

    // Label changes
    if let Some(ref added) = entry.added_labels {
        if !added.nodes.is_empty() {
            let labels = added
                .nodes
                .iter()
                .map(|l| l.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("+labels: {}", labels));
        }
    }
    if let Some(ref removed) = entry.removed_labels {
        if !removed.nodes.is_empty() {
            let labels = removed
                .nodes
                .iter()
                .map(|l| l.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("-labels: {}", labels));
        }
    }

    parts.join(" | ")
}

// ---- #31: Webhooks ----

pub fn format_webhook(webhook: &Webhook) -> String {
    let url = webhook.url.as_deref().unwrap_or("no URL");
    let label = webhook.label.as_deref().unwrap_or("no label");
    let enabled = if webhook.enabled.unwrap_or(false) {
        "enabled"
    } else {
        "disabled"
    };
    let resources = webhook
        .resource_types
        .as_ref()
        .map(|r| r.join(", "))
        .unwrap_or_default();
    format!("{} | {} | {} [{}]", label, url, enabled, resources)
}

// ---- #32: Integrations and Audit Log ----

pub fn format_integration(integration: &Integration) -> String {
    let mut parts = vec![integration.service.clone()];
    if let Some(ref created) = integration.created_at {
        parts.push(format!("({})", format_date(created)));
    }
    parts.join(" ")
}

pub fn format_audit_entry(entry: &AuditEntry) -> String {
    let mut parts = Vec::new();
    if let Some(ref created) = entry.created_at {
        parts.push(format_date(created).to_string());
    }
    if let Some(ref entry_type) = entry.entry_type {
        parts.push(entry_type.clone());
    }
    if let Some(ref actor_id) = entry.actor_id {
        parts.push(format!("actor: {}", actor_id));
    }
    if let Some(ref ip) = entry.ip {
        parts.push(format!("IP: {}", ip));
    }
    parts.join(" | ")
}

// ---- #33: Team and User enriched ----

pub fn format_team_detail(team: &TeamDetail) -> String {
    let mut parts = vec![format!("{} | {}", team.key, team.name)];
    let mut meta = Vec::new();
    if let Some(ref desc) = team.description {
        if !desc.is_empty() {
            meta.push(desc.clone());
        }
    }
    if let Some(ref tz) = team.timezone {
        meta.push(format!("tz: {}", tz));
    }
    if !meta.is_empty() {
        parts.push(format!("({})", meta.join(", ")));
    }
    parts.join(" ")
}

pub fn format_cycle_created(cycle: &CycleDetail) -> String {
    let name = cycle
        .name
        .as_deref()
        .unwrap_or("(unnamed)");
    let number = cycle
        .number
        .map(|n| format!(" #{}", n))
        .unwrap_or_default();
    let starts = cycle
        .starts_at
        .as_deref()
        .map(|d| format_date(d))
        .unwrap_or("?");
    let ends = cycle
        .ends_at
        .as_deref()
        .map(|d| format_date(d))
        .unwrap_or("?");
    format!(
        "**{}{}** [id: {}]\n{} → {}",
        name, number, cycle.id, starts, ends
    )
}

// ---- Phase 12: Remaining tools ----

/// Format a document search result as a one-line summary.
pub fn format_document_search_result(doc: &DocumentSearchResult) -> String {
    let mut parts = vec![doc.title.clone()];
    if let Some(ref project) = doc.project {
        parts.push(format!("({})", project.name));
    }
    if let Some(ref creator) = doc.creator {
        parts.push(format!("by {}", creator.display_name));
    }
    if let Some(ref url) = doc.url {
        parts.push(format!("[{}]", url));
    }
    parts.join(" ")
}

/// Format an initiative detail (from mutation result).
pub fn format_initiative_detail(initiative: &InitiativeDetail) -> String {
    let mut lines = vec![format!("**{}** [id: {}]", initiative.name, initiative.id)];
    if let Some(ref desc) = initiative.description {
        if !desc.is_empty() {
            lines.push(format!("Description: {}", desc));
        }
    }
    if let Some(ref status) = initiative.status {
        lines.push(format!("Status: {}", status));
    }
    lines.join("\n")
}
