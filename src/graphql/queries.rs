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
            estimate
            dueDate
            url
            state { id name type color }
            assignee { id displayName email }
            team { id key name }
            project { id name state progress }
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
query SearchIssues($query: String!, $first: Int, $after: String, $filter: IssueFilter) {
    searchIssues(term: $query, first: $first, after: $after, filter: $filter) {
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
        startedAt
        completedAt
        canceledAt
        url
        state { id name type color }
        assignee { id displayName email }
        creator { displayName email }
        team { id key name }
        project { id name state progress }
        cycle { id number name }
        labels { nodes { id name } }
        parent { identifier title }
        children { nodes { identifier title } }
        relations { nodes { id type relatedIssue { identifier title } } }
        subscribers { nodes { displayName email } }
        comments { nodes { id body createdAt user { displayName } } }
    }
}
"#;

/// List issues for the authenticated user (my_issues).
pub const MY_ISSUES: &str = r#"
query MyIssues($first: Int!, $after: String, $filter: IssueFilter) {
    issues(first: $first, after: $after, filter: $filter) {
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
            description
            url
            startDate
            targetDate
            lead { displayName email }
            teams { nodes { id key name } }
            members { nodes { displayName } }
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

/// Update a comment.
pub const UPDATE_COMMENT: &str = r#"
mutation UpdateComment($id: String!, $input: CommentUpdateInput!) {
    commentUpdate(id: $id, input: $input) {
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

/// Delete a comment.
pub const DELETE_COMMENT: &str = r#"
mutation DeleteComment($id: String!) {
    commentDelete(id: $id) {
        success
    }
}
"#;

/// List cycles for a team.
pub const LIST_CYCLES: &str = r#"
query ListCycles($teamId: String!, $first: Int!) {
    team(id: $teamId) {
        cycles(first: $first, orderBy: createdAt) {
            nodes {
                id
                number
                name
                startsAt
                endsAt
                completedAt
                progress
            }
        }
    }
}
"#;

/// Get a single cycle by ID.
pub const GET_CYCLE: &str = r#"
query GetCycle($id: String!) {
    cycle(id: $id) {
        id
        number
        name
        startsAt
        endsAt
        completedAt
        progress
    }
}
"#;

/// List labels (issue labels).
pub const LIST_LABELS: &str = r#"
query ListLabels($first: Int!, $filter: IssueLabelFilter) {
    issueLabels(first: $first, filter: $filter) {
        nodes {
            id
            name
        }
    }
}
"#;

/// Create a label.
pub const CREATE_LABEL: &str = r#"
mutation CreateLabel($input: IssueLabelCreateInput!) {
    issueLabelCreate(input: $input) {
        success
        issueLabel {
            id
            name
        }
    }
}
"#;

/// Resolve labels by name (for matching comma-separated label names to IDs).
pub const RESOLVE_LABELS: &str = r#"
query ResolveLabels($filter: IssueLabelFilter) {
    issueLabels(first: 100, filter: $filter) {
        nodes {
            id
            name
        }
    }
}
"#;

/// Resolve project by name.
pub const RESOLVE_PROJECT: &str = r#"
query ResolveProject($filter: ProjectFilter) {
    projects(first: 5, filter: $filter) {
        nodes {
            id
            name
        }
    }
}
"#;

/// Create an issue relation.
pub const CREATE_ISSUE_RELATION: &str = r#"
mutation CreateIssueRelation($input: IssueRelationCreateInput!) {
    issueRelationCreate(input: $input) {
        success
        issueRelation {
            id
            type
            issue { identifier title }
            relatedIssue { identifier title }
        }
    }
}
"#;

/// Delete an issue relation.
pub const DELETE_ISSUE_RELATION: &str = r#"
mutation DeleteIssueRelation($id: String!) {
    issueRelationDelete(id: $id) {
        success
    }
}
"#;

/// Archive an issue.
pub const ARCHIVE_ISSUE: &str = r#"
mutation ArchiveIssue($id: String!) {
    issueArchive(id: $id) {
        success
    }
}
"#;

// ---- #17: Document queries ----

/// List documents with optional filter.
pub const LIST_DOCUMENTS: &str = r#"
query ListDocuments($first: Int!) {
    documents(first: $first) {
        nodes {
            id
            title
            content
            createdAt
            updatedAt
            project { name }
            creator { displayName }
        }
    }
}
"#;

/// Get a single document by ID.
pub const GET_DOCUMENT: &str = r#"
query GetDocument($id: String!) {
    document(id: $id) {
        id
        title
        content
        createdAt
        updatedAt
        project { name }
        creator { displayName }
    }
}
"#;

/// Create a document.
pub const CREATE_DOCUMENT: &str = r#"
mutation CreateDocument($input: DocumentCreateInput!) {
    documentCreate(input: $input) {
        success
        document {
            id
            title
            content
            createdAt
            updatedAt
            project { name }
            creator { displayName }
        }
    }
}
"#;

// ---- #18: Project queries ----

/// Get a single project with full details.
pub const GET_PROJECT: &str = r#"
query GetProject($id: String!) {
    project(id: $id) {
        id
        name
        description
        state
        progress
        targetDate
        startDate
        createdAt
        updatedAt
        lead { id displayName }
        teams { nodes { id key name } }
        members { nodes { id displayName } }
    }
}
"#;

/// Create a project.
pub const CREATE_PROJECT: &str = r#"
mutation CreateProject($input: ProjectCreateInput!) {
    projectCreate(input: $input) {
        success
        project {
            id
            name
            description
            state
            progress
            targetDate
            startDate
            createdAt
            updatedAt
            lead { id displayName }
            teams { nodes { id key name } }
            members { nodes { id displayName } }
        }
    }
}
"#;

/// Update a project.
pub const UPDATE_PROJECT: &str = r#"
mutation UpdateProject($id: String!, $input: ProjectUpdateInput!) {
    projectUpdate(id: $id, input: $input) {
        success
        project {
            id
            name
            description
            state
            progress
            targetDate
            startDate
            createdAt
            updatedAt
            lead { id displayName }
            teams { nodes { id key name } }
            members { nodes { id displayName } }
        }
    }
}
"#;

// ---- #19: Project update queries ----

/// List project updates for a project.
pub const LIST_PROJECT_UPDATES: &str = r#"
query ListProjectUpdates($id: String!) {
    project(id: $id) {
        projectUpdates {
            nodes {
                id
                body
                health
                createdAt
                user { displayName }
            }
        }
    }
}
"#;

/// Create a project update.
pub const CREATE_PROJECT_UPDATE: &str = r#"
mutation CreateProjectUpdate($input: ProjectUpdateCreateInput!) {
    projectUpdateCreate(input: $input) {
        success
        projectUpdate {
            id
            body
            health
            createdAt
            user { displayName }
        }
    }
}
"#;

// ---- #20: Project milestone queries ----

/// List project milestones for a project.
pub const LIST_PROJECT_MILESTONES: &str = r#"
query ListProjectMilestones($id: String!) {
    project(id: $id) {
        projectMilestones {
            nodes {
                id
                name
                description
                targetDate
                sortOrder
            }
        }
    }
}
"#;

/// Create a project milestone.
pub const CREATE_PROJECT_MILESTONE: &str = r#"
mutation CreateProjectMilestone($input: ProjectMilestoneCreateInput!) {
    projectMilestoneCreate(input: $input) {
        success
        projectMilestone {
            id
            name
            description
            targetDate
            sortOrder
        }
    }
}
"#;

// ---- #21: Roadmap and Initiative queries ----

/// List roadmaps.
pub const LIST_ROADMAPS: &str = r#"
query ListRoadmaps($first: Int!) {
    roadmaps(first: $first) {
        nodes {
            id
            name
            description
            slugId
        }
    }
}
"#;

/// List initiatives.
pub const LIST_INITIATIVES: &str = r#"
query ListInitiatives($first: Int!) {
    initiatives(first: $first) {
        nodes {
            id
            name
            description
            status
        }
    }
}
"#;

// ---- #22: Notification queries ----

/// List notifications.
pub const LIST_NOTIFICATIONS: &str = r#"
query ListNotifications($first: Int!) {
    notifications(first: $first) {
        nodes {
            id
            type
            readAt
            createdAt
            ... on IssueNotification {
                issue { identifier title }
            }
        }
    }
}
"#;

// NOTE: verify this mutation name — Linear may use `notificationUpdate` or `notificationArchive`
/// Mark a notification as read (archive it).
pub const MARK_NOTIFICATION_READ: &str = r#"
mutation MarkNotificationRead($id: String!, $input: NotificationUpdateInput!) {
    notificationUpdate(id: $id, input: $input) {
        success
    }
}
"#;

// ---- #23: Attachment queries ----

/// List attachments for an issue.
pub const LIST_ATTACHMENTS: &str = r#"
query ListAttachments($id: String!) {
    issue(id: $id) {
        attachments {
            nodes {
                id
                title
                url
                createdAt
            }
        }
    }
}
"#;

/// Add an attachment to an issue.
pub const ADD_ATTACHMENT: &str = r#"
mutation AddAttachment($input: AttachmentCreateInput!) {
    attachmentCreate(input: $input) {
        success
        attachment {
            id
            title
            url
            createdAt
        }
    }
}
"#;

// ---- #24: Reaction queries ----

/// Add a reaction to a comment.
pub const ADD_REACTION: &str = r#"
mutation AddReaction($input: ReactionCreateInput!) {
    reactionCreate(input: $input) {
        success
        reaction {
            id
            emoji
        }
    }
}
"#;

/// Remove a reaction.
pub const REMOVE_REACTION: &str = r#"
mutation RemoveReaction($id: String!) {
    reactionDelete(id: $id) {
        success
    }
}
"#;

// ---- #25: Custom View queries ----

/// List custom views.
pub const LIST_VIEWS: &str = r#"
query ListViews($first: Int!) {
    customViews(first: $first) {
        nodes {
            id
            name
            description
            filterData
        }
    }
}
"#;

// ---- #26: Favorite queries ----

/// List favorites.
pub const LIST_FAVORITES: &str = r#"
query ListFavorites($first: Int!) {
    favorites(first: $first) {
        nodes {
            id
            type
            issue { identifier title }
            project { name }
        }
    }
}
"#;

/// Add a favorite.
pub const ADD_FAVORITE: &str = r#"
mutation AddFavorite($input: FavoriteCreateInput!) {
    favoriteCreate(input: $input) {
        success
        favorite {
            id
            type
            issue { identifier title }
            project { name }
        }
    }
}
"#;

/// Remove a favorite.
pub const REMOVE_FAVORITE: &str = r#"
mutation RemoveFavorite($id: String!) {
    favoriteDelete(id: $id) {
        success
    }
}
"#;

// ---- #29: Template queries ----

/// List templates.
// NOTE: verify this GraphQL field name — Linear may use `templates` or `issueTemplates`
pub const LIST_TEMPLATES: &str = r#"
query ListTemplates {
    templates {
        id
        name
        description
        templateData
    }
}
"#;

// ---- #30: Issue history queries ----

/// Get issue history.
pub const GET_ISSUE_HISTORY: &str = r#"
query GetIssueHistory($id: String!, $first: Int!) {
    issue(id: $id) {
        history(first: $first) {
            nodes {
                id
                createdAt
                fromState { name }
                toState { name }
                actor { displayName }
                addedLabels { nodes { name } }
                removedLabels { nodes { name } }
            }
        }
    }
}
"#;

// ---- #31: Webhook queries ----

/// List webhooks.
pub const LIST_WEBHOOKS: &str = r#"
query ListWebhooks($first: Int!) {
    webhooks(first: $first) {
        nodes {
            id
            url
            label
            enabled
            resourceTypes
        }
    }
}
"#;

/// Create a webhook.
pub const CREATE_WEBHOOK: &str = r#"
mutation CreateWebhook($input: WebhookCreateInput!) {
    webhookCreate(input: $input) {
        success
        webhook {
            id
            url
            label
            enabled
            resourceTypes
        }
    }
}
"#;

/// Delete a webhook.
pub const DELETE_WEBHOOK: &str = r#"
mutation DeleteWebhook($id: String!) {
    webhookDelete(id: $id) {
        success
    }
}
"#;

// ---- #32: Integration and Audit Log queries ----

/// List integrations.
pub const LIST_INTEGRATIONS: &str = r#"
query ListIntegrations($first: Int!) {
    integrations(first: $first) {
        nodes {
            id
            service
            createdAt
        }
    }
}
"#;

// NOTE: verify this GraphQL field name — Linear may use `auditEntries` or `auditLogs`
/// Query audit log entries.
pub const QUERY_AUDIT_LOG: &str = r#"
query QueryAuditLog($first: Int!) {
    auditEntries(first: $first) {
        nodes {
            id
            type
            createdAt
            actorId
            ip
        }
    }
}
"#;

// ---- #33: Team CRUD queries ----

/// Create a team.
pub const CREATE_TEAM: &str = r#"
mutation CreateTeam($input: TeamCreateInput!) {
    teamCreate(input: $input) {
        success
        team {
            id
            key
            name
            description
            timezone
        }
    }
}
"#;

/// Update a team.
pub const UPDATE_TEAM: &str = r#"
mutation UpdateTeam($id: String!, $input: TeamUpdateInput!) {
    teamUpdate(id: $id, input: $input) {
        success
        team {
            id
            key
            name
            description
            timezone
        }
    }
}
"#;

// ---- Phase 11: Additional tools ----

/// Archive a project.
pub const ARCHIVE_PROJECT: &str = r#"
mutation ArchiveProject($id: String!) {
    projectArchive(id: $id) {
        success
    }
}
"#;

/// Update a document.
pub const UPDATE_DOCUMENT: &str = r#"
mutation UpdateDocument($id: String!, $input: DocumentUpdateInput!) {
    documentUpdate(id: $id, input: $input) {
        success
        document {
            id
            title
            content
            createdAt
            updatedAt
            project { name }
            creator { displayName }
        }
    }
}
"#;

/// Create a cycle.
pub const CREATE_CYCLE: &str = r#"
mutation CreateCycle($input: CycleCreateInput!) {
    cycleCreate(input: $input) {
        success
        cycle {
            id
            name
            number
            startsAt
            endsAt
        }
    }
}
"#;

/// Update a label.
pub const UPDATE_LABEL: &str = r#"
mutation UpdateLabel($id: String!, $input: IssueLabelUpdateInput!) {
    issueLabelUpdate(id: $id, input: $input) {
        success
        issueLabel {
            id
            name
            color
        }
    }
}
"#;

/// Delete a label.
pub const DELETE_LABEL: &str = r#"
mutation DeleteLabel($id: String!) {
    issueLabelDelete(id: $id) {
        success
    }
}
"#;

/// Unarchive an issue.
pub const UNARCHIVE_ISSUE: &str = r#"
mutation UnarchiveIssue($id: String!) {
    issueUnarchive(id: $id) {
        success
    }
}
"#;
