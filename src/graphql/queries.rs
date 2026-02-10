/// Viewer (authenticated user) query.
pub const VIEWER: &str = r#"
query {
    viewer {
        id
        displayName
        email
    }
}
"#;

/// List issues with filters and pagination.
pub const LIST_ISSUES: &str = r#"
query ListIssues($first: Int!, $after: String, $filter: IssueFilter, $orderBy: PaginationOrderBy) {
    issues(first: $first, after: $after, filter: $filter, orderBy: $orderBy) {
        nodes {
            id
            identifier
            title
            priority
            url
            state { id name type color }
            assignee { id displayName email }
            labels { nodes { id name } }
        }
        pageInfo {
            hasNextPage
            endCursor
        }
    }
}
"#;

/// Full-text search issues.
pub const SEARCH_ISSUES: &str = r#"
query SearchIssues($query: String!, $first: Int, $filter: IssueFilter) {
    searchIssues(term: $query, first: $first, filter: $filter) {
        nodes {
            id
            identifier
            title
            priority
            url
            state { id name type color }
            assignee { id displayName email }
            labels { nodes { id name } }
        }
        pageInfo {
            hasNextPage
            endCursor
        }
    }
}
"#;

/// Get a single issue with full details.
pub const GET_ISSUE: &str = r#"
query GetIssue($id: String!) {
    issue(id: $id) {
        id
        identifier
        title
        description
        priority
        estimate
        dueDate
        branchName
        createdAt
        updatedAt
        url
        state { id name type color }
        assignee { id displayName email }
        team { id key name }
        project { id name state progress }
        labels { nodes { id name } }
        parent { identifier title }
        children { nodes { identifier title } }
        comments { nodes { id body createdAt user { displayName } } }
    }
}
"#;

/// List issues for the authenticated user (my_issues).
pub const MY_ISSUES: &str = r#"
query MyIssues($first: Int!, $filter: IssueFilter) {
    issues(first: $first, filter: $filter) {
        nodes {
            id
            identifier
            title
            priority
            url
            state { id name type color }
            assignee { id displayName email }
            labels { nodes { id name } }
        }
        pageInfo {
            hasNextPage
            endCursor
        }
    }
}
"#;

/// List teams.
pub const LIST_TEAMS: &str = r#"
query ListTeams {
    teams(first: 100) {
        nodes {
            id
            key
            name
        }
    }
}
"#;

/// List teams with member counts.
pub const LIST_TEAMS_WITH_MEMBERS: &str = r#"
query ListTeamsWithMembers {
    teams(first: 100) {
        nodes {
            id
            key
            name
            members { nodes { id } }
        }
    }
}
"#;

/// List projects.
pub const LIST_PROJECTS: &str = r#"
query ListProjects($first: Int!, $filter: ProjectFilter) {
    projects(first: $first, filter: $filter) {
        nodes {
            id
            name
            state
            progress
        }
    }
}
"#;

/// List users.
pub const LIST_USERS: &str = r#"
query ListUsers($first: Int!) {
    users(first: $first, includeDisabled: false) {
        nodes {
            id
            displayName
            email
            admin
            guest
            active
        }
    }
}
"#;

/// List workflow states.
pub const LIST_STATES: &str = r#"
query ListStates($first: Int!, $filter: WorkflowStateFilter) {
    workflowStates(first: $first, filter: $filter) {
        nodes {
            id
            name
            type
            color
            team { id key name }
        }
    }
}
"#;

/// Resolve team key to ID.
pub const RESOLVE_TEAM: &str = r#"
query ResolveTeam($filter: TeamFilter!) {
    teams(filter: $filter) {
        nodes { id key name }
    }
}
"#;

/// Resolve user email to ID.
pub const RESOLVE_USER: &str = r#"
query ResolveUser($filter: UserFilter!) {
    users(filter: $filter) {
        nodes { id displayName email }
    }
}
"#;

/// Resolve workflow state by name and team.
pub const RESOLVE_STATE: &str = r#"
query ResolveState($filter: WorkflowStateFilter!) {
    workflowStates(filter: $filter) {
        nodes { id name type }
    }
}
"#;

/// Create an issue.
pub const CREATE_ISSUE: &str = r#"
mutation CreateIssue($input: IssueCreateInput!) {
    issueCreate(input: $input) {
        success
        issue {
            id
            identifier
            title
            description
            priority
            estimate
            dueDate
            branchName
            createdAt
            updatedAt
            url
            state { id name type color }
            assignee { id displayName email }
            team { id key name }
            project { id name state progress }
            labels { nodes { id name } }
            parent { identifier title }
            children { nodes { identifier title } }
            comments { nodes { id body createdAt user { displayName } } }
        }
    }
}
"#;

/// Update an issue.
pub const UPDATE_ISSUE: &str = r#"
mutation UpdateIssue($id: String!, $input: IssueUpdateInput!) {
    issueUpdate(id: $id, input: $input) {
        success
        issue {
            id
            identifier
            title
            description
            priority
            estimate
            dueDate
            branchName
            createdAt
            updatedAt
            url
            state { id name type color }
            assignee { id displayName email }
            team { id key name }
            project { id name state progress }
            labels { nodes { id name } }
            parent { identifier title }
            children { nodes { identifier title } }
            comments { nodes { id body createdAt user { displayName } } }
        }
    }
}
"#;

/// Add a comment.
pub const ADD_COMMENT: &str = r#"
mutation AddComment($input: CommentCreateInput!) {
    commentCreate(input: $input) {
        success
        comment {
            id
            body
            createdAt
            user { displayName }
        }
    }
}
"#;
