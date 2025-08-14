use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::db_utils::with_connection;
use crate::utils::{log_step, log_success};

// ================================
// UNIFIED CONTENT STORAGE SCHEMA
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredContent {
    pub id: Uuid,
    pub content_type: ContentType,
    pub source_url: String,
    pub source_platform: String,
    pub title: String,
    pub body_text: String,
    pub raw_content: serde_json::Value, // Platform-specific full data
    pub content_hash: String, // For deduplication
    pub author: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub extracted_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub content_status: ContentStatus,
    pub access_permissions: serde_json::Value,
    pub metadata: ContentMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ContentType {
    GoogleDoc,
    GoogleSheet, 
    GoogleSlide,
    SlackThread,
    SlackMessage,
    ConfluencePage,
    GitHubPR,
    GitHubIssue,
    JiraTicket, // For consistency
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ContentStatus {
    Active,
    Outdated,
    Deleted,
    AccessDenied,
    ExtractionFailed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentMetadata {
    pub word_count: u32,
    pub char_count: u32,
    pub participant_count: u32,
    pub comment_count: u32,
    pub attachment_count: u32,
    pub language: Option<String>,
    pub tags: Vec<String>,
    pub concepts: Vec<String>, // Extracted concepts
    pub technologies: Vec<String>, // Identified technologies
    pub sentiment_score: Option<f32>, // -1.0 to 1.0
    pub quality_score: Option<f32>, // 0.0 to 10.0
    pub engagement_metrics: EngagementMetrics,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngagementMetrics {
    pub view_count: u32,
    pub edit_count: u32,
    pub comment_count: u32,
    pub share_count: u32,
    pub reaction_count: u32,
    pub mention_count: u32,
}

// ================================
// CONTENT RELATIONSHIPS
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentRelationship {
    pub id: Uuid,
    pub source_content_id: Uuid,
    pub target_content_id: Uuid,
    pub relationship_type: RelationshipType,
    pub strength: f32, // 0.0 to 1.0
    pub context: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RelationshipType {
    LinkedFrom,        // Content A links to Content B
    References,        // Content A references Content B
    DiscussionOf,      // Slack thread discusses Google Doc
    ImplementationOf,  // Code implements design
    FollowUpTo,        // Content B follows up on Content A
    DuplicateOf,       // Similar/duplicate content
    PartOf,           // Content is part of larger content
    Mentions,         // Content mentions other content
}

// ================================
// SEARCH INDEX STRUCTURES
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchIndex {
    pub id: Uuid,
    pub content_id: Uuid,
    pub content_type: ContentType,
    pub title_tokens: Vec<String>,
    pub body_tokens: Vec<String>,
    pub concept_tokens: Vec<String>,
    pub author_tokens: Vec<String>,
    pub full_text_search: String, // Concatenated searchable text
    pub embedding_vector: Option<Vec<f32>>, // For semantic search
    pub indexed_at: DateTime<Utc>,
}

// ================================
// CONTENT EXTRACTION JOBS STORAGE
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentExtractionJobRecord {
    pub id: Uuid,
    pub source_ticket_id: String,
    pub source_url: String,
    pub platform_type: ContentType,
    pub user_id: String,
    pub team_id: Option<String>, // For Slack
    pub priority: String,
    pub status: String,
    pub retry_count: u32,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub extracted_content_id: Option<Uuid>,
}

// ================================
// USER AUTHENTICATION STORAGE
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAuthToken {
    pub id: Uuid,
    pub user_id: String,
    pub platform: String, // "google", "slack", "github", etc.
    pub team_id: Option<String>, // For team-based platforms like Slack
    pub access_token_encrypted: String,
    pub refresh_token_encrypted: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub scopes: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
    pub is_active: bool,
}

// ================================
// CONTENT ANALYTICS
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentAnalytics {
    pub id: Uuid,
    pub content_id: Uuid,
    pub date: chrono::NaiveDate,
    pub view_count: u32,
    pub search_hits: u32,
    pub link_clicks: u32,
    pub note_creations: u32, // How many notes reference this content
    pub knowledge_impact_score: f32,
}

// ================================
// CONTENT VERSIONS (FOR CHANGE TRACKING)
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentVersion {
    pub id: Uuid,
    pub content_id: Uuid,
    pub version_number: u32,
    pub title: String,
    pub body_text: String,
    pub content_hash: String,
    pub author: Option<String>,
    pub modified_at: DateTime<Utc>,
    pub change_summary: Option<String>,
    pub diff_from_previous: Option<String>, // JSON diff
}

// ================================
// SQL TABLE DEFINITIONS
// ================================

pub const CREATE_CONTENT_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS extracted_content (
    id TEXT PRIMARY KEY,
    content_type TEXT NOT NULL,
    source_url TEXT NOT NULL UNIQUE,
    source_platform TEXT NOT NULL,
    title TEXT NOT NULL,
    body_text TEXT NOT NULL,
    raw_content TEXT, -- JSON
    content_hash TEXT NOT NULL,
    author TEXT,
    created_at TEXT,
    modified_at TEXT,
    extracted_at TEXT NOT NULL,
    last_updated_at TEXT NOT NULL,
    content_status TEXT NOT NULL,
    access_permissions TEXT, -- JSON
    metadata TEXT, -- JSON ContentMetadata
    
    -- Indexes for search
    INDEX idx_content_type (content_type),
    INDEX idx_source_platform (source_platform),
    INDEX idx_content_hash (content_hash),
    INDEX idx_author (author),
    INDEX idx_created_at (created_at)
);
"#;

pub const CREATE_CONTENT_RELATIONSHIPS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS content_relationships (
    id TEXT PRIMARY KEY,
    source_content_id TEXT NOT NULL,
    target_content_id TEXT NOT NULL,
    relationship_type TEXT NOT NULL,
    strength REAL NOT NULL,
    context TEXT,
    created_at TEXT NOT NULL,
    
    FOREIGN KEY (source_content_id) REFERENCES extracted_content(id),
    FOREIGN KEY (target_content_id) REFERENCES extracted_content(id),
    
    INDEX idx_source_content (source_content_id),
    INDEX idx_target_content (target_content_id),
    INDEX idx_relationship_type (relationship_type)
);
"#;

pub const CREATE_SEARCH_INDEX_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS content_search_index (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL,
    content_type TEXT NOT NULL,
    title_tokens TEXT, -- JSON array
    body_tokens TEXT, -- JSON array  
    concept_tokens TEXT, -- JSON array
    author_tokens TEXT, -- JSON array
    full_text_search TEXT NOT NULL,
    embedding_vector TEXT, -- JSON array of floats
    indexed_at TEXT NOT NULL,
    
    FOREIGN KEY (content_id) REFERENCES extracted_content(id),
    
    -- Full-text search index
    INDEX idx_full_text (full_text_search),
    INDEX idx_content_type_search (content_type),
    INDEX idx_indexed_at (indexed_at)
);
"#;

pub const CREATE_EXTRACTION_JOBS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS content_extraction_jobs (
    id TEXT PRIMARY KEY,
    source_ticket_id TEXT NOT NULL,
    source_url TEXT NOT NULL,
    platform_type TEXT NOT NULL,
    user_id TEXT NOT NULL,
    team_id TEXT,
    priority TEXT NOT NULL,
    status TEXT NOT NULL,
    retry_count INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    started_at TEXT,
    completed_at TEXT,
    error_message TEXT,
    extracted_content_id TEXT,
    
    FOREIGN KEY (extracted_content_id) REFERENCES extracted_content(id),
    
    INDEX idx_status (status),
    INDEX idx_user_id (user_id),
    INDEX idx_source_ticket (source_ticket_id),
    INDEX idx_created_at (created_at)
);
"#;

pub const CREATE_USER_AUTH_TOKENS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS user_auth_tokens (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    platform TEXT NOT NULL,
    team_id TEXT,
    access_token_encrypted TEXT NOT NULL,
    refresh_token_encrypted TEXT,
    token_expires_at TEXT,
    scopes TEXT, -- JSON array
    created_at TEXT NOT NULL,
    last_used_at TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    
    INDEX idx_user_platform (user_id, platform),
    INDEX idx_team (team_id),
    INDEX idx_active (is_active)
);
"#;

pub const CREATE_CONTENT_ANALYTICS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS content_analytics (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL,
    date DATE NOT NULL,
    view_count INTEGER DEFAULT 0,
    search_hits INTEGER DEFAULT 0,
    link_clicks INTEGER DEFAULT 0,
    note_creations INTEGER DEFAULT 0,
    knowledge_impact_score REAL DEFAULT 0.0,
    
    FOREIGN KEY (content_id) REFERENCES extracted_content(id),
    
    UNIQUE(content_id, date),
    INDEX idx_date (date),
    INDEX idx_impact_score (knowledge_impact_score)
);
"#;

pub const CREATE_CONTENT_VERSIONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS content_versions (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL,
    version_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    body_text TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    author TEXT,
    modified_at TEXT NOT NULL,
    change_summary TEXT,
    diff_from_previous TEXT, -- JSON
    
    FOREIGN KEY (content_id) REFERENCES extracted_content(id),
    
    UNIQUE(content_id, version_number),
    INDEX idx_content_version (content_id, version_number),
    INDEX idx_modified_at (modified_at)
);
"#;

// ================================
// INSERTION QUERIES
// ================================

pub const INSERT_CONTENT: &str = r#"
INSERT OR REPLACE INTO extracted_content 
(id, content_type, source_url, source_platform, title, body_text, raw_content, 
 content_hash, author, created_at, modified_at, extracted_at, last_updated_at, 
 content_status, access_permissions, metadata)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#;

pub const INSERT_CONTENT_RELATIONSHIP: &str = r#"
INSERT OR REPLACE INTO content_relationships
(id, source_content_id, target_content_id, relationship_type, strength, context, created_at)
VALUES (?, ?, ?, ?, ?, ?, ?)
"#;

pub const INSERT_SEARCH_INDEX: &str = r#"
INSERT OR REPLACE INTO content_search_index
(id, content_id, content_type, title_tokens, body_tokens, concept_tokens, 
 author_tokens, full_text_search, embedding_vector, indexed_at)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#;

pub const INSERT_EXTRACTION_JOB: &str = r#"
INSERT OR REPLACE INTO content_extraction_jobs
(id, source_ticket_id, source_url, platform_type, user_id, team_id, 
 priority, status, retry_count, created_at, started_at, completed_at, 
 error_message, extracted_content_id)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#;

pub const INSERT_USER_AUTH_TOKEN: &str = r#"
INSERT OR REPLACE INTO user_auth_tokens
(id, user_id, platform, team_id, access_token_encrypted, refresh_token_encrypted,
 token_expires_at, scopes, created_at, last_used_at, is_active)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#;

// ================================
// SEARCH QUERIES
// ================================

pub const SEARCH_CONTENT_FULL_TEXT: &str = r#"
SELECT c.*, si.full_text_search 
FROM extracted_content c
JOIN content_search_index si ON c.id = si.content_id
WHERE si.full_text_search LIKE ? 
  AND c.content_status = 'Active'
ORDER BY c.last_updated_at DESC
LIMIT ?
"#;

pub const SEARCH_CONTENT_BY_TYPE_AND_AUTHOR: &str = r#"
SELECT * FROM extracted_content 
WHERE content_type = ? 
  AND author LIKE ?
  AND content_status = 'Active'
ORDER BY last_updated_at DESC
LIMIT ?
"#;

pub const GET_RELATED_CONTENT: &str = r#"
SELECT c.*, cr.relationship_type, cr.strength
FROM extracted_content c
JOIN content_relationships cr ON c.id = cr.target_content_id
WHERE cr.source_content_id = ?
ORDER BY cr.strength DESC
LIMIT ?
"#;

pub const GET_CONTENT_BY_TICKET: &str = r#"
SELECT c.* FROM extracted_content c
JOIN content_extraction_jobs cej ON c.id = cej.extracted_content_id
WHERE cej.source_ticket_id = ?
  AND c.content_status = 'Active'
ORDER BY c.extracted_at DESC
"#;

// ================================
// UTILITY FUNCTIONS
// ================================

impl ContentType {
    pub fn from_platform_type(platform_type: &crate::types::PlatformType) -> Self {
        match platform_type {
            crate::types::PlatformType::GoogleDocs { .. } => ContentType::GoogleDoc,
            crate::types::PlatformType::GoogleSheets { .. } => ContentType::GoogleSheet,
            crate::types::PlatformType::GoogleSlides { .. } => ContentType::GoogleSlide,
            crate::types::PlatformType::SlackThread { .. } => ContentType::SlackThread,
            crate::types::PlatformType::SlackMessage { .. } => ContentType::SlackMessage,
            crate::types::PlatformType::ConfluencePage { .. } => ContentType::ConfluencePage,
            crate::types::PlatformType::GitHubPR { .. } => ContentType::GitHubPR,
            crate::types::PlatformType::GitHubIssue { .. } => ContentType::GitHubIssue,
            crate::types::PlatformType::GitHubCommit { .. } => ContentType::GitHubIssue, // Treat as GitHub content
            crate::types::PlatformType::Unknown { .. } => ContentType::JiraTicket, // fallback
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            ContentType::GoogleDoc => "google_doc".to_string(),
            ContentType::GoogleSheet => "google_sheet".to_string(),
            ContentType::GoogleSlide => "google_slide".to_string(),
            ContentType::SlackThread => "slack_thread".to_string(),
            ContentType::SlackMessage => "slack_message".to_string(),
            ContentType::ConfluencePage => "confluence_page".to_string(),
            ContentType::GitHubPR => "github_pr".to_string(),
            ContentType::GitHubIssue => "github_issue".to_string(),
            ContentType::JiraTicket => "jira_ticket".to_string(),
        }
    }
}

impl Default for EngagementMetrics {
    fn default() -> Self {
        Self {
            view_count: 0,
            edit_count: 0,
            comment_count: 0,
            share_count: 0,
            reaction_count: 0,
            mention_count: 0,
        }
    }
}

impl Default for ContentMetadata {
    fn default() -> Self {
        Self {
            word_count: 0,
            char_count: 0,
            participant_count: 0,
            comment_count: 0,
            attachment_count: 0,
            language: None,
            tags: Vec::new(),
            concepts: Vec::new(),
            technologies: Vec::new(),
            sentiment_score: None,
            quality_score: None,
            engagement_metrics: EngagementMetrics::default(),
        }
    }
}

// Table creation function
pub async fn create_content_storage_tables() {
    log_step("🗄️", "Initializing content storage tables...");
    
    with_connection("create_content_storage", |conn| {
        // Create all content storage tables
        let tables = vec![
            ("extracted_content", CREATE_CONTENT_TABLE),
            ("content_relationships", CREATE_CONTENT_RELATIONSHIPS_TABLE),
            ("content_search_index", CREATE_SEARCH_INDEX_TABLE),
            ("content_extraction_jobs", CREATE_EXTRACTION_JOBS_TABLE),
            ("user_auth_tokens", CREATE_USER_AUTH_TOKENS_TABLE),
            ("content_analytics", CREATE_CONTENT_ANALYTICS_TABLE),
            ("content_versions", CREATE_CONTENT_VERSIONS_TABLE),
        ];
        
        for (table_name, create_sql) in tables {
            match conn.execute_batch(create_sql) {
                Ok(_) => {
                    println!("✅ Created table: {}", table_name);
                },
                Err(e) => {
                    panic!("❌ Failed to create table {}: {}", table_name, e);
                }
            }
        }
        
        log_success("All content storage tables ready.");
    });
}