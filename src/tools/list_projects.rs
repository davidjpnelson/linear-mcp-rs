use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListProjectsParams {
    /// Filter by project status
    pub status: Option<ProjectStatus>,
    /// Filter by team key (e.g. 'ENG'). Shows projects associated with this team.
    pub team: Option<String>,
    /// Filter by lead email or display name
    pub lead: Option<String>,
    /// Max results (default 50)
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Planned,
    Started,
    Paused,
    Completed,
    Canceled,
}

impl ProjectStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectStatus::Planned => "planned",
            ProjectStatus::Started => "started",
            ProjectStatus::Paused => "paused",
            ProjectStatus::Completed => "completed",
            ProjectStatus::Canceled => "canceled",
        }
    }
}
