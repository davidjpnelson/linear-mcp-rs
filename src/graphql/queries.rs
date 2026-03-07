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
        autoClosedAt
        autoArchivedAt
        slaBreachesAt
        slaStartedAt
        slaType
        customerTicketCount
        previousIdentifiers
        trashed
        snoozedUntilAt
        url
        state { id name type color }
        assignee { id displayName email }
        creator { displayName email }
        team { id key name }
        project { id name state progress }
        cycle { id number name }
        projectMilestone { id name }
        labels { nodes { id name } }
        parent { identifier title }
        children { nodes { identifier title } }
        relations { nodes { id type relatedIssue { identifier title } } }
        subscribers { nodes { displayName email } }
        comments { nodes { id body createdAt url resolvedAt user { displayName } parent { id } } }
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
            description
            timezone
            triageEnabled
            defaultIssueState { id name }
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
            description
            timezone
            triageEnabled
            defaultIssueState { id name }
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
            health
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
            comments { nodes { id body createdAt url resolvedAt user { displayName } } }
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
            comments { nodes { id body createdAt url resolvedAt user { displayName } } }
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
            url
            resolvedAt
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
            url
            resolvedAt
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
                description
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
        description
        startsAt
        endsAt
        completedAt
        progress
        issues(first: 50) {
            nodes { id identifier title state { name } }
        }
        uncompletedIssuesUponClose(first: 50) {
            nodes { id identifier title state { name } }
        }
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
            color
            parent { id name }
            team { id key name }
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
            color
            parent { id name }
            team { id key name }
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
        health
        url
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
            health
            url
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
            health
            url
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

// ---- #21: Initiative queries ----

/// List initiatives.
pub const LIST_INITIATIVES: &str = r#"
query ListInitiatives($first: Int!) {
    initiatives(first: $first) {
        nodes {
            id
            name
            description
            status
            targetDate
            completedAt
            startedAt
            url
            slugId
            owner { displayName email }
            projects(first: 10) { nodes { id name } }
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
                addedLabels { name }
                removedLabels { name }
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

// ---- Phase 12: Remaining tools ----

/// Batch update multiple issues.
pub const BATCH_UPDATE_ISSUES: &str = r#"
mutation BatchUpdateIssues($ids: [UUID!]!, $input: IssueUpdateInput!) {
    issueBatchUpdate(ids: $ids, input: $input) {
        success
        issues {
            id
            identifier
            title
            state { name }
        }
    }
}
"#;

/// Search documents by term.
pub const SEARCH_DOCUMENTS: &str = r#"
query SearchDocuments($term: String!, $first: Int, $includeComments: Boolean) {
    searchDocuments(term: $term, first: $first, includeComments: $includeComments) {
        nodes {
            id
            title
            url
            slugId
            createdAt
            updatedAt
            project { name }
            creator { displayName }
        }
        totalCount
    }
}
"#;

/// Create an initiative.
pub const CREATE_INITIATIVE: &str = r#"
mutation CreateInitiative($input: InitiativeCreateInput!) {
    initiativeCreate(input: $input) {
        success
        initiative {
            id
            name
            description
            status
            targetDate
            completedAt
            startedAt
            url
            slugId
            owner { displayName email }
            projects(first: 10) { nodes { id name } }
        }
    }
}
"#;

/// Update an initiative.
pub const UPDATE_INITIATIVE: &str = r#"
mutation UpdateInitiative($id: String!, $input: InitiativeUpdateInput!) {
    initiativeUpdate(id: $id, input: $input) {
        success
        initiative {
            id
            name
            description
            status
            targetDate
            completedAt
            startedAt
            url
            slugId
            owner { displayName email }
            projects(first: 10) { nodes { id name } }
        }
    }
}
"#;

/// Delete an initiative.
pub const DELETE_INITIATIVE: &str = r#"
mutation DeleteInitiative($id: String!) {
    initiativeDelete(id: $id) {
        success
    }
}
"#;

/// Get issues from a custom view.
pub const GET_VIEW_ISSUES: &str = r#"
query GetViewIssues($id: String!, $first: Int) {
    customView(id: $id) {
        id
        name
        issues(first: $first) {
            nodes {
                id
                identifier
                title
                priority
                url
                state { id name type color }
                assignee { id displayName email }
                team { id key name }
                labels { nodes { id name } }
            }
            pageInfo { hasNextPage endCursor }
        }
    }
}
"#;

/// List triage issues (issues in triage state for a team).
pub const LIST_TRIAGE_ISSUES: &str = r#"
query ListTriageIssues($first: Int!, $filter: IssueFilter) {
    issues(first: $first, filter: $filter) {
        nodes {
            id
            identifier
            title
            priority
            createdAt
            url
            state { id name type color }
            assignee { id displayName email }
            team { id key name }
            labels { nodes { id name } }
        }
        pageInfo { hasNextPage endCursor }
    }
}
"#;

// ---- Phase 2: Delete/Archive mutations ----

/// Delete a document.
pub const DELETE_DOCUMENT: &str = r#"
mutation DeleteDocument($id: String!) {
    documentDelete(id: $id) { success }
}
"#;

/// Delete a project milestone.
pub const DELETE_PROJECT_MILESTONE: &str = r#"
mutation DeleteProjectMilestone($id: String!) {
    projectMilestoneDelete(id: $id) { success }
}
"#;

/// Delete a project update.
pub const DELETE_PROJECT_UPDATE: &str = r#"
mutation DeleteProjectUpdate($id: String!) {
    projectUpdateDelete(id: $id) { success }
}
"#;

/// Delete an attachment.
pub const DELETE_ATTACHMENT: &str = r#"
mutation DeleteAttachment($id: String!) {
    attachmentDelete(id: $id) { success }
}
"#;

/// Permanently delete an issue.
pub const DELETE_ISSUE: &str = r#"
mutation DeleteIssue($id: String!) {
    issueDelete(id: $id) { success }
}
"#;

/// Delete a custom view.
pub const DELETE_VIEW: &str = r#"
mutation DeleteView($id: String!) {
    customViewDelete(id: $id) { success }
}
"#;

/// Archive a cycle.
pub const ARCHIVE_CYCLE: &str = r#"
mutation ArchiveCycle($id: String!) {
    cycleArchive(id: $id) { success }
}
"#;

// ---- Phase 3: Update mutations ----

/// Update a cycle.
pub const UPDATE_CYCLE: &str = r#"
mutation UpdateCycle($id: String!, $input: CycleUpdateInput!) {
    cycleUpdate(id: $id, input: $input) {
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

/// Update a project milestone.
pub const UPDATE_PROJECT_MILESTONE: &str = r#"
mutation UpdateProjectMilestone($id: String!, $input: ProjectMilestoneUpdateInput!) {
    projectMilestoneUpdate(id: $id, input: $input) {
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

/// Update a project update.
pub const UPDATE_PROJECT_UPDATE: &str = r#"
mutation UpdateProjectUpdate($id: String!, $input: ProjectUpdateUpdateInput!) {
    projectUpdateUpdate(id: $id, input: $input) {
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

/// Update a webhook.
pub const UPDATE_WEBHOOK: &str = r#"
mutation UpdateWebhook($id: String!, $input: WebhookUpdateInput!) {
    webhookUpdate(id: $id, input: $input) {
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

/// Update an attachment.
pub const UPDATE_ATTACHMENT: &str = r#"
mutation UpdateAttachment($id: String!, $input: AttachmentUpdateInput!) {
    attachmentUpdate(id: $id, input: $input) {
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

/// Update a custom view.
pub const UPDATE_VIEW: &str = r#"
mutation UpdateView($id: String!, $input: CustomViewUpdateInput!) {
    customViewUpdate(id: $id, input: $input) {
        success
        customView {
            id
            name
            description
            filterData
        }
    }
}
"#;

// ---- Phase 4: Comment tools ----

/// List comments for an issue.
pub const LIST_COMMENTS: &str = r#"
query ListComments($id: String!, $first: Int!) {
    issue(id: $id) {
        comments(first: $first) {
            nodes {
                id
                body
                createdAt
                url
                resolvedAt
                user { displayName }
                parent { id }
            }
        }
    }
}
"#;

/// Resolve a comment thread.
pub const RESOLVE_COMMENT: &str = r#"
mutation ResolveComment($id: String!) {
    commentResolve(id: $id) {
        success
        comment {
            id
            body
            createdAt
            url
            resolvedAt
            user { displayName }
        }
    }
}
"#;

/// Unresolve a comment thread.
pub const UNRESOLVE_COMMENT: &str = r#"
mutation UnresolveComment($id: String!) {
    commentUnresolve(id: $id) {
        success
        comment {
            id
            body
            createdAt
            url
            resolvedAt
            user { displayName }
        }
    }
}
"#;

// ---- Phase 5: Issue subscribe/unsubscribe ----

/// Subscribe to an issue.
pub const SUBSCRIBE_TO_ISSUE: &str = r#"
mutation SubscribeToIssue($id: String!, $userId: String) {
    issueSubscribe(id: $id, userId: $userId) { success }
}
"#;

/// Unsubscribe from an issue.
pub const UNSUBSCRIBE_FROM_ISSUE: &str = r#"
mutation UnsubscribeFromIssue($id: String!, $userId: String) {
    issueUnsubscribe(id: $id, userId: $userId) { success }
}
"#;

// ---- Phase 6: View creates ----

/// Create a custom view.
pub const CREATE_VIEW: &str = r#"
mutation CreateView($input: CustomViewCreateInput!) {
    customViewCreate(input: $input) {
        success
        customView {
            id
            name
            description
            filterData
        }
    }
}
"#;

// ---- Phase 7: Search & Query tools ----

/// Full-text search projects.
pub const SEARCH_PROJECTS: &str = r#"
query SearchProjects($term: String!, $first: Int) {
    searchProjects(term: $term, first: $first) {
        nodes {
            id
            name
            description
            state
            progress
            url
            startDate
            targetDate
            lead { displayName email }
            teams { nodes { id key name } }
        }
        totalCount
    }
}
"#;

/// Find issue by VCS branch name.
pub const ISSUE_VCS_BRANCH_SEARCH: &str = r#"
query IssueVcsBranchSearch($branchName: String!) {
    issueVcsBranchSearch(branchName: $branchName) {
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
    }
}
"#;

// ---- Phase 8: Agent Sessions & Activities ----

/// Create an agent session on an issue.
pub const AGENT_SESSION_CREATE_ON_ISSUE: &str = r#"
mutation AgentSessionCreateOnIssue($input: AgentSessionCreateOnIssue!) {
    agentSessionCreateOnIssue(input: $input) {
        success
        agentSession {
            id
            status
            createdAt
            url
            issue { identifier title }
        }
    }
}
"#;

/// Create an agent session on a comment.
pub const AGENT_SESSION_CREATE_ON_COMMENT: &str = r#"
mutation AgentSessionCreateOnComment($input: AgentSessionCreateOnComment!) {
    agentSessionCreateOnComment(input: $input) {
        success
        agentSession {
            id
            status
            createdAt
            url
            comment { id body }
        }
    }
}
"#;

/// Update an agent session.
pub const UPDATE_AGENT_SESSION: &str = r#"
mutation UpdateAgentSession($id: String!, $input: AgentSessionUpdateInput!) {
    agentSessionUpdate(id: $id, input: $input) {
        success
        agentSession {
            id
            status
            url
            plan
        }
    }
}
"#;

/// Create an agent activity.
pub const CREATE_AGENT_ACTIVITY: &str = r#"
mutation CreateAgentActivity($input: AgentActivityCreateInput!) {
    agentActivityCreate(input: $input) {
        success
        agentActivity {
            id
            createdAt
        }
    }
}
"#;

/// List agent sessions.
pub const LIST_AGENT_SESSIONS: &str = r#"
query ListAgentSessions($first: Int!) {
    agentSessions(first: $first) {
        nodes {
            id
            status
            createdAt
            url
            issue { identifier title }
        }
        pageInfo { hasNextPage endCursor }
    }
}
"#;

/// Get a single agent session.
pub const GET_AGENT_SESSION: &str = r#"
query GetAgentSession($id: String!) {
    agentSession(id: $id) {
        id
        status
        createdAt
        startedAt
        endedAt
        url
        plan
        summary
        issue { identifier title }
        comment { id body }
        activities(first: 50) {
            nodes { id createdAt ephemeral }
        }
    }
}
"#;

// ---- Phase 9: Customer Management ----

/// List customers.
pub const LIST_CUSTOMERS: &str = r#"
query ListCustomers($first: Int!) {
    customers(first: $first) {
        nodes {
            id
            name
            domains
            revenue
            size
            slugId
            status { displayName color }
            tier { name }
            owner { displayName email }
        }
        pageInfo { hasNextPage endCursor }
    }
}
"#;

/// Get a single customer.
pub const GET_CUSTOMER: &str = r#"
query GetCustomer($id: String!) {
    customer(id: $id) {
        id
        name
        domains
        externalIds
        revenue
        size
        slugId
        logoUrl
        status { displayName color }
        tier { name }
        owner { displayName email }
        needs { id body priority createdAt issue { identifier title } customer { id name } }
    }
}
"#;

/// Create a customer.
pub const CREATE_CUSTOMER: &str = r#"
mutation CreateCustomer($input: CustomerCreateInput!) {
    customerCreate(input: $input) {
        success
        customer {
            id
            name
            domains
            revenue
            size
            status { displayName }
            owner { displayName email }
        }
    }
}
"#;

/// Update a customer.
pub const UPDATE_CUSTOMER: &str = r#"
mutation UpdateCustomer($id: String!, $input: CustomerUpdateInput!) {
    customerUpdate(id: $id, input: $input) {
        success
        customer {
            id
            name
            domains
            revenue
            size
            status { displayName }
            owner { displayName email }
        }
    }
}
"#;

/// Delete a customer.
pub const DELETE_CUSTOMER: &str = r#"
mutation DeleteCustomer($id: String!) {
    customerDelete(id: $id) { success }
}
"#;

/// List customer needs.
pub const LIST_CUSTOMER_NEEDS: &str = r#"
query ListCustomerNeeds($first: Int!) {
    customerNeeds(first: $first) {
        nodes {
            id
            body
            priority
            createdAt
            customer { id name }
            issue { identifier title }
        }
        pageInfo { hasNextPage endCursor }
    }
}
"#;

/// Create a customer need.
pub const CREATE_CUSTOMER_NEED: &str = r#"
mutation CreateCustomerNeed($input: CustomerNeedCreateInput!) {
    customerNeedCreate(input: $input) {
        success
        need {
            id
            body
            priority
            createdAt
            customer { id name }
            issue { identifier title }
        }
    }
}
"#;

/// Update a customer need.
pub const UPDATE_CUSTOMER_NEED: &str = r#"
mutation UpdateCustomerNeed($id: String!, $input: CustomerNeedUpdateInput!) {
    customerNeedUpdate(id: $id, input: $input) {
        success
        need {
            id
            body
            priority
            createdAt
            customer { id name }
            issue { identifier title }
        }
    }
}
"#;

// ---- Phase 10: Initiative Updates + Initiative-to-Project Links ----

/// List initiative updates.
pub const LIST_INITIATIVE_UPDATES: &str = r#"
query ListInitiativeUpdates($id: String!, $first: Int!) {
    initiative(id: $id) {
        initiativeUpdates(first: $first) {
            nodes {
                id
                body
                health
                createdAt
                url
                user { displayName }
            }
        }
    }
}
"#;

/// Create an initiative update.
pub const CREATE_INITIATIVE_UPDATE: &str = r#"
mutation CreateInitiativeUpdate($input: InitiativeUpdateCreateInput!) {
    initiativeUpdateCreate(input: $input) {
        success
        initiativeUpdate {
            id
            body
            health
            createdAt
            url
            user { displayName }
        }
    }
}
"#;

/// Add a project to an initiative.
pub const ADD_PROJECT_TO_INITIATIVE: &str = r#"
mutation AddProjectToInitiative($input: InitiativeToProjectCreateInput!) {
    initiativeToProjectCreate(input: $input) {
        success
        initiativeToProject {
            id
            initiative { name }
            project { name }
        }
    }
}
"#;

/// Remove a project from an initiative.
pub const REMOVE_PROJECT_FROM_INITIATIVE: &str = r#"
mutation RemoveProjectFromInitiative($id: String!) {
    initiativeToProjectDelete(id: $id) { success }
}
"#;

// ---- Phase 11: Project Relations ----

/// Create a project relation.
pub const CREATE_PROJECT_RELATION: &str = r#"
mutation CreateProjectRelation($input: ProjectRelationCreateInput!) {
    projectRelationCreate(input: $input) {
        success
        projectRelation {
            id
            type
            project { name }
            relatedProject { name }
        }
    }
}
"#;

/// Delete a project relation.
pub const DELETE_PROJECT_RELATION: &str = r#"
mutation DeleteProjectRelation($id: String!) {
    projectRelationDelete(id: $id) { success }
}
"#;

/// List project relations.
pub const LIST_PROJECT_RELATIONS: &str = r#"
query ListProjectRelations($id: String!) {
    project(id: $id) {
        relations(first: 50) {
            nodes {
                id
                type
                project { name }
                relatedProject { name }
            }
        }
    }
}
"#;

// ---- Phase 12: Releases ----

/// List releases.
pub const LIST_RELEASES: &str = r#"
query ListReleases($first: Int!) {
    releases(first: $first) {
        nodes {
            id
            name
            version
            startDate
            targetDate
            url
            stage { name color }
            pipeline { name }
        }
        pageInfo { hasNextPage endCursor }
    }
}
"#;

/// Create a release.
pub const CREATE_RELEASE: &str = r#"
mutation CreateRelease($input: ReleaseCreateInput!) {
    releaseCreate(input: $input) {
        success
        release {
            id
            name
            version
            url
            startDate
            targetDate
            stage { name }
            pipeline { name }
        }
    }
}
"#;

/// Update a release.
pub const UPDATE_RELEASE: &str = r#"
mutation UpdateRelease($id: String!, $input: ReleaseUpdateInput!) {
    releaseUpdate(id: $id, input: $input) {
        success
        release {
            id
            name
            version
            url
            startDate
            targetDate
            stage { name }
            pipeline { name }
        }
    }
}
"#;
