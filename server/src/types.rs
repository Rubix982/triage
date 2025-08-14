use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
}

// Graph data structures for knowledge visualization
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub node_type: NodeType,
    pub size: f32,
    pub color: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub edge_type: EdgeType,
    pub weight: f32,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum NodeType {
    Issue,
    Person,
    Project,
    Component,
    Technology,
    Concept,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum EdgeType {
    References,
    AssignedTo,
    PartOf,
    DependsOn,
    Similar,
    Collaborates,
    Mentions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KnowledgeGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub metadata: GraphMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphMetadata {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub node_types: HashMap<NodeType, usize>,
    pub edge_types: HashMap<EdgeType, usize>,
    pub generated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub domain: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Issue {
    pub id: String,
    pub key: String,

    #[serde(rename = "fields")]
    pub fields: IssueFields,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IssueFields {
    pub summary: Option<String>,
    pub description: Option<serde_json::Value>,
    pub status: Option<IssueStatus>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub issuetype: Option<IssueType>,
    pub priority: Option<IssuePriority>,
    pub assignee: Option<serde_json::Value>,
    pub reporter: Option<serde_json::Value>,
    pub labels: Option<Vec<String>>,
    pub components: Option<Vec<serde_json::Value>>,
    pub parent: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IssueStatus {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IssueType {
    pub name: String,
    pub id: String,
    pub description: Option<String>,
    pub subtask: Option<bool>,
    pub hierarchy_level: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IssuePriority {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueFieldMetadata {
    pub id: String,
    pub key: String,
    pub self_link: String,
    pub summary: Option<String>,
    pub status: String,
    pub issue_type: Option<String>,
    pub issue_type_id: Option<String>,
    pub is_subtask: Option<bool>,
    pub hierarchy_level: Option<i32>,
    pub priority: Option<String>,
    pub priority_id: Option<String>,
    pub assignee: Option<String>,
    pub reporter: Option<String>,
    pub labels: Option<Vec<String>>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub project_name: Option<String>,
    pub project_key: Option<String>,
    pub extracted_links: Option<Vec<ExtractedLink>>,
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

// New types for link extraction
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtractedLink {
    pub url: String,
    pub platform_type: PlatformType,
    pub link_context: String, // Where the link was found (description, comment, etc.)
    pub extraction_metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum PlatformType {
    GoogleDocs { document_id: String },
    GoogleSheets { spreadsheet_id: String },
    GoogleSlides { presentation_id: String },
    SlackThread { workspace: String, channel: String, thread_ts: String },
    SlackMessage { workspace: String, channel: String, message_ts: String },
    ConfluencePage { space: String, page_id: String },
    GitHubPR { owner: String, repo: String, pr_number: u64 },
    GitHubIssue { owner: String, repo: String, issue_number: u64 },
    GitHubCommit { owner: String, repo: String, commit_hash: String },
    Unknown { domain: String },
}
