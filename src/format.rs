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
    lines.push(format!("**ID:** {}", issue.id));

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
    if let Some(ref auto_closed) = issue.auto_closed_at {
        lines.push(format!("**Auto Closed:** {}", format_date(auto_closed)));
    }
    if let Some(ref auto_archived) = issue.auto_archived_at {
        lines.push(format!("**Auto Archived:** {}", format_date(auto_archived)));
    }
    if let Some(ref sla_type) = issue.sla_type {
        lines.push(format!("**SLA Type:** {}", sla_type));
    }
    if let Some(ref sla_breaches) = issue.sla_breaches_at {
        lines.push(format!("**SLA Breaches At:** {}", format_date(sla_breaches)));
    }
    if let Some(ref sla_started) = issue.sla_started_at {
        lines.push(format!("**SLA Started At:** {}", format_date(sla_started)));
    }
    if let Some(count) = issue.customer_ticket_count {
        if count > 0 {
            lines.push(format!("**Customer Tickets:** {}", count));
        }
    }
    if let Some(ref prev) = issue.previous_identifiers {
        if !prev.is_empty() {
            lines.push(format!("**Previous IDs:** {}", prev.join(", ")));
        }
    }
    if let Some(true) = issue.trashed {
        lines.push("**Trashed:** yes".to_string());
    }
    if let Some(ref snoozed) = issue.snoozed_until_at {
        lines.push(format!("**Snoozed Until:** {}", format_date(snoozed)));
    }
    if let Some(ref milestone) = issue.project_milestone {
        lines.push(format!("**Milestone:** {}", milestone.name));
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
    let resolved = if comment.resolved_at.is_some() {
        " [resolved]"
    } else {
        ""
    };
    let user = comment
        .user
        .as_ref()
        .map(|u| format!(" ({})", u.display_name))
        .unwrap_or_default();
    format!(
        "**{}{}{}:** {}\n[id: {}]",
        date, user, resolved, comment.body, comment.id
    )
}

pub fn format_team(team: &crate::types::Team, member_count: Option<usize>) -> String {
    let mut parts = vec![format!("{} | {}", team.key, team.name)];
    let mut meta = Vec::new();
    if let Some(count) = member_count {
        meta.push(format!("{} members", count));
    }
    if let Some(ref tz) = team.timezone {
        meta.push(format!("tz: {}", tz));
    }
    if let Some(true) = team.triage_enabled {
        meta.push("triage: on".to_string());
    }
    if let Some(ref state) = team.default_issue_state {
        meta.push(format!("default: {}", state.name));
    }
    if !meta.is_empty() {
        parts.push(format!("({})", meta.join(", ")));
    }
    if let Some(ref desc) = team.description {
        if !desc.is_empty() {
            let truncated = if desc.chars().count() > 80 {
                let t: String = desc.chars().take(80).collect();
                format!("{}...", t)
            } else {
                desc.clone()
            };
            parts.push(format!("- {}", truncated));
        }
    }
    parts.join(" ")
}

pub fn format_project(project: &Project) -> String {
    let progress = project.progress.unwrap_or(0.0);
    let pct = (progress * 100.0).round() as i32;
    let state = project.state.as_deref().unwrap_or("unknown");
    let health = project
        .health
        .as_ref()
        .map(|h| format!(" [{}]", h))
        .unwrap_or_default();
    let mut parts = vec![format!("{} [{}] - {}% complete{}", project.name, state, pct, health)];

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
    if let Some(ref desc) = cycle.description {
        if !desc.is_empty() {
            lines.push(format!("**Description:** {}", desc));
        }
    }
    if let Some(ref issues) = cycle.issues {
        if !issues.nodes.is_empty() {
            lines.push(String::new());
            lines.push("## Issues".to_string());
            for issue in &issues.nodes {
                let state = issue
                    .state
                    .as_ref()
                    .map(|s| format!(" ({})", s.name))
                    .unwrap_or_default();
                lines.push(format!("- [{}] {}{}", issue.identifier, issue.title, state));
            }
        }
    }
    if let Some(ref uncompleted) = cycle.uncompleted_issues_upon_close {
        if !uncompleted.nodes.is_empty() {
            lines.push(String::new());
            lines.push("## Uncompleted Issues (upon close)".to_string());
            for issue in &uncompleted.nodes {
                lines.push(format!("- [{}] {}", issue.identifier, issue.title));
            }
        }
    }
    lines.join("\n")
}

/// Format a label as a one-line summary.
pub fn format_label(label: &Label) -> String {
    let mut parts = vec![label.name.clone()];
    if let Some(ref color) = label.color {
        parts.push(format!("({})", color));
    }
    if let Some(ref parent) = label.parent {
        parts.push(format!("parent: {}", parent.name));
    }
    if let Some(ref team) = label.team {
        parts.push(format!("team: {}", team.key));
    }
    parts.push(format!("[id: {}]", label.id));
    parts.join(" ")
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
    lines.push(format!("**ID:** {}", project.id));
    lines.push(format!("**State:** {}", project.state.as_deref().unwrap_or("unknown")));
    lines.push(format!("**Progress:** {}%", pct));
    if let Some(ref health) = project.health {
        lines.push(format!("**Health:** {}", health));
    }
    if let Some(ref url) = project.url {
        lines.push(format!("**URL:** {}", url));
    }

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
    parts.push(format!("[id: {}]", update.id));

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
    parts.push(format!("[id: {}]", milestone.id));
    parts.join(" ")
}

// ---- #21: Initiatives ----

pub fn format_initiative(initiative: &Initiative) -> String {
    let mut parts = vec![format!("{} [{}]", initiative.name, initiative.id)];
    let mut meta = Vec::new();
    if let Some(ref status) = initiative.status {
        meta.push(status.clone());
    }
    if let Some(ref owner) = initiative.owner {
        meta.push(format!("owner: {}", owner.display_name));
    }
    if !meta.is_empty() {
        parts.push(format!("({})", meta.join(", ")));
    }
    if let Some(ref target) = initiative.target_date {
        parts.push(format!("target: {}", target));
    }
    if let Some(ref projects) = initiative.projects {
        if !projects.nodes.is_empty() {
            let names: Vec<&str> = projects.nodes.iter().map(|p| p.name.as_str()).collect();
            parts.push(format!("projects: {}", names.join(", ")));
        }
    }
    if let Some(ref desc) = initiative.description {
        if !desc.is_empty() {
            let truncated = if desc.chars().count() > 100 {
                let t: String = desc.chars().take(100).collect();
                format!("{}...", t)
            } else {
                desc.clone()
            };
            parts.push(format!("- {}", truncated));
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
    parts.push(format!("[id: {}]", attachment.id));
    parts.join(" ")
}

// ---- #25: Custom Views ----

pub fn format_custom_view(view: &CustomView) -> String {
    let mut parts = vec![format!("{} [{}]", view.name, view.id)];
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
    parts.push(format!("[id: {}]", favorite.id));
    parts.join(" ")
}

// ---- #29: Templates ----

pub fn format_template(template: &Template) -> String {
    let mut parts = vec![format!("{} [{}]", template.name, template.id)];
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
    if let (Some(from), Some(to)) = (&entry.from_state, &entry.to_state) {
        parts.push(format!("{} -> {}", from.name, to.name));
    } else if let Some(ref to) = entry.to_state {
        parts.push(format!("-> {}", to.name));
    }

    // Label changes
    if let Some(ref added) = entry.added_labels {
        if !added.is_empty() {
            let labels = added
                .iter()
                .map(|l| l.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("+labels: {}", labels));
        }
    }
    if let Some(ref removed) = entry.removed_labels {
        if !removed.is_empty() {
            let labels = removed
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
    format!(
        "{} | {} | {} [{}] [id: {}]",
        label, url, enabled, resources, webhook.id
    )
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
    if let Some(ref status) = initiative.status {
        lines.push(format!("Status: {}", status));
    }
    if let Some(ref owner) = initiative.owner {
        let owner_str = if let Some(ref email) = owner.email {
            format!("{} <{}>", owner.display_name, email)
        } else {
            owner.display_name.clone()
        };
        lines.push(format!("Owner: {}", owner_str));
    }
    if let Some(ref target) = initiative.target_date {
        lines.push(format!("Target: {}", target));
    }
    if let Some(ref started) = initiative.started_at {
        lines.push(format!("Started: {}", format_date(started)));
    }
    if let Some(ref completed) = initiative.completed_at {
        lines.push(format!("Completed: {}", format_date(completed)));
    }
    if let Some(ref url) = initiative.url {
        lines.push(format!("URL: {}", url));
    }
    if let Some(ref projects) = initiative.projects {
        if !projects.nodes.is_empty() {
            let names: Vec<&str> = projects.nodes.iter().map(|p| p.name.as_str()).collect();
            lines.push(format!("Projects: {}", names.join(", ")));
        }
    }
    if let Some(ref desc) = initiative.description {
        if !desc.is_empty() {
            lines.push(format!("Description: {}", desc));
        }
    }
    lines.join("\n")
}

// ---- New entity formatters ----

/// Format a comment with full detail for list_comments.
pub fn format_comment_detail(comment: &Comment) -> String {
    let date = format_date(&comment.created_at);
    let user = comment
        .user
        .as_ref()
        .map(|u| u.display_name.as_str())
        .unwrap_or("unknown");
    let resolved = if comment.resolved_at.is_some() {
        " [resolved]"
    } else {
        ""
    };
    let thread = comment
        .parent
        .as_ref()
        .map(|p| format!(" (reply to {})", p.id))
        .unwrap_or_default();
    let url = comment
        .url
        .as_ref()
        .map(|u| format!("\n  {}", u))
        .unwrap_or_default();
    format!(
        "**{} ({}){}{}:**\n{}{}\n[id: {}]",
        date, user, resolved, thread, comment.body, url, comment.id
    )
}

/// Format an agent session as a one-line summary.
pub fn format_agent_session_summary(session: &AgentSession) -> String {
    let status = session.status.as_deref().unwrap_or("unknown");
    let issue = session
        .issue
        .as_ref()
        .map(|i| format!(" on [{}] {}", i.identifier, i.title))
        .unwrap_or_default();
    let date = session
        .created_at
        .as_deref()
        .map(|d| format!(" ({})", format_date(d)))
        .unwrap_or_default();
    format!("[{}]{}{} [id: {}]", status, issue, date, session.id)
}

/// Format an agent session with full detail.
pub fn format_agent_session_detail(session: &AgentSession) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "# Agent Session {}",
        session.status.as_deref().unwrap_or("unknown")
    ));
    lines.push(String::new());
    lines.push(format!("**ID:** {}", session.id));
    if let Some(ref status) = session.status {
        lines.push(format!("**Status:** {}", status));
    }
    if let Some(ref issue) = session.issue {
        lines.push(format!("**Issue:** [{}] {}", issue.identifier, issue.title));
    }
    if let Some(ref comment) = session.comment {
        lines.push(format!("**Comment:** {}", comment.id));
    }
    if let Some(ref created) = session.created_at {
        lines.push(format!("**Created:** {}", format_date(created)));
    }
    if let Some(ref started) = session.started_at {
        lines.push(format!("**Started:** {}", format_date(started)));
    }
    if let Some(ref ended) = session.ended_at {
        lines.push(format!("**Ended:** {}", format_date(ended)));
    }
    if let Some(ref url) = session.url {
        lines.push(format!("**URL:** {}", url));
    }
    if let Some(ref summary) = session.summary {
        lines.push(format!("**Summary:** {}", summary));
    }
    if let Some(ref activities) = session.activities {
        if !activities.nodes.is_empty() {
            lines.push(String::new());
            lines.push(format!("## Activities ({})", activities.nodes.len()));
            for activity in &activities.nodes {
                let date = activity
                    .created_at
                    .as_deref()
                    .map(|d| format_date(d))
                    .unwrap_or("?");
                let ephemeral = if activity.ephemeral.unwrap_or(false) {
                    " [ephemeral]"
                } else {
                    ""
                };
                lines.push(format!("- {} {}{}", date, activity.id, ephemeral));
            }
        }
    }
    lines.join("\n")
}

/// Format a customer as a one-line summary.
pub fn format_customer_summary(customer: &Customer) -> String {
    let mut parts = vec![customer.name.clone()];
    if let Some(ref status) = customer.status {
        if let Some(ref name) = status.display_name {
            parts.push(format!("[{}]", name));
        }
    }
    if let Some(ref domains) = customer.domains {
        if !domains.is_empty() {
            parts.push(format!("({})", domains.join(", ")));
        }
    }
    if let Some(rev) = customer.revenue {
        if rev > 0.0 {
            parts.push(format!("rev: ${}", rev));
        }
    }
    if let Some(ref owner) = customer.owner {
        parts.push(format!("owner: {}", owner.display_name));
    }
    parts.push(format!("[id: {}]", customer.id));
    parts.join(" ")
}

/// Format a customer with full detail.
pub fn format_customer_detail(customer: &Customer) -> String {
    let mut lines = vec![format!("# {}", customer.name)];
    lines.push(String::new());
    lines.push(format!("**ID:** {}", customer.id));
    if let Some(ref status) = customer.status {
        if let Some(ref name) = status.display_name {
            lines.push(format!("**Status:** {}", name));
        }
    }
    if let Some(ref tier) = customer.tier {
        if let Some(ref name) = tier.name {
            lines.push(format!("**Tier:** {}", name));
        }
    }
    if let Some(ref domains) = customer.domains {
        if !domains.is_empty() {
            lines.push(format!("**Domains:** {}", domains.join(", ")));
        }
    }
    if let Some(ref owner) = customer.owner {
        lines.push(format!("**Owner:** {}", owner.display_name));
    }
    if let Some(rev) = customer.revenue {
        lines.push(format!("**Revenue:** ${}", rev));
    }
    if let Some(size) = customer.size {
        lines.push(format!("**Size:** {}", size));
    }
    if let Some(ref needs) = customer.needs {
        if !needs.is_empty() {
            lines.push(String::new());
            lines.push("## Needs".to_string());
            for need in needs {
                lines.push(format_customer_need(need));
            }
        }
    }
    lines.join("\n")
}

/// Format a customer need.
pub fn format_customer_need(need: &CustomerNeed) -> String {
    let mut parts = Vec::new();
    if let Some(priority) = need.priority {
        parts.push(format!("[p{}]", priority));
    }
    if let Some(ref body) = need.body {
        let truncated = if body.chars().count() > 100 {
            let t: String = body.chars().take(100).collect();
            format!("{}...", t)
        } else {
            body.clone()
        };
        parts.push(truncated);
    }
    if let Some(ref issue) = need.issue {
        parts.push(format!("-> [{}] {}", issue.identifier, issue.title));
    }
    if let Some(ref customer) = need.customer {
        parts.push(format!("({})", customer.name));
    }
    parts.push(format!("[id: {}]", need.id));
    parts.join(" ")
}

/// Format an initiative status update.
pub fn format_initiative_update(update: &InitiativeStatusUpdate) -> String {
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
    parts.push(format!("[id: {}]", update.id));
    parts.join(" ")
}

/// Format an initiative-to-project link.
pub fn format_initiative_to_project(link: &InitiativeToProject) -> String {
    let init = link
        .initiative
        .as_ref()
        .map(|i| i.name.as_str())
        .unwrap_or("?");
    let proj = link
        .project
        .as_ref()
        .map(|p| p.name.as_str())
        .unwrap_or("?");
    format!("{} -> {} [id: {}]", init, proj, link.id)
}

/// Format a project relation.
pub fn format_project_relation(relation: &ProjectRelation) -> String {
    let proj = relation
        .project
        .as_ref()
        .map(|p| p.name.as_str())
        .unwrap_or("?");
    let related = relation
        .related_project
        .as_ref()
        .map(|p| p.name.as_str())
        .unwrap_or("?");
    let rel_type = relation.relation_type.as_deref().unwrap_or("related");
    format!("{} --{}-- {} [id: {}]", proj, rel_type, related, relation.id)
}

/// Format a release as a one-line summary.
pub fn format_release_summary(release: &Release) -> String {
    let name = release.name.as_deref().unwrap_or("Unnamed");
    let mut parts = vec![name.to_string()];
    if let Some(ref version) = release.version {
        parts.push(format!("v{}", version));
    }
    if let Some(ref stage) = release.stage {
        parts.push(format!("[{}]", stage.name));
    }
    if let Some(ref pipeline) = release.pipeline {
        parts.push(format!("({})", pipeline.name));
    }
    if let Some(ref target) = release.target_date {
        parts.push(format!("target: {}", target));
    }
    parts.push(format!("[id: {}]", release.id));
    parts.join(" ")
}

/// Format a release with full detail.
pub fn format_release_detail(release: &Release) -> String {
    let name = release.name.as_deref().unwrap_or("Unnamed");
    let mut lines = vec![format!("# {}", name)];
    lines.push(String::new());
    lines.push(format!("**ID:** {}", release.id));
    if let Some(ref version) = release.version {
        lines.push(format!("**Version:** {}", version));
    }
    if let Some(ref stage) = release.stage {
        lines.push(format!("**Stage:** {}", stage.name));
    }
    if let Some(ref pipeline) = release.pipeline {
        lines.push(format!("**Pipeline:** {}", pipeline.name));
    }
    if let Some(ref start) = release.start_date {
        lines.push(format!("**Start:** {}", start));
    }
    if let Some(ref target) = release.target_date {
        lines.push(format!("**Target:** {}", target));
    }
    if let Some(ref url) = release.url {
        lines.push(format!("**URL:** {}", url));
    }
    lines.join("\n")
}

/// Format a project search result.
pub fn format_project_search_result(project: &ProjectSearchResult) -> String {
    let progress = project.progress.unwrap_or(0.0);
    let pct = (progress * 100.0).round() as i32;
    let state = project.state.as_deref().unwrap_or("unknown");
    let mut parts = vec![format!("{} [{}] - {}% complete", project.name, state, pct)];
    if let Some(ref lead) = project.lead {
        parts.push(format!("  Lead: {}", lead.display_name));
    }
    if let Some(ref url) = project.url {
        parts.push(format!("  URL: {}", url));
    }
    parts.join("\n")
}

// ---- Phase 2 (Complete Coverage): New Formatters ----

pub fn format_customer_status(s: &CustomerStatusFull) -> String {
    let name = s.name.as_deref().unwrap_or("Unnamed");
    let mut parts = vec![format!("**{}** [id: {}]", name, s.id)];
    if let Some(ref color) = s.color {
        parts.push(format!("Color: {}", color));
    }
    if let Some(ref desc) = s.description {
        parts.push(format!("Description: {}", desc));
    }
    if let Some(ref dn) = s.display_name {
        parts.push(format!("Display: {}", dn));
    }
    if let Some(pos) = s.position {
        parts.push(format!("Position: {}", pos));
    }
    parts.join("\n")
}

pub fn format_customer_tier(t: &CustomerTierFull) -> String {
    let name = t.name.as_deref().unwrap_or("Unnamed");
    let mut parts = vec![format!("**{}** [id: {}]", name, t.id)];
    if let Some(ref color) = t.color {
        parts.push(format!("Color: {}", color));
    }
    if let Some(ref desc) = t.description {
        parts.push(format!("Description: {}", desc));
    }
    if let Some(ref dn) = t.display_name {
        parts.push(format!("Display: {}", dn));
    }
    if let Some(pos) = t.position {
        parts.push(format!("Position: {}", pos));
    }
    parts.join("\n")
}

pub fn format_release_pipeline(p: &ReleasePipelineFull) -> String {
    let mut parts = vec![format!("**{}** [id: {}]", p.name, p.id)];
    if let Some(ref t) = p.type_ {
        parts.push(format!("Type: {}", t));
    }
    if let Some(ref slug) = p.slug_id {
        parts.push(format!("Slug: {}", slug));
    }
    if let Some(ref patterns) = p.include_path_patterns {
        if !patterns.is_empty() {
            parts.push(format!("Paths: {}", patterns.join(", ")));
        }
    }
    parts.join("\n")
}

pub fn format_release_stage(s: &ReleaseStageFull) -> String {
    let mut parts = vec![format!("**{}** [id: {}]", s.name, s.id)];
    if let Some(ref color) = s.color {
        parts.push(format!("Color: {}", color));
    }
    if let Some(ref t) = s.type_ {
        parts.push(format!("Type: {}", t));
    }
    if let Some(pos) = s.position {
        parts.push(format!("Position: {}", pos));
    }
    if let Some(frozen) = s.frozen {
        parts.push(format!("Frozen: {}", frozen));
    }
    parts.join("\n")
}

pub fn format_issue_to_release(link: &IssueToRelease) -> String {
    let issue = link.issue.as_ref().map(|i| i.identifier.as_str()).unwrap_or("?");
    let release = link.release.as_ref().and_then(|r| r.name.as_deref()).unwrap_or("?");
    format!("{} → {} [id: {}]", issue, release, link.id)
}

pub fn format_project_status(s: &ProjectStatusFull) -> String {
    let mut parts = vec![format!("**{}** [id: {}]", s.name, s.id)];
    if let Some(ref color) = s.color {
        parts.push(format!("Color: {}", color));
    }
    if let Some(ref t) = s.type_ {
        parts.push(format!("Type: {}", t));
    }
    if let Some(ref desc) = s.description {
        parts.push(format!("Description: {}", desc));
    }
    if let Some(pos) = s.position {
        parts.push(format!("Position: {}", pos));
    }
    if let Some(indef) = s.indefinite {
        parts.push(format!("Indefinite: {}", indef));
    }
    parts.join("\n")
}

pub fn format_project_label(l: &ProjectLabel) -> String {
    let mut parts = vec![format!("**{}** [id: {}]", l.name, l.id)];
    if let Some(ref color) = l.color {
        parts.push(format!("Color: {}", color));
    }
    if let Some(ref desc) = l.description {
        parts.push(format!("Description: {}", desc));
    }
    if let Some(is_group) = l.is_group {
        parts.push(format!("Group: {}", is_group));
    }
    if let Some(ref parent) = l.parent {
        parts.push(format!("Parent: {}", parent.name));
    }
    parts.join("\n")
}

pub fn format_team_membership(m: &TeamMembership) -> String {
    let user = m.user.as_ref().map(|u| u.display_name.as_str()).unwrap_or("?");
    let team = m.team.as_ref().map(|t| t.key.as_str()).unwrap_or("?");
    let owner = m.owner.unwrap_or(false);
    format!("{} → {} (owner: {}) [id: {}]", user, team, owner, m.id)
}

pub fn format_notification_subscription(s: &NotificationSubscription) -> String {
    let subscriber = s.subscriber.as_ref().map(|u| u.display_name.as_str()).unwrap_or("?");
    let active = s.active.unwrap_or(true);
    let view_type = s.context_view_type.as_deref().unwrap_or("?");
    format!("{} - context: {} (active: {}) [id: {}]", subscriber, view_type, active, s.id)
}

pub fn format_entity_external_link(l: &EntityExternalLink) -> String {
    let mut parts = vec![format!("**{}** - {} [id: {}]", l.label, l.url, l.id)];
    if let Some(ref creator) = l.creator {
        parts.push(format!("Creator: {}", creator.display_name));
    }
    if let Some(order) = l.sort_order {
        parts.push(format!("Order: {}", order));
    }
    parts.join("\n")
}

pub fn format_emoji(e: &Emoji) -> String {
    let mut parts = vec![format!("**{}** [id: {}]", e.name, e.id)];
    if let Some(ref url) = e.url {
        parts.push(format!("URL: {}", url));
    }
    if let Some(ref source) = e.source {
        parts.push(format!("Source: {}", source));
    }
    parts.join("\n")
}

pub fn format_initiative_relation(r: &InitiativeRelation) -> String {
    let init = r.initiative.as_ref().map(|i| i.name.as_str()).unwrap_or("?");
    let related = r.related_initiative.as_ref().map(|i| i.name.as_str()).unwrap_or("?");
    format!("{} ↔ {} [id: {}]", init, related, r.id)
}

pub fn format_time_schedule(s: &TimeSchedule) -> String {
    let name = s.name.as_deref().unwrap_or("Unnamed");
    let mut parts = vec![format!("**{}** [id: {}]", name, s.id)];
    if let Some(ref ext_id) = s.external_id {
        parts.push(format!("External ID: {}", ext_id));
    }
    if let Some(ref ext_url) = s.external_url {
        parts.push(format!("External URL: {}", ext_url));
    }
    parts.join("\n")
}

pub fn format_triage_responsibility(r: &TriageResponsibility) -> String {
    let action = r.action.as_deref().unwrap_or("?");
    let team = r.team.as_ref().map(|t| t.key.as_str()).unwrap_or("?");
    format!("Team {} - action: {} [id: {}]", team, action, r.id)
}

pub fn format_git_automation_state(s: &GitAutomationState) -> String {
    let event = s.event.as_deref().unwrap_or("?");
    let team = s.team.as_ref().map(|t| t.key.as_str()).unwrap_or("?");
    let state = s.state.as_ref().map(|s| s.name.as_str()).unwrap_or("none");
    format!("Team {} - event: {} → state: {} [id: {}]", team, event, state, s.id)
}

pub fn format_git_automation_target_branch(b: &GitAutomationTargetBranch) -> String {
    let pattern = b.branch_pattern.as_deref().unwrap_or("?");
    let team = b.team.as_ref().map(|t| t.key.as_str()).unwrap_or("?");
    let regex = b.is_regex.unwrap_or(false);
    format!("Team {} - pattern: {} (regex: {}) [id: {}]", team, pattern, regex, b.id)
}

pub fn format_email_intake_address(a: &EmailIntakeAddress) -> String {
    let addr = a.address.as_deref().unwrap_or("?");
    let enabled = a.enabled.unwrap_or(false);
    let mut parts = vec![format!("**{}** (enabled: {}) [id: {}]", addr, enabled, a.id)];
    if let Some(ref name) = a.sender_name {
        parts.push(format!("Sender: {}", name));
    }
    parts.join("\n")
}

pub fn format_organization(o: &Organization) -> String {
    let mut parts = vec![format!("**{}** [id: {}]", o.name, o.id)];
    if let Some(ref key) = o.url_key {
        parts.push(format!("URL Key: {}", key));
    }
    if let Some(ref logo) = o.logo_url {
        parts.push(format!("Logo: {}", logo));
    }
    if let Some(count) = o.user_count {
        parts.push(format!("Users: {}", count));
    }
    if let Some(ref created) = o.created_at {
        parts.push(format!("Created: {}", format_date(created)));
    }
    parts.join("\n")
}

pub fn format_rate_limit_status(r: &RateLimitStatus) -> String {
    let mut parts = vec![format!("Rate limit kind: {}", r.kind)];
    for limit in &r.limits {
        let mut line = format!("  {} —", limit.limit_type);
        if let Some(remaining) = limit.remaining_amount {
            line.push_str(&format!(" remaining: {:.0}", remaining));
        }
        if let Some(allowed) = limit.allowed_amount {
            line.push_str(&format!(" / {:.0}", allowed));
        }
        if let Some(ref reset) = limit.reset {
            line.push_str(&format!(" (resets: {})", reset));
        }
        parts.push(line);
    }
    parts.join("\n")
}

pub fn format_application_info(a: &ApplicationInfo) -> String {
    let mut parts = vec![format!("**{}** [client: {}]", a.name, a.client_id)];
    if let Some(ref desc) = a.description {
        parts.push(format!("Description: {}", desc));
    }
    if let Some(ref dev) = a.developer {
        parts.push(format!("Developer: {}", dev));
    }
    if let Some(ref url) = a.developer_url {
        parts.push(format!("Developer URL: {}", url));
    }
    if let Some(ref img) = a.image_url {
        parts.push(format!("Image: {}", img));
    }
    parts.join("\n")
}

pub fn format_external_user(u: &ExternalUser) -> String {
    let name = u.display_name.as_deref().or(u.name.as_deref()).unwrap_or("?");
    let email = u.email.as_deref().unwrap_or("");
    if email.is_empty() {
        format!("{} [id: {}]", name, u.id)
    } else {
        format!("{} <{}> [id: {}]", name, email, u.id)
    }
}

pub fn format_priority_value(p: &IssuePriorityValue) -> String {
    format!("{}: {} ({})", p.priority, p.label, match p.priority {
        0 => "No priority",
        1 => "Urgent",
        2 => "High",
        3 => "Medium",
        4 => "Low",
        _ => "Unknown",
    })
}

pub fn format_document_content_history_entry(e: &DocumentContentHistoryEntry) -> String {
    let date = e.created_at.as_deref().map(format_date).unwrap_or("?");
    let actors = e.actor_ids.as_ref()
        .map(|ids| ids.join(", "))
        .unwrap_or_else(|| "unknown".into());
    format!("{} by {} [id: {}]", date, actors, e.id)
}
