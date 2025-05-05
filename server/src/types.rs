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
