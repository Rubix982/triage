use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub domain: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub key: String,

    #[serde(rename = "fields")]
    pub fields: IssueFields,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueFields {
    pub summary: Option<String>,
    pub description: Option<serde_json::Value>,
    pub status: Option<IssueStatus>,
    pub created: Option<String>,
    pub updated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueStatus {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueFieldMetadata {
    pub id: String,
    pub key: String,
    pub self_link: String,
    pub summary: Option<String>,
    pub status: String,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub rendered_fields: Option<String>,
    pub names: Option<String>,
    pub schema: Option<String>,
    pub transitions: Option<String>,
    pub edit_meta: Option<String>,
    pub changelog: Option<String>,
    pub versioned_representations: Option<String>,
    pub watcher: Option<serde_json::Value>,
    pub attachment: Option<serde_json::Value>,
    pub sub_tasks: Option<serde_json::Value>,
    pub description: Option<serde_json::Value>,
    pub project: Option<serde_json::Value>,
    pub comment: Option<serde_json::Value>,
    pub issue_links: Option<serde_json::Value>,
    pub work_log: Option<serde_json::Value>,
    pub time_tracking: Option<serde_json::Value>,
}
