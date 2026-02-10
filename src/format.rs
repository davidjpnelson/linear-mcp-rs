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

    // Dates — extract just the date portion
    if let Some(ref created) = issue.created_at {
        lines.push(format!("**Created:** {}", format_date(created)));
    }
    if let Some(ref updated) = issue.updated_at {
        lines.push(format!("**Updated:** {}", format_date(updated)));
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
    format!("{} [{}] - {}% complete", project.name, project.state, pct)
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

pub fn format_pagination(has_next_page: bool, count: usize) -> String {
    let s = if count != 1 { "s" } else { "" };
    if has_next_page {
        format!("\n---\nShowing {} result{}. More available.", count, s)
    } else {
        format!("\n---\nShowing all {} result{}.", count, s)
    }
}

fn format_date(iso: &str) -> &str {
    // Extract YYYY-MM-DD from ISO timestamp
    if iso.len() >= 10 {
        &iso[..10]
    } else {
        iso
    }
}
