use crate::db_utils::with_connection;
use crate::knowledge_engine::{KnowledgeConcept, TechnologyKnowledge};
use crate::types::Issue;
use crate::utils::{log_step, log_success};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub content: String,
    pub result_type: SearchResultType,
    pub similarity_score: f64,
    pub context: SearchContext,
    pub related_items: Vec<RelatedItem>,
    pub tags: Vec<String>,
    pub created_date: String,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SearchResultType {
    Issue,
    Concept,
    Technology,
    Pattern,
    LearningMaterial,
    UserNote,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchContext {
    pub project: String,
    pub category: String,
    pub difficulty: String,
    pub expertise_level: f64,
    pub usage_frequency: i32,
    pub related_concepts: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelatedItem {
    pub id: String,
    pub title: String,
    pub relationship_type: String,
    pub similarity_score: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SemanticSearchQuery {
    pub query: String,
    pub search_types: Vec<SearchResultType>,
    pub similarity_threshold: f64,
    pub max_results: usize,
    pub context_filters: Vec<String>,
    pub include_related: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConceptCluster {
    pub id: String,
    pub name: String,
    pub center_concept: String,
    pub related_concepts: Vec<String>,
    pub cohesion_score: f64,
    pub cluster_size: usize,
    pub dominant_category: String,
    pub average_difficulty: String,
    pub key_technologies: Vec<String>,
    pub representative_issues: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartRecommendation {
    pub id: String,
    pub recommendation_type: RecommendationType,
    pub title: String,
    pub description: String,
    pub target_items: Vec<String>,
    pub confidence_score: f64,
    pub reasoning: Vec<String>,
    pub action_items: Vec<String>,
    pub estimated_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RecommendationType {
    SimilarIssue,
    RelatedConcept,
    LearningPath,
    ExpertConnection,
    KnowledgeGap,
    BestPractice,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SemanticSearchResponse {
    pub results: Vec<SearchResult>,
    pub clusters: Vec<ConceptCluster>,
    pub recommendations: Vec<SmartRecommendation>,
    pub query_analysis: QueryAnalysis,
    pub total_results: usize,
    pub search_time_ms: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryAnalysis {
    pub detected_concepts: Vec<String>,
    pub detected_technologies: Vec<String>,
    pub query_intent: QueryIntent,
    pub difficulty_level: String,
    pub suggested_refinements: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QueryIntent {
    FindSimilar,
    LearnAbout,
    Troubleshoot,
    GetBestPractices,
    ExploreRelated,
    FindExperts,
}

pub async fn semantic_search(query: SemanticSearchQuery) -> SemanticSearchResponse {
    let start_time = std::time::Instant::now();
    log_step("🔍", &format!("Performing semantic search for: '{}'", query.query));

    // Analyze the query to understand intent and extract concepts
    let query_analysis = analyze_query(&query.query).await;
    
    // Get base search results
    let mut results = perform_base_search(&query, &query_analysis).await;
    
    // Calculate semantic similarities
    enhance_with_similarity_scores(&mut results, &query.query).await;
    
    // Filter and sort results
    results.retain(|r| r.similarity_score >= query.similarity_threshold);
    results.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
    results.truncate(query.max_results);
    
    // Add related items if requested
    if query.include_related {
        add_related_items(&mut results).await;
    }
    
    // Generate concept clusters
    let clusters = generate_concept_clusters(&results).await;
    
    // Generate smart recommendations
    let recommendations = generate_smart_recommendations(&query, &results, &query_analysis).await;
    
    let search_time_ms = start_time.elapsed().as_millis() as u64;
    
    log_success(&format!("Found {} results in {}ms", results.len(), search_time_ms));
    
    SemanticSearchResponse {
        results,
        clusters,
        recommendations,
        query_analysis,
        total_results: results.len(),
        search_time_ms,
    }
}

async fn analyze_query(query: &str) -> QueryAnalysis {
    let mut detected_concepts = Vec::new();
    let mut detected_technologies = Vec::new();
    let mut suggested_refinements = Vec::new();
    
    // Extract concepts from query using same patterns as knowledge engine
    let concepts = extract_concepts_from_query(query);
    detected_concepts.extend(concepts);
    
    // Extract technologies
    let technologies = extract_technologies_from_query(query);
    detected_technologies.extend(technologies);
    
    // Determine query intent
    let query_intent = determine_query_intent(query);
    
    // Assess difficulty level
    let difficulty_level = assess_query_difficulty(query, &detected_technologies);
    
    // Generate suggestions for refinement
    if detected_concepts.is_empty() && detected_technologies.is_empty() {
        suggested_refinements.push("Try including specific technologies or concepts".to_string());
    }
    
    if query.len() < 5 {
        suggested_refinements.push("Try a more descriptive query".to_string());
    }
    
    QueryAnalysis {
        detected_concepts,
        detected_technologies,
        query_intent,
        difficulty_level,
        suggested_refinements,
    }
}

fn extract_concepts_from_query(query: &str) -> Vec<String> {
    let mut concepts = Vec::new();
    
    // Technical concepts
    let concept_patterns = vec![
        r"(?i)\b(api|rest|graphql|microservices|authentication|authorization|caching|testing)\b",
        r"(?i)\b(performance|optimization|security|deployment|monitoring|logging)\b",
        r"(?i)\b(database|migration|query|orm|nosql|sql)\b",
        r"(?i)\b(frontend|backend|fullstack|ui|ux|responsive)\b",
    ];
    
    for pattern in concept_patterns {
        let re = Regex::new(pattern).unwrap();
        for mat in re.find_iter(query) {
            let concept = mat.as_str().to_lowercase();
            if !concepts.contains(&concept) {
                concepts.push(concept);
            }
        }
    }
    
    concepts
}

fn extract_technologies_from_query(query: &str) -> Vec<String> {
    let mut technologies = Vec::new();
    
    let tech_patterns = vec![
        r"(?i)\b(react|vue|angular|javascript|typescript|python|rust|java|go)\b",
        r"(?i)\b(docker|kubernetes|jenkins|github|aws|azure|gcp)\b",
        r"(?i)\b(mysql|postgresql|mongodb|redis|elasticsearch)\b",
        r"(?i)\b(express|fastify|spring|django|flask|rails)\b",
    ];
    
    for pattern in tech_patterns {
        let re = Regex::new(pattern).unwrap();
        for mat in re.find_iter(query) {
            let tech = mat.as_str().to_lowercase();
            if !technologies.contains(&tech) {
                technologies.push(tech);
            }
        }
    }
    
    technologies
}

fn determine_query_intent(query: &str) -> QueryIntent {
    let query_lower = query.to_lowercase();
    
    if query_lower.contains("similar") || query_lower.contains("like") {
        QueryIntent::FindSimilar
    } else if query_lower.contains("how to") || query_lower.contains("learn") {
        QueryIntent::LearnAbout
    } else if query_lower.contains("error") || query_lower.contains("problem") || query_lower.contains("issue") {
        QueryIntent::Troubleshoot
    } else if query_lower.contains("best") || query_lower.contains("practice") || query_lower.contains("recommend") {
        QueryIntent::GetBestPractices
    } else if query_lower.contains("related") || query_lower.contains("connect") {
        QueryIntent::ExploreRelated
    } else if query_lower.contains("expert") || query_lower.contains("who") {
        QueryIntent::FindExperts
    } else {
        QueryIntent::FindSimilar
    }
}

fn assess_query_difficulty(query: &str, technologies: &[String]) -> String {
    let advanced_terms = ["kubernetes", "microservices", "distributed", "architecture", "security"];
    let has_advanced_terms = advanced_terms.iter().any(|term| query.to_lowercase().contains(term));
    
    if has_advanced_terms || technologies.len() > 3 {
        "advanced".to_string()
    } else if technologies.len() > 1 || query.len() > 50 {
        "intermediate".to_string()
    } else {
        "beginner".to_string()
    }
}

async fn perform_base_search(query: &SemanticSearchQuery, analysis: &QueryAnalysis) -> Vec<SearchResult> {
    let mut results = Vec::new();
    
    with_connection("semantic_search", |conn| {
        // Search issues
        let issue_query = r#"
        SELECT id, key, summary, description, comment, status, created,
               JSON_EXTRACT(project, '$.name') as project_name,
               labels
        FROM issues 
        WHERE (summary LIKE ?1 OR description LIKE ?1 OR comment LIKE ?1)
           OR (? IN (summary, description, comment))
        ORDER BY created DESC
        LIMIT 50
        "#;
        
        let search_term = format!("%{}%", query.query);
        let mut stmt = conn.prepare(issue_query).expect("Failed to prepare search query");
        let rows = stmt.query_map([&search_term, &query.query], |row| {
            Ok(SearchResult {
                id: row.get::<_, String>(0)?,
                title: format!("{}: {}", row.get::<_, String>(1)?, row.get::<_, String>(2)?),
                content: format!("{} {}", row.get::<_, String>(3).unwrap_or_default(), row.get::<_, String>(4).unwrap_or_default()),
                result_type: SearchResultType::Issue,
                similarity_score: 0.5, // Will be calculated later
                context: SearchContext {
                    project: row.get::<_, String>(7).unwrap_or_default(),
                    category: "Issue".to_string(),
                    difficulty: "intermediate".to_string(),
                    expertise_level: 5.0,
                    usage_frequency: 1,
                    related_concepts: Vec::new(),
                },
                related_items: Vec::new(),
                tags: parse_labels(&row.get::<_, String>(8).unwrap_or_default()),
                created_date: row.get::<_, String>(6)?,
                last_updated: row.get::<_, String>(6)?,
            })
        }).expect("Failed to execute search query");

        for row in rows {
            if let Ok(result) = row {
                results.push(result);
            }
        }
    });
    
    results
}

fn parse_labels(labels_json: &str) -> Vec<String> {
    // Simple JSON array parsing for labels
    if labels_json.is_empty() || labels_json == "{}" {
        return Vec::new();
    }
    
    // Extract labels from JSON string (simplified approach)
    labels_json
        .trim_matches(|c| c == '[' || c == ']' || c == '{' || c == '}')
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

async fn enhance_with_similarity_scores(results: &mut [SearchResult], query: &str) {
    for result in results.iter_mut() {
        result.similarity_score = calculate_text_similarity(&result.content, query);
        
        // Boost score for title matches
        if result.title.to_lowercase().contains(&query.to_lowercase()) {
            result.similarity_score += 0.2;
        }
        
        // Boost score for exact concept matches
        let query_concepts = extract_concepts_from_query(query);
        for concept in &query_concepts {
            if result.content.to_lowercase().contains(concept) {
                result.similarity_score += 0.1;
            }
        }
    }
}

fn calculate_text_similarity(text: &str, query: &str) -> f64 {
    // Simple similarity calculation using word overlap
    let text_words: HashSet<String> = text
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    let query_words: HashSet<String> = query
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    if query_words.is_empty() {
        return 0.0;
    }
    
    let intersection_size = text_words.intersection(&query_words).count() as f64;
    let union_size = text_words.union(&query_words).count() as f64;
    
    if union_size == 0.0 {
        0.0
    } else {
        intersection_size / union_size
    }
}

async fn add_related_items(results: &mut [SearchResult]) {
    for result in results.iter_mut() {
        // Find related items based on shared concepts and technologies
        let related = find_related_items(&result.id, &result.content).await;
        result.related_items = related;
    }
}

async fn find_related_items(item_id: &str, content: &str) -> Vec<RelatedItem> {
    let mut related = Vec::new();
    
    // Extract concepts from current content
    let concepts = extract_concepts_from_query(content);
    
    if concepts.is_empty() {
        return related;
    }
    
    with_connection("find_related", |conn| {
        let query = r#"
        SELECT id, key, summary, description
        FROM issues 
        WHERE id != ?1 
          AND (description LIKE ?2 OR summary LIKE ?2)
        LIMIT 5
        "#;
        
        let search_pattern = format!("%{}%", concepts.join("%"));
        let mut stmt = conn.prepare(query).expect("Failed to prepare related query");
        let rows = stmt.query_map([item_id, &search_pattern], |row| {
            Ok(RelatedItem {
                id: row.get::<_, String>(0)?,
                title: format!("{}: {}", row.get::<_, String>(1)?, row.get::<_, String>(2)?),
                relationship_type: "Similar Concept".to_string(),
                similarity_score: 0.7, // Simplified
            })
        }).expect("Failed to execute related query");

        for row in rows {
            if let Ok(item) = row {
                related.push(item);
            }
        }
    });
    
    related
}

async fn generate_concept_clusters(results: &[SearchResult]) -> Vec<ConceptCluster> {
    let mut clusters = Vec::new();
    let mut concept_groups: HashMap<String, Vec<&SearchResult>> = HashMap::new();
    
    // Group results by detected concepts
    for result in results {
        let concepts = extract_concepts_from_query(&result.content);
        for concept in concepts {
            concept_groups.entry(concept).or_default().push(result);
        }
    }
    
    // Create clusters from groups with sufficient size
    for (concept, group_results) in concept_groups {
        if group_results.len() >= 2 {
            let cluster = ConceptCluster {
                id: format!("cluster_{}", concept),
                name: format!("Cluster: {}", concept.to_uppercase()),
                center_concept: concept.clone(),
                related_concepts: extract_related_concepts(&group_results),
                cohesion_score: calculate_cluster_cohesion(&group_results),
                cluster_size: group_results.len(),
                dominant_category: find_dominant_category(&group_results),
                average_difficulty: "intermediate".to_string(),
                key_technologies: extract_cluster_technologies(&group_results),
                representative_issues: group_results.iter().take(3).map(|r| r.id.clone()).collect(),
            };
            clusters.push(cluster);
        }
    }
    
    // Sort clusters by size and cohesion
    clusters.sort_by(|a, b| {
        let score_a = a.cluster_size as f64 * a.cohesion_score;
        let score_b = b.cluster_size as f64 * b.cohesion_score;
        score_b.partial_cmp(&score_a).unwrap()
    });
    
    clusters.truncate(10); // Keep top 10 clusters
    clusters
}

fn extract_related_concepts(results: &[&SearchResult]) -> Vec<String> {
    let mut all_concepts = Vec::new();
    for result in results {
        let concepts = extract_concepts_from_query(&result.content);
        all_concepts.extend(concepts);
    }
    
    // Remove duplicates and return most frequent
    let mut concept_counts: HashMap<String, usize> = HashMap::new();
    for concept in all_concepts {
        *concept_counts.entry(concept).or_insert(0) += 1;
    }
    
    let mut sorted_concepts: Vec<(String, usize)> = concept_counts.into_iter().collect();
    sorted_concepts.sort_by(|a, b| b.1.cmp(&a.1));
    
    sorted_concepts.into_iter().take(5).map(|(concept, _)| concept).collect()
}

fn calculate_cluster_cohesion(results: &[&SearchResult]) -> f64 {
    if results.len() < 2 {
        return 1.0;
    }
    
    let mut total_similarity = 0.0;
    let mut comparisons = 0;
    
    for i in 0..results.len() {
        for j in (i + 1)..results.len() {
            total_similarity += calculate_text_similarity(&results[i].content, &results[j].content);
            comparisons += 1;
        }
    }
    
    if comparisons == 0 {
        1.0
    } else {
        total_similarity / comparisons as f64
    }
}

fn find_dominant_category(results: &[&SearchResult]) -> String {
    let mut category_counts: HashMap<String, usize> = HashMap::new();
    for result in results {
        *category_counts.entry(result.context.category.clone()).or_insert(0) += 1;
    }
    
    category_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(category, _)| category)
        .unwrap_or_else(|| "Mixed".to_string())
}

fn extract_cluster_technologies(results: &[&SearchResult]) -> Vec<String> {
    let mut all_technologies = Vec::new();
    for result in results {
        let technologies = extract_technologies_from_query(&result.content);
        all_technologies.extend(technologies);
    }
    
    // Remove duplicates and return most frequent
    let mut tech_counts: HashMap<String, usize> = HashMap::new();
    for tech in all_technologies {
        *tech_counts.entry(tech).or_insert(0) += 1;
    }
    
    let mut sorted_techs: Vec<(String, usize)> = tech_counts.into_iter().collect();
    sorted_techs.sort_by(|a, b| b.1.cmp(&a.1));
    
    sorted_techs.into_iter().take(3).map(|(tech, _)| tech).collect()
}

async fn generate_smart_recommendations(
    query: &SemanticSearchQuery,
    results: &[SearchResult],
    analysis: &QueryAnalysis,
) -> Vec<SmartRecommendation> {
    let mut recommendations = Vec::new();
    
    // Recommendation 1: Similar Issues
    if !results.is_empty() && matches!(analysis.query_intent, QueryIntent::FindSimilar | QueryIntent::Troubleshoot) {
        recommendations.push(SmartRecommendation {
            id: "rec_similar".to_string(),
            recommendation_type: RecommendationType::SimilarIssue,
            title: "Similar Issues You Might Find Helpful".to_string(),
            description: format!("Found {} issues with similar patterns to your search", results.len()),
            target_items: results.iter().take(3).map(|r| r.id.clone()).collect(),
            confidence_score: 0.8,
            reasoning: vec![
                "Based on content similarity analysis".to_string(),
                "Matching concepts and technologies".to_string(),
            ],
            action_items: vec![
                "Review solutions from similar issues".to_string(),
                "Check for common patterns and approaches".to_string(),
            ],
            estimated_value: "High - can accelerate problem solving".to_string(),
        });
    }
    
    // Recommendation 2: Learning Path
    if !analysis.detected_technologies.is_empty() {
        recommendations.push(SmartRecommendation {
            id: "rec_learning".to_string(),
            recommendation_type: RecommendationType::LearningPath,
            title: "Suggested Learning Path".to_string(),
            description: format!("Based on your interest in: {}", analysis.detected_technologies.join(", ")),
            target_items: analysis.detected_technologies.clone(),
            confidence_score: 0.7,
            reasoning: vec![
                "Detected relevant technologies in your search".to_string(),
                "Common learning progression identified".to_string(),
            ],
            action_items: vec![
                "Start with foundational concepts".to_string(),
                "Practice with real examples from issues".to_string(),
                "Connect with team members who have experience".to_string(),
            ],
            estimated_value: "Medium - structured learning approach".to_string(),
        });
    }
    
    // Recommendation 3: Knowledge Gaps
    if results.len() < 5 {
        recommendations.push(SmartRecommendation {
            id: "rec_gap".to_string(),
            recommendation_type: RecommendationType::KnowledgeGap,
            title: "Potential Knowledge Gap Identified".to_string(),
            description: "Limited information found - this might be an area for knowledge development".to_string(),
            target_items: vec![query.query.clone()],
            confidence_score: 0.6,
            reasoning: vec![
                "Few relevant results found".to_string(),
                "May indicate underexplored area".to_string(),
            ],
            action_items: vec![
                "Consider creating documentation for this topic".to_string(),
                "Share knowledge if you solve this problem".to_string(),
                "Look for external resources and best practices".to_string(),
            ],
            estimated_value: "High - opportunity to build team knowledge".to_string(),
        });
    }
    
    recommendations
}