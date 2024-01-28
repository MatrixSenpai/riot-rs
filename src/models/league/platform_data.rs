use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformDataDto {
    pub id: String,
    pub name: String,
    pub locales: Vec<String>,
    pub maintenances: Vec<StatusDto>,
    pub incidents: Vec<StatusDto>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusDto {
    pub id: i32,
    pub maintenance_status: MaintenanceStatus,
    pub incident_severity: IncidentSeverity,
    pub titles: Vec<ContentDto>,
    pub updates: Vec<UpdateDto>,
    pub created_at: String,
    pub archive_at: Option<String>,
    pub updated_at: Option<String>,
    pub platforms: Vec<Platform>,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceStatus {
    Scheduled,
    InProgress,
    Complete,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IncidentSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentDto {
    pub locale: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDto {
    pub id: i32,
    pub author: String,
    pub publish: bool,
    pub publish_location: Vec<PublishLocation>,
    pub translations: Vec<ContentDto>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PublishLocation {
    RiotClient,
    RiotStatus,
    Game,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Windows,
    Macos,
    Android,
    Ios,
    Ps4,
    Xbone,
    Switch,
}