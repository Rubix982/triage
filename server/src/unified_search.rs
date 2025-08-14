use crate::content_storage::{SEARCH_CONTENT_FULL_TEXT, GET_RELATED_CONTENT};
use crate::db_utils::with_connection;
use crate::semantic_search::RelatedItem;
use crate::knowledge_engine::{KnowledgeConcept, TechnologyKnowledge};
use crate::user_notes::{UserNote, SavedView};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnifiedSearchRequest {
    pub query: String,
    pub content_types: Vec<UnifiedContentType>,
    pub platforms: Vec<String>, // "jira", "google", "slack", "confluence", "github"
    pub date_range: Option<DateRange>,
    pub authors: Vec<String>,
    pub projects: Vec<String>,
    pub max_results: Option<usize>,
    pub similarity_threshold: Option<f64>,
    pub include_relationships: bool,
    pub search_mode: SearchMode,
    pub user_id: Option<String>, // For permission filtering
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum UnifiedContentType {
    JiraIssue,
    GoogleDoc,
    GoogleSheet,
    GoogleSlide,
    SlackThread,
    SlackMessage,
    ConfluencePage,
    GitHubPR,
    GitHubIssue,
    UserNote,
    SavedView,
    KnowledgeConcept,
    TechnologyKnowledge,
    All,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SearchMode {
    Relevance,        // Sort by relevance score
    Recent,          // Sort by most recent
    Popular,         // Sort by engagement metrics
    Comprehensive,   // Include all related content
    Semantic,        // AI-powered semantic matching
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateRange {
    pub from: String, // ISO 8601 date
    pub to: String,   // ISO 8601 date
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnifiedSearchResult {
    pub query: String,
    pub total_results: usize,
    pub search_time_ms: u64,
    pub results: Vec<EnhancedSearchResult>,
    pub facets: SearchFacets,
    pub suggestions: Vec<String>,
    pub related_queries: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnhancedSearchResult {
    pub id: String,
    pub title: String,
    pub content: String,
    pub content_preview: String, // First 200 chars with highlights
    pub result_type: UnifiedContentType,
    pub platform: String,
    pub similarity_score: f64,
    pub relevance_score: f64,
    pub context: EnhancedSearchContext,
    pub related_items: Vec<RelatedItem>,
    pub tags: Vec<String>,
    pub concepts: Vec<String>,
    pub technologies: Vec<String>,
    pub created_date: String,
    pub last_updated: String,
    pub author: Option<String>,
    pub engagement_metrics: EngagementSummary,
    pub access_info: AccessInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnhancedSearchContext {
    pub project: String,
    pub platform: String,
    pub content_type: String,
    pub category: String,
    pub source_url: String,
    pub parent_ticket: Option<String>, // If linked to a Jira ticket
    pub knowledge_impact_score: f64,
    pub usage_frequency: i32,
    pub related_concepts: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EngagementSummary {
    pub view_count: u32,
    pub comment_count: u32,
    pub share_count: u32,
    pub reaction_count: u32,
    pub search_hits: u32,
    pub knowledge_score: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessInfo {
    pub is_accessible: bool,
    pub requires_auth: bool,
    pub platform_auth_required: Vec<String>, // ["google", "slack"]
    pub sharing_level: String, // "public", "team", "private"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchFacets {
    pub platforms: HashMap<String, usize>,
    pub content_types: HashMap<String, usize>,
    pub authors: HashMap<String, usize>,
    pub projects: HashMap<String, usize>,
    pub date_ranges: HashMap<String, usize>,
    pub concepts: HashMap<String, usize>,
    pub technologies: HashMap<String, usize>,
}

pub struct UnifiedSearchEngine {
    max_results: usize,
}

impl UnifiedSearchEngine {
    pub fn new() -> Self {
        Self {
            max_results: 100,
        }
    }

    pub async fn search(&self, request: UnifiedSearchRequest) -> Result<UnifiedSearchResult, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        let mut all_results = Vec::new();
        
        // Search across different content types
        if request.content_types.contains(&UnifiedContentType::All) || 
           request.content_types.iter().any(|t| matches!(t, UnifiedContentType::GoogleDoc | UnifiedContentType::GoogleSheet | UnifiedContentType::GoogleSlide | UnifiedContentType::SlackThread | UnifiedContentType::SlackMessage)) {
            let content_results = self.search_extracted_content(&request).await?;
            all_results.extend(content_results);
        }

        if request.content_types.contains(&UnifiedContentType::All) || 
           request.content_types.contains(&UnifiedContentType::JiraIssue) {
            let jira_results = self.search_jira_issues(&request).await?;
            all_results.extend(jira_results);
        }

        if request.content_types.contains(&UnifiedContentType::All) || 
           request.content_types.contains(&UnifiedContentType::UserNote) {
            let note_results = self.search_user_notes(&request).await?;
            all_results.extend(note_results);
        }

        if request.content_types.contains(&UnifiedContentType::All) || 
           request.content_types.contains(&UnifiedContentType::KnowledgeConcept) {
            let concept_results = self.search_knowledge_concepts(&request).await?;
            all_results.extend(concept_results);
        }

        // Sort and limit results
        all_results.sort_by(|a, b| {
            match request.search_mode {
                SearchMode::Relevance => b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal),
                SearchMode::Recent => b.last_updated.cmp(&a.last_updated),
                SearchMode::Popular => b.engagement_metrics.knowledge_score.partial_cmp(&a.engagement_metrics.knowledge_score).unwrap_or(std::cmp::Ordering::Equal),
                SearchMode::Comprehensive => b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal),
                SearchMode::Semantic => b.similarity_score.partial_cmp(&a.similarity_score).unwrap_or(std::cmp::Ordering::Equal),
            }
        });

        let max_results = request.max_results.unwrap_or(self.max_results);
        all_results.truncate(max_results);

        // Add related content if requested
        if request.include_relationships {
            for result in &mut all_results {
                result.related_items = self.find_related_content(&result.id).await?;
            }
        }

        // Generate facets
        let facets = self.generate_facets(&all_results);
        
        // Generate search suggestions
        let suggestions = self.generate_suggestions(&request.query).await?;
        let related_queries = self.generate_related_queries(&request.query).await?;

        let search_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(UnifiedSearchResult {
            query: request.query,
            total_results: all_results.len(),
            search_time_ms,
            results: all_results,
            facets,
            suggestions,
            related_queries,
        })
    }

    async fn search_extracted_content(&self, request: &UnifiedSearchRequest) -> Result<Vec<EnhancedSearchResult>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        with_connection("search_extracted_content", |conn| {
            let search_pattern = format!("%{}%", request.query);
            let limit = request.max_results.unwrap_or(50);
            
            let mut stmt = conn.prepare(SEARCH_CONTENT_FULL_TEXT).expect("Failed to prepare search statement");
            let rows = stmt.query_map([&search_pattern, &limit.to_string()], |row| {
                Ok((
                    row.get::<_, String>(0)?, // id
                    row.get::<_, String>(1)?, // content_type
                    row.get::<_, String>(2)?, // source_url
                    row.get::<_, String>(3)?, // source_platform
                    row.get::<_, String>(4)?, // title
                    row.get::<_, String>(5)?, // body_text
                    row.get::<_, Option<String>>(6)?, // author
                    row.get::<_, Option<String>>(7)?, // created_at
                    row.get::<_, String>(8)?, // last_updated_at
                    row.get::<_, String>(9)?, // metadata (JSON)
                ))
            }).expect("Failed to execute search query");

            for row_result in rows {
                if let Ok((id, content_type_str, source_url, platform, title, body_text, author, created_at, last_updated_at, metadata)) = row_result {
                    let content_type = self.parse_content_type(&content_type_str);
                    let metadata_json: serde_json::Value = serde_json::from_str(&metadata).unwrap_or_default();
                    
                    let result = EnhancedSearchResult {
                        id: id.clone(),
                        title: title.clone(),
                        content: body_text.clone(),
                        content_preview: self.create_preview(&body_text, &request.query),
                        result_type: content_type.clone(),
                        platform: platform.clone(),
                        similarity_score: self.calculate_similarity(&title, &body_text, &request.query),
                        relevance_score: self.calculate_relevance(&title, &body_text, &request.query, &metadata_json),
                        context: EnhancedSearchContext {
                            project: self.extract_project_from_metadata(&metadata_json),
                            platform: source_url.split('/').nth(2).unwrap_or("unknown").to_string(),
                            content_type: content_type.to_string(),
                            category: self.extract_category_from_metadata(&metadata_json),
                            source_url: source_url.clone(),
                            parent_ticket: self.find_parent_ticket(&id).unwrap_or_default(),
                            knowledge_impact_score: metadata_json["quality_score"].as_f64().unwrap_or(5.0),
                            usage_frequency: metadata_json["engagement_metrics"]["view_count"].as_i64().unwrap_or(0) as i32,
                            related_concepts: self.extract_concepts_from_metadata(&metadata_json),
                        },
                        related_items: Vec::new(), // Filled later if requested
                        tags: self.extract_tags_from_metadata(&metadata_json),
                        concepts: self.extract_concepts_from_metadata(&metadata_json),
                        technologies: self.extract_technologies_from_metadata(&metadata_json),
                        created_date: created_at.unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
                        last_updated: last_updated_at,
                        author,
                        engagement_metrics: self.extract_engagement_metrics(&metadata_json),
                        access_info: AccessInfo {
                            is_accessible: true, // TODO: Check user permissions
                            requires_auth: !platform.contains("jira"),
                            platform_auth_required: if platform.contains("google") { vec!["google".to_string()] } else if platform.contains("slack") { vec!["slack".to_string()] } else { vec![] },
                            sharing_level: "team".to_string(), // TODO: Extract from permissions
                        },
                    };
                    
                    results.push(result);
                }
            }
            
        });

        Ok(results)
    }

    async fn search_jira_issues(&self, request: &UnifiedSearchRequest) -> Result<Vec<EnhancedSearchResult>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        with_connection("search_jira_issues", |conn| {
            let search_pattern = format!("%{}%", request.query);
            
            let query = r#"
                SELECT id, key, summary, status, issue_type, priority, assignee, reporter, 
                       created, updated, project_name, project_key, description, extracted_links
                FROM issues 
                WHERE (summary LIKE ? OR description LIKE ?)
                ORDER BY updated DESC
                LIMIT ?
            "#;
            
            let mut stmt = conn.prepare(query).expect("Failed to prepare search query");
            let limit = request.max_results.unwrap_or(25);
            let rows = stmt.query_map([&search_pattern, &search_pattern, &limit.to_string()], |row| {
                Ok((
                    row.get::<_, String>(0)?, // id
                    row.get::<_, String>(1)?, // key
                    row.get::<_, Option<String>>(2)?, // summary
                    row.get::<_, String>(3)?, // status
                    row.get::<_, Option<String>>(4)?, // issue_type
                    row.get::<_, Option<String>>(5)?, // priority
                    row.get::<_, Option<String>>(6)?, // assignee
                    row.get::<_, Option<String>>(7)?, // reporter
                    row.get::<_, Option<String>>(8)?, // created
                    row.get::<_, Option<String>>(9)?, // updated
                    row.get::<_, Option<String>>(10)?, // project_name
                    row.get::<_, Option<String>>(11)?, // project_key
                    row.get::<_, Option<String>>(12)?, // description
                    row.get::<_, Option<String>>(13)?, // extracted_links
                ))
            }).expect("Failed to execute search query");

            for row_result in rows {
                if let Ok((id, key, summary, status, issue_type_opt, priority, _assignee, reporter, created, updated, project_name, _project_key, description, _extracted_links)) = row_result {
                    let title = summary.unwrap_or_else(|| format!("{} - {}", key, status));
                    let content = description.unwrap_or_default();
                    
                    let result = EnhancedSearchResult {
                        id: id.clone(),
                        title: title.clone(),
                        content: content.clone(),
                        content_preview: self.create_preview(&content, &request.query),
                        result_type: UnifiedContentType::JiraIssue,
                        platform: "jira".to_string(),
                        similarity_score: self.calculate_similarity(&title, &content, &request.query),
                        relevance_score: self.calculate_relevance(&title, &content, &request.query, &serde_json::Value::Null),
                        context: EnhancedSearchContext {
                            project: project_name.unwrap_or_default(),
                            platform: "jira".to_string(),
                            content_type: issue_type_opt.clone().unwrap_or("Issue".to_string()),
                            category: priority.clone().unwrap_or("Medium".to_string()),
                            source_url: format!("https://jira.example.com/browse/{}", key), // TODO: Use actual domain
                            parent_ticket: None,
                            knowledge_impact_score: 5.0, // TODO: Calculate based on comments, links, etc.
                            usage_frequency: 1,
                            related_concepts: Vec::new(),
                        },
                        related_items: Vec::new(),
                        tags: vec![status.clone(), issue_type_opt.unwrap_or_default(), priority.unwrap_or_default()],
                        concepts: Vec::new(), // TODO: Extract from knowledge engine
                        technologies: Vec::new(), // TODO: Extract from knowledge engine
                        created_date: created.unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
                        last_updated: updated.unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
                        author: reporter,
                        engagement_metrics: EngagementSummary {
                            view_count: 1,
                            comment_count: 0, // TODO: Parse from comments
                            share_count: 0,
                            reaction_count: 0,
                            search_hits: 1,
                            knowledge_score: 5.0,
                        },
                        access_info: AccessInfo {
                            is_accessible: true,
                            requires_auth: false,
                            platform_auth_required: vec![],
                            sharing_level: "team".to_string(),
                        },
                    };
                    
                    results.push(result);
                }
            }
            
        });

        Ok(results)
    }

    async fn search_user_notes(&self, _request: &UnifiedSearchRequest) -> Result<Vec<EnhancedSearchResult>, Box<dyn std::error::Error>> {
        // TODO: Implement user notes search
        // This would search the user_notes table similar to above
        Ok(Vec::new())
    }

    async fn search_knowledge_concepts(&self, _request: &UnifiedSearchRequest) -> Result<Vec<EnhancedSearchResult>, Box<dyn std::error::Error>> {
        // TODO: Implement knowledge concepts search
        // This would search extracted concepts and technologies
        Ok(Vec::new())
    }

    async fn find_related_content(&self, content_id: &str) -> Result<Vec<RelatedItem>, Box<dyn std::error::Error>> {
        let mut related_items = Vec::new();
        
        with_connection("find_related_content", |conn| {
            let mut stmt = conn.prepare(GET_RELATED_CONTENT).expect("Failed to prepare related content query");
            let rows = stmt.query_map([content_id, "10"], |row| {
                Ok((
                    row.get::<_, String>(0)?, // id
                    row.get::<_, String>(1)?, // title  
                    row.get::<_, String>(2)?, // relationship_type
                    row.get::<_, f64>(3)?,    // strength
                ))
            }).expect("Failed to execute search query");

            for row_result in rows {
                if let Ok((id, title, relationship_type, strength)) = row_result {
                    related_items.push(RelatedItem {
                        id,
                        title,
                        relationship_type,
                        similarity_score: strength,
                    });
                }
            }
            
        });

        Ok(related_items)
    }

    fn generate_facets(&self, results: &[EnhancedSearchResult]) -> SearchFacets {
        let mut platforms = HashMap::new();
        let mut content_types = HashMap::new();
        let mut authors = HashMap::new();
        let mut projects = HashMap::new();
        let mut concepts = HashMap::new();
        let mut technologies = HashMap::new();

        for result in results {
            *platforms.entry(result.platform.clone()).or_insert(0) += 1;
            *content_types.entry(result.result_type.to_string()).or_insert(0) += 1;
            if let Some(author) = &result.author {
                *authors.entry(author.clone()).or_insert(0) += 1;
            }
            *projects.entry(result.context.project.clone()).or_insert(0) += 1;
            
            for concept in &result.concepts {
                *concepts.entry(concept.clone()).or_insert(0) += 1;
            }
            for tech in &result.technologies {
                *technologies.entry(tech.clone()).or_insert(0) += 1;
            }
        }

        SearchFacets {
            platforms,
            content_types,
            authors,
            projects,
            date_ranges: HashMap::new(), // TODO: Implement date range facets
            concepts,
            technologies,
        }
    }

    async fn generate_suggestions(&self, _query: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // TODO: Implement query suggestions based on popular searches, concepts, etc.
        Ok(vec![])
    }

    async fn generate_related_queries(&self, _query: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // TODO: Implement related query suggestions
        Ok(vec![])
    }

    // Helper methods
    fn parse_content_type(&self, content_type: &str) -> UnifiedContentType {
        match content_type {
            "google_doc" => UnifiedContentType::GoogleDoc,
            "google_sheet" => UnifiedContentType::GoogleSheet,
            "google_slide" => UnifiedContentType::GoogleSlide,
            "slack_thread" => UnifiedContentType::SlackThread,
            "slack_message" => UnifiedContentType::SlackMessage,
            "confluence_page" => UnifiedContentType::ConfluencePage,
            "github_pr" => UnifiedContentType::GitHubPR,
            "github_issue" => UnifiedContentType::GitHubIssue,
            _ => UnifiedContentType::JiraIssue,
        }
    }

    fn create_preview(&self, content: &str, query: &str) -> String {
        let query_lower = query.to_lowercase();
        if let Some(pos) = content.to_lowercase().find(&query_lower) {
            let start = pos.saturating_sub(50);
            let end = std::cmp::min(content.len(), pos + query.len() + 150);
            let preview = &content[start..end];
            format!("...{preview}...")
        } else {
            content.chars().take(200).collect::<String>() + if content.len() > 200 { "..." } else { "" }
        }
    }

    fn calculate_similarity(&self, title: &str, content: &str, query: &str) -> f64 {
        let query_lower = query.to_lowercase();
        let title_lower = title.to_lowercase();
        let content_lower = content.to_lowercase();
        
        let mut score: f64 = 0.0;
        
        // Title exact match
        if title_lower.contains(&query_lower) {
            score += 0.8;
        }
        
        // Content match
        if content_lower.contains(&query_lower) {
            score += 0.4;
        }
        
        // Word-level matching
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();
        let title_words: Vec<&str> = title_lower.split_whitespace().collect();
        let content_words: Vec<&str> = content_lower.split_whitespace().collect();
        
        for query_word in &query_words {
            if title_words.contains(query_word) {
                score += 0.3;
            }
            if content_words.contains(query_word) {
                score += 0.1;
            }
        }
        
        // Normalize to 0-1 range
        score.min(1.0)
    }

    fn calculate_relevance(&self, title: &str, content: &str, query: &str, metadata: &serde_json::Value) -> f64 {
        let similarity = self.calculate_similarity(title, content, query);
        let quality_score = metadata["quality_score"].as_f64().unwrap_or(5.0) / 10.0;
        let engagement_score = metadata["engagement_metrics"]["view_count"].as_f64().unwrap_or(1.0).ln() / 10.0;
        
        similarity * 0.6 + quality_score * 0.3 + engagement_score * 0.1
    }

    // Metadata extraction helpers
    fn extract_project_from_metadata(&self, metadata: &serde_json::Value) -> String {
        metadata["project"].as_str().unwrap_or("Unknown").to_string()
    }

    fn extract_category_from_metadata(&self, metadata: &serde_json::Value) -> String {
        metadata["category"].as_str().unwrap_or("General").to_string()
    }

    fn extract_concepts_from_metadata(&self, metadata: &serde_json::Value) -> Vec<String> {
        metadata["concepts"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(String::from).collect())
            .unwrap_or_default()
    }

    fn extract_technologies_from_metadata(&self, metadata: &serde_json::Value) -> Vec<String> {
        metadata["technologies"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(String::from).collect())
            .unwrap_or_default()
    }

    fn extract_tags_from_metadata(&self, metadata: &serde_json::Value) -> Vec<String> {
        metadata["tags"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(String::from).collect())
            .unwrap_or_default()
    }

    fn extract_engagement_metrics(&self, metadata: &serde_json::Value) -> EngagementSummary {
        let metrics = &metadata["engagement_metrics"];
        EngagementSummary {
            view_count: metrics["view_count"].as_u64().unwrap_or(0) as u32,
            comment_count: metrics["comment_count"].as_u64().unwrap_or(0) as u32,
            share_count: metrics["share_count"].as_u64().unwrap_or(0) as u32,
            reaction_count: metrics["reaction_count"].as_u64().unwrap_or(0) as u32,
            search_hits: metrics["search_hits"].as_u64().unwrap_or(0) as u32,
            knowledge_score: metadata["quality_score"].as_f64().unwrap_or(5.0),
        }
    }

    fn find_parent_ticket(&self, _content_id: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        // TODO: Query content_extraction_jobs to find source ticket
        Ok(None)
    }
}

impl UnifiedContentType {
    pub fn to_string(&self) -> String {
        match self {
            UnifiedContentType::JiraIssue => "jira_issue".to_string(),
            UnifiedContentType::GoogleDoc => "google_doc".to_string(),
            UnifiedContentType::GoogleSheet => "google_sheet".to_string(),
            UnifiedContentType::GoogleSlide => "google_slide".to_string(),
            UnifiedContentType::SlackThread => "slack_thread".to_string(),
            UnifiedContentType::SlackMessage => "slack_message".to_string(),
            UnifiedContentType::ConfluencePage => "confluence_page".to_string(),
            UnifiedContentType::GitHubPR => "github_pr".to_string(),
            UnifiedContentType::GitHubIssue => "github_issue".to_string(),
            UnifiedContentType::UserNote => "user_note".to_string(),
            UnifiedContentType::SavedView => "saved_view".to_string(),
            UnifiedContentType::KnowledgeConcept => "knowledge_concept".to_string(),
            UnifiedContentType::TechnologyKnowledge => "technology_knowledge".to_string(),
            UnifiedContentType::All => "all".to_string(),
        }
    }
}

// Main search function to be called from the API
pub async fn unified_search(request: UnifiedSearchRequest) -> Result<UnifiedSearchResult, Box<dyn std::error::Error>> {
    let search_engine = UnifiedSearchEngine::new();
    search_engine.search(request).await
}