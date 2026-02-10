use serde::Serialize;

/// Generic string comparison filter used throughout Linear's API.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StringFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq_ignore_case: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_ignore_case: Option<String>,
}

impl StringFilter {
    pub fn eq_ignore_case(val: impl Into<String>) -> Self {
        Self {
            eq: None,
            eq_ignore_case: Some(val.into()),
            contains_ignore_case: None,
        }
    }

    pub fn eq_exact(val: impl Into<String>) -> Self {
        Self {
            eq: Some(val.into()),
            eq_ignore_case: None,
            contains_ignore_case: None,
        }
    }

    pub fn contains_ignore_case(val: impl Into<String>) -> Self {
        Self {
            eq: None,
            eq_ignore_case: None,
            contains_ignore_case: Some(val.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NumberFilter {
    pub eq: i32,
}

/// Issue filter matching Linear's IssueFilter input type.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<TeamFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<AssigneeFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<StateFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectNameFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<LabelsFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<NumberFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<IssueFilter>>,
}

impl IssueFilter {
    pub fn combine(filters: Vec<IssueFilter>) -> Option<IssueFilter> {
        match filters.len() {
            0 => None,
            1 => Some(filters.into_iter().next().unwrap()),
            _ => Some(IssueFilter {
                and: Some(filters),
                ..Default::default()
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TeamFilter {
    pub key: Option<StringFilter>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssigneeFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<AssigneeFieldFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<StringFilter>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssigneeFieldFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<StringFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<StringFilter>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StateFilter {
    pub name: Option<StringFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub state_type: Option<StateTypeFilter>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StateTypeFilter {
    pub nin: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectNameFilter {
    pub name: StringFilter,
}

#[derive(Debug, Clone, Serialize)]
pub struct LabelsFilter {
    pub some: LabelNameFilter,
}

#[derive(Debug, Clone, Serialize)]
pub struct LabelNameFilter {
    pub name: StringFilter,
}

/// Workflow state filter for resolve and list queries.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStateFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<StringFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<TeamFilter>,
}

/// User filter for resolve queries.
#[derive(Debug, Clone, Serialize)]
pub struct UserFilter {
    pub email: StringFilter,
}

/// Project filter for list queries.
#[derive(Debug, Clone, Serialize)]
pub struct ProjectFilter {
    pub state: StringFilter,
}

// Builder helpers for common filter patterns.

pub fn team_filter(key: &str) -> IssueFilter {
    IssueFilter {
        team: Some(TeamFilter {
            key: Some(StringFilter::eq_ignore_case(key)),
        }),
        ..Default::default()
    }
}

pub fn assignee_filter(assignee: &str) -> IssueFilter {
    IssueFilter {
        assignee: Some(AssigneeFilter {
            or: Some(vec![
                AssigneeFieldFilter {
                    email: Some(StringFilter::eq_ignore_case(assignee)),
                    display_name: None,
                },
                AssigneeFieldFilter {
                    email: None,
                    display_name: Some(StringFilter::eq_ignore_case(assignee)),
                },
            ]),
            id: None,
        }),
        ..Default::default()
    }
}

pub fn status_filter(status: &str) -> IssueFilter {
    IssueFilter {
        state: Some(StateFilter {
            name: Some(StringFilter::eq_ignore_case(status)),
            state_type: None,
        }),
        ..Default::default()
    }
}

pub fn project_filter(project: &str) -> IssueFilter {
    IssueFilter {
        project: Some(ProjectNameFilter {
            name: StringFilter::contains_ignore_case(project),
        }),
        ..Default::default()
    }
}

pub fn label_filter(label: &str) -> IssueFilter {
    IssueFilter {
        labels: Some(LabelsFilter {
            some: LabelNameFilter {
                name: StringFilter::eq_ignore_case(label),
            },
        }),
        ..Default::default()
    }
}

pub fn priority_filter(priority: i32) -> IssueFilter {
    IssueFilter {
        priority: Some(NumberFilter { eq: priority }),
        ..Default::default()
    }
}

pub fn viewer_filter(viewer_id: &str) -> IssueFilter {
    IssueFilter {
        assignee: Some(AssigneeFilter {
            or: None,
            id: Some(StringFilter::eq_exact(viewer_id)),
        }),
        ..Default::default()
    }
}

pub fn exclude_completed_filter() -> IssueFilter {
    IssueFilter {
        state: Some(StateFilter {
            name: None,
            state_type: Some(StateTypeFilter {
                nin: vec!["completed".to_string(), "canceled".to_string()],
            }),
        }),
        ..Default::default()
    }
}
