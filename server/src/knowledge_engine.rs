use crate::db_utils::with_connection;
use crate::utils::{log_step, log_success};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeBase {
    pub concepts: Vec<KnowledgeConcept>,
    pub technologies: Vec<TechnologyKnowledge>,
    pub patterns: Vec<KnowledgePattern>,
    pub learning_materials: Vec<LearningMaterial>,
    pub knowledge_gaps: Vec<KnowledgeGap>,
    pub expertise_map: HashMap<String, Vec<ExpertiseArea>>,
    pub knowledge_graph: KnowledgeSemanticGraph,
    pub insights: Vec<KnowledgeInsight>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeConcept {
    pub id: String,
    pub name: String,
    pub category: ConceptCategory,
    pub description: String,
    pub confidence_score: f64,
    pub frequency: i32,
    pub related_issues: Vec<String>,
    pub context_examples: Vec<String>,
    pub learning_difficulty: String, // "beginner", "intermediate", "advanced"
    pub prerequisites: Vec<String>,
    pub related_concepts: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TechnologyKnowledge {
    pub id: String,
    pub name: String,
    pub category: TechCategory,
    pub version_info: Option<String>,
    pub usage_patterns: Vec<UsagePattern>,
    pub common_issues: Vec<CommonIssue>,
    pub best_practices: Vec<BestPractice>,
    pub learning_resources: Vec<String>,
    pub skill_level_required: String,
    pub team_expertise_level: f64,
    pub adoption_trend: String,
    pub related_technologies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgePattern {
    pub id: String,
    pub pattern_type: PatternType,
    pub name: String,
    pub description: String,
    pub examples: Vec<PatternExample>,
    pub effectiveness_score: f64,
    pub usage_frequency: i32,
    pub success_rate: f64,
    pub anti_patterns: Vec<String>,
    pub when_to_use: Vec<String>,
    pub when_not_to_use: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LearningMaterial {
    pub id: String,
    pub title: String,
    pub content_type: ContentType,
    pub source_issues: Vec<String>,
    pub extracted_content: String,
    pub key_learnings: Vec<String>,
    pub difficulty_level: String,
    pub estimated_reading_time: i32, // minutes
    pub prerequisites: Vec<String>,
    pub related_materials: Vec<String>,
    pub quality_score: f64,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeGap {
    pub id: String,
    pub gap_type: GapType,
    pub title: String,
    pub description: String,
    pub severity: f64,
    pub affected_areas: Vec<String>,
    pub potential_impact: String,
    pub suggested_actions: Vec<String>,
    pub learning_priority: i32,
    pub estimated_effort: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpertiseArea {
    pub area: String,
    pub level: f64, // 0-10 scale
    pub confidence: f64,
    pub evidence_count: i32,
    pub recent_activity: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeSemanticGraph {
    pub concept_relationships: Vec<ConceptRelationship>,
    pub learning_paths: Vec<LearningPath>,
    pub knowledge_clusters: Vec<KnowledgeCluster>,
    pub expertise_networks: Vec<ExpertiseNetwork>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeInsight {
    pub id: String,
    pub insight_type: InsightType,
    pub title: String,
    pub description: String,
    pub confidence: f64,
    pub impact: String,
    pub action_items: Vec<String>,
    pub supporting_evidence: Vec<String>,
    pub generated_at: String,
}

// Enums
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConceptCategory {
    Technical,
    Business,
    Process,
    Architecture,
    Security,
    Performance,
    Testing,
    DevOps,
    UserExperience,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TechCategory {
    Language,
    Framework,
    Library,
    Tool,
    Platform,
    Database,
    Infrastructure,
    Protocol,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PatternType {
    Design,
    Architecture,
    Process,
    Problem,
    Solution,
    AntiPattern,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContentType {
    Tutorial,
    BestPractice,
    Troubleshooting,
    Architecture,
    ProcessGuide,
    LessonsLearned,
    FAQ,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GapType {
    Knowledge,
    Skills,
    Documentation,
    Process,
    Tools,
    Training,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InsightType {
    LearningOpportunity,
    KnowledgeRisk,
    SkillGap,
    BestPractice,
    ProcessImprovement,
}

// Supporting structures
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsagePattern {
    pub context: String,
    pub frequency: i32,
    pub success_rate: f64,
    pub common_mistakes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonIssue {
    pub issue: String,
    pub solutions: Vec<String>,
    pub prevention: Vec<String>,
    pub frequency: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BestPractice {
    pub practice: String,
    pub rationale: String,
    pub examples: Vec<String>,
    pub benefits: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatternExample {
    pub title: String,
    pub description: String,
    pub code_example: Option<String>,
    pub outcome: String,
    pub lessons_learned: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConceptRelationship {
    pub from_concept: String,
    pub to_concept: String,
    pub relationship_type: String,
    pub strength: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LearningPath {
    pub id: String,
    pub name: String,
    pub concepts: Vec<String>,
    pub estimated_time: String,
    pub difficulty_curve: Vec<f64>,
    pub prerequisites: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeCluster {
    pub id: String,
    pub name: String,
    pub concepts: Vec<String>,
    pub cohesion_score: f64,
    pub cluster_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpertiseNetwork {
    pub domain: String,
    pub experts: Vec<String>,
    pub knowledge_flow: Vec<KnowledgeFlow>,
    pub learning_opportunities: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeFlow {
    pub from_expert: String,
    pub to_learner: String,
    pub knowledge_area: String,
    pub flow_strength: f64,
}

pub async fn build_knowledge_base() -> KnowledgeBase {
    log_step("🧠", "Building intelligent knowledge base from issue data...");

    let concepts = extract_knowledge_concepts().await;
    let technologies = identify_technologies().await;
    let patterns = discover_patterns().await;
    let learning_materials = generate_learning_materials().await;
    let knowledge_gaps = detect_knowledge_gaps(&concepts, &technologies).await;
    let expertise_map = analyze_team_expertise(&concepts, &technologies).await;
    let knowledge_graph = build_semantic_graph(&concepts, &patterns).await;
    let insights = generate_knowledge_insights(&concepts, &knowledge_gaps, &expertise_map).await;

    log_success("Knowledge base built with AI-powered analysis");

    KnowledgeBase {
        concepts,
        technologies,
        patterns,
        learning_materials,
        knowledge_gaps,
        expertise_map,
        knowledge_graph,
        insights,
    }
}

async fn extract_knowledge_concepts() -> Vec<KnowledgeConcept> {
    log_step("🔍", "Extracting knowledge concepts from issue content...");
    
    let mut concepts = Vec::new();
    
    with_connection("extract_concepts", |conn| {
        let query = r#"
        SELECT 
            id, key, summary, description, comment, 
            JSON_EXTRACT(project, '$.name') as project_name,
            status
        FROM issues 
        WHERE (description IS NOT NULL AND description != '{}' AND description != '') 
           OR (comment IS NOT NULL AND comment != '{}' AND comment != '')
        ORDER BY created DESC
        LIMIT 200
        "#;

        let mut stmt = conn.prepare(query).expect("Failed to prepare concepts query");
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2).unwrap_or_default(),
                row.get::<_, String>(3).unwrap_or_default(),
                row.get::<_, String>(4).unwrap_or_default(),
                row.get::<_, String>(5).unwrap_or_default(),
                row.get::<_, String>(6).unwrap_or_default(),
            ))
        }).expect("Failed to execute concepts query");

        let mut concept_frequency: HashMap<String, i32> = HashMap::new();
        let mut concept_contexts: HashMap<String, Vec<String>> = HashMap::new();
        let mut concept_issues: HashMap<String, HashSet<String>> = HashMap::new();

        for row in rows {
            if let Ok((id, key, summary, description, comment, project, _status)) = row {
                let full_text = format!("{} {} {} {}", summary, description, comment, project);
                let extracted_concepts = extract_concepts_from_text(&full_text);
                
                for concept in extracted_concepts {
                    *concept_frequency.entry(concept.clone()).or_insert(0) += 1;
                    concept_contexts.entry(concept.clone()).or_default()
                        .push(format!("{}: {}", key, summary.chars().take(100).collect::<String>()));
                    concept_issues.entry(concept.clone()).or_default().insert(id.clone());
                }
            }
        }

        // Convert to structured concepts
        for (concept_name, frequency) in concept_frequency {
            if frequency >= 2 { // Only include concepts mentioned multiple times
                let category = categorize_concept(&concept_name);
                let confidence_score = calculate_concept_confidence(frequency, &concept_name);
                
                concepts.push(KnowledgeConcept {
                    id: format!("concept_{}", concepts.len()),
                    name: concept_name.clone(),
                    category,
                    description: generate_concept_description(&concept_name, &concept_contexts.get(&concept_name).unwrap_or(&Vec::new())),
                    confidence_score,
                    frequency,
                    related_issues: concept_issues.get(&concept_name).unwrap_or(&HashSet::new()).iter().cloned().collect(),
                    context_examples: concept_contexts.get(&concept_name).unwrap_or(&Vec::new()).clone(),
                    learning_difficulty: assess_learning_difficulty(&concept_name),
                    prerequisites: identify_prerequisites(&concept_name),
                    related_concepts: Vec::new(), // Will be populated later
                    tags: generate_concept_tags(&concept_name),
                });
            }
        }
    });

    // Sort by importance (frequency * confidence)
    concepts.sort_by(|a, b| {
        let score_a = a.frequency as f64 * a.confidence_score;
        let score_b = b.frequency as f64 * b.confidence_score;
        score_b.partial_cmp(&score_a).unwrap()
    });

    concepts.truncate(50); // Keep top 50 concepts
    concepts
}

// Helper functions for technology identification
fn extract_technologies_from_text(text: &str) -> Vec<String> {
    let mut technologies = Vec::new();
    
    // Programming languages
    let lang_patterns = vec![
        r"(?i)\b(javascript|js|typescript|ts|python|java|kotlin|swift|rust|go|golang|php|ruby|c\+\+|cpp|c#|csharp)\b",
        r"(?i)\b(scala|clojure|elixir|erlang|haskell|perl|r\b|matlab|julia|dart)\b",
    ];
    
    // Frameworks and libraries
    let framework_patterns = vec![
        r"(?i)\b(react|reactjs|vue|vuejs|angular|angularjs|svelte|next|nextjs|nuxt|gatsby)\b",
        r"(?i)\b(express|fastify|koa|spring|django|flask|rails|laravel|symfony)\b",
        r"(?i)\b(bootstrap|tailwind|material-ui|mui|chakra|antd|semantic)\b",
    ];
    
    // Tools and platforms
    let tool_patterns = vec![
        r"(?i)\b(docker|kubernetes|k8s|jenkins|github|gitlab|bitbucket|jira|confluence)\b",
        r"(?i)\b(aws|azure|gcp|firebase|heroku|vercel|netlify|cloudflare)\b",
        r"(?i)\b(webpack|vite|rollup|parcel|babel|eslint|prettier|jest|cypress)\b",
    ];
    
    // Databases
    let db_patterns = vec![
        r"(?i)\b(mysql|postgresql|postgres|mongodb|redis|elasticsearch|sqlite|oracle)\b",
        r"(?i)\b(cassandra|dynamodb|neo4j|influxdb|prometheus|grafana)\b",
    ];
    
    let all_patterns = [lang_patterns, framework_patterns, tool_patterns, db_patterns].concat();
    
    for pattern in all_patterns {
        let re = Regex::new(pattern).unwrap();
        for mat in re.find_iter(text) {
            let tech = mat.as_str().to_lowercase();
            if !technologies.contains(&tech) {
                technologies.push(tech);
            }
        }
    }
    
    technologies
}

fn categorize_technology(tech: &str) -> TechCategory {
    match tech.to_lowercase().as_str() {
        "javascript" | "typescript" | "python" | "java" | "rust" | "go" | "php" | "ruby" => TechCategory::Language,
        "react" | "vue" | "angular" | "django" | "flask" | "spring" | "rails" => TechCategory::Framework,
        "docker" | "kubernetes" | "jenkins" | "webpack" | "vite" => TechCategory::Tool,
        "aws" | "azure" | "gcp" | "heroku" | "vercel" => TechCategory::Platform,
        "mysql" | "postgresql" | "mongodb" | "redis" | "elasticsearch" => TechCategory::Database,
        "bootstrap" | "tailwind" | "material-ui" | "jquery" => TechCategory::Library,
        _ => TechCategory::Tool,
    }
}

fn calculate_team_expertise(tech: &str, frequency: i32) -> f64 {
    let base_expertise = (frequency as f64 / 10.0).min(8.0);
    let complexity_modifier = match tech.to_lowercase().as_str() {
        "kubernetes" | "aws" | "microservices" => -1.0,
        "docker" | "react" | "typescript" => 0.5,
        "html" | "css" | "javascript" => 1.0,
        _ => 0.0,
    };
    (base_expertise + complexity_modifier).max(1.0).min(10.0)
}

fn extract_version_info(tech: &str, contexts: &[String]) -> Option<String> {
    // Create tech-specific patterns first, then fall back to general patterns
    let mut version_patterns = vec![
        format!(r"(?i){}\s+v?(\d+\.\d+(?:\.\d+)?)", regex::escape(tech)),
        format!(r"(?i){}\s+version\s+(\d+\.\d+(?:\.\d+)?)", regex::escape(tech)),
        format!(r"(?i){}-(\d+\.\d+(?:\.\d+)?)", regex::escape(tech)),
    ];
    
    // Add general patterns as fallback
    version_patterns.extend(vec![
        r"(?i)v?\d+\.\d+(?:\.\d+)?".to_string(),
        r"(?i)version\s+\d+\.\d+".to_string(),
    ]);
    
    for context in contexts {
        // Check if context mentions the technology
        let context_lower = context.to_lowercase();
        let tech_lower = tech.to_lowercase();
        
        if context_lower.contains(&tech_lower) {
            for pattern in &version_patterns {
                let re = Regex::new(pattern).unwrap();
                if let Some(mat) = re.find(context) {
                    return Some(mat.as_str().to_string());
                }
            }
        }
    }
    None
}

fn generate_usage_context(tech: &str, contexts: &[String]) -> String {
    if contexts.is_empty() {
        return format!("{} usage in project development", tech);
    }
    
    let first_context = contexts.first().unwrap();
    format!("{} - commonly used in: {}", tech, 
            first_context.chars().take(80).collect::<String>())
}

fn calculate_success_rate(tech: &str, issues: &[String], total_usage: i32) -> f64 {
    if total_usage == 0 { return 0.85; }
    
    // Tech-specific adjustments based on complexity and maturity
    let base_success_rate = match tech.to_lowercase().as_str() {
        // Well-established technologies typically have higher success rates
        tech if tech.contains("react") || tech.contains("javascript") || tech.contains("python") => 0.9,
        // Newer or more complex technologies might have lower success rates
        tech if tech.contains("kubernetes") || tech.contains("docker") || tech.contains("microservice") => 0.75,
        // Database technologies usually have good stability
        tech if tech.contains("postgres") || tech.contains("mysql") || tech.contains("redis") => 0.85,
        // Default rate for unknown technologies
        _ => 0.8,
    };
    
    let issue_rate = issues.len() as f64 / total_usage as f64;
    let adjusted_rate = base_success_rate * (1.0 - issue_rate.min(0.4));
    adjusted_rate.max(0.5).min(0.98)
}

fn extract_common_mistakes(tech: &str, issues: &[String]) -> Vec<String> {
    let mut mistakes = Vec::new();
    
    for issue in issues.iter().take(3) {
        if issue.to_lowercase().contains("error") || issue.to_lowercase().contains("bug") {
            mistakes.push(issue.split(":").last().unwrap_or(issue).trim().to_string());
        }
    }
    
    if mistakes.is_empty() {
        match tech.to_lowercase().as_str() {
            "react" => mistakes.push("Mutating state directly instead of using setState".to_string()),
            "javascript" => mistakes.push("Not handling async operations properly".to_string()),
            "docker" => mistakes.push("Not optimizing image sizes".to_string()),
            _ => mistakes.push("Configuration and setup issues".to_string()),
        }
    }
    
    mistakes
}

fn generate_solutions(tech: &str, issue: &str) -> Vec<String> {
    let mut solutions = Vec::new();
    
    if issue.to_lowercase().contains("error") {
        solutions.push("Check logs for detailed error messages".to_string());
        solutions.push("Verify configuration and dependencies".to_string());
    }
    
    match tech.to_lowercase().as_str() {
        "react" => {
            solutions.push("Use React DevTools for debugging".to_string());
            solutions.push("Check component lifecycle and state management".to_string());
        },
        "docker" => {
            solutions.push("Rebuild image with --no-cache flag".to_string());
            solutions.push("Check Dockerfile syntax and base image".to_string());
        },
        _ => {
            solutions.push("Review documentation and best practices".to_string());
        }
    }
    
    solutions
}

fn generate_prevention_tips(tech: &str) -> Vec<String> {
    match tech.to_lowercase().as_str() {
        "react" => vec![
            "Use TypeScript for better type safety".to_string(),
            "Implement proper error boundaries".to_string(),
            "Follow React hooks best practices".to_string(),
        ],
        "docker" => vec![
            "Use multi-stage builds for optimization".to_string(),
            "Implement proper health checks".to_string(),
            "Use .dockerignore to reduce build context".to_string(),
        ],
        _ => vec![
            "Follow established coding standards".to_string(),
            "Implement comprehensive testing".to_string(),
            "Regular code reviews and documentation".to_string(),
        ],
    }
}

fn generate_best_practices(tech: &str) -> Vec<BestPractice> {
    match tech.to_lowercase().as_str() {
        "react" => vec![
            BestPractice {
                practice: "Use functional components with hooks".to_string(),
                rationale: "Modern React pattern with better performance".to_string(),
                examples: vec!["useState, useEffect, custom hooks".to_string()],
                benefits: vec!["Less boilerplate".to_string(), "Better testability".to_string()],
            }
        ],
        "docker" => vec![
            BestPractice {
                practice: "Use multi-stage builds".to_string(),
                rationale: "Reduces final image size and improves security".to_string(),
                examples: vec!["Separate build and runtime stages".to_string()],
                benefits: vec!["Smaller images".to_string(), "Better security".to_string()],
            }
        ],
        _ => vec![
            BestPractice {
                practice: "Follow established patterns".to_string(),
                rationale: "Consistency and maintainability".to_string(),
                examples: vec!["Standard project structure".to_string()],
                benefits: vec!["Team efficiency".to_string(), "Easier onboarding".to_string()],
            }
        ],
    }
}

fn generate_learning_resources(tech: &str) -> Vec<String> {
    match tech.to_lowercase().as_str() {
        "react" => vec![
            "Official React Documentation".to_string(),
            "React Patterns and Best Practices".to_string(),
            "Modern React Tutorials".to_string(),
        ],
        "docker" => vec![
            "Docker Official Documentation".to_string(),
            "Docker Best Practices Guide".to_string(),
            "Container Security Guidelines".to_string(),
        ],
        _ => vec![
            "Official documentation".to_string(),
            "Community tutorials and guides".to_string(),
            "Best practices documentation".to_string(),
        ],
    }
}

fn assess_skill_level(tech: &str) -> String {
    match tech.to_lowercase().as_str() {
        "kubernetes" | "microservices" | "distributed systems" => "advanced".to_string(),
        "docker" | "react" | "typescript" | "api design" => "intermediate".to_string(),
        "html" | "css" | "basic programming" => "beginner".to_string(),
        _ => "intermediate".to_string(),
    }
}

fn assess_adoption_trend(tech: &str, frequency: i32) -> String {
    // Adjust thresholds based on technology type and expected adoption patterns
    let (emerging_threshold, stable_threshold, growing_threshold) = match tech.to_lowercase().as_str() {
        // Frontend frameworks tend to have higher adoption rates
        tech if tech.contains("react") || tech.contains("vue") || tech.contains("angular") => (5, 15, 30),
        // Infrastructure tools have different adoption patterns
        tech if tech.contains("kubernetes") || tech.contains("docker") => (3, 8, 20),
        // Database technologies typically have slower but steady adoption
        tech if tech.contains("postgres") || tech.contains("mysql") => (2, 6, 15),
        // Languages and core technologies
        tech if tech.contains("python") || tech.contains("javascript") || tech.contains("java") => (8, 25, 50),
        // Default thresholds for unknown technologies
        _ => (3, 10, 20),
    };
    
    match frequency {
        f if f >= growing_threshold => "growing".to_string(),
        f if f >= stable_threshold => "stable".to_string(),
        f if f >= emerging_threshold => "emerging".to_string(),
        _ => "experimental".to_string(),
    }
}

fn find_related_technologies(tech: &str, all_techs: &HashMap<String, i32>) -> Vec<String> {
    let mut related = Vec::new();
    
    let related_groups = match tech.to_lowercase().as_str() {
        "react" => vec!["javascript", "typescript", "jsx", "webpack", "babel"],
        "docker" => vec!["kubernetes", "container", "devops", "ci/cd"],
        "javascript" => vec!["typescript", "node", "npm", "webpack"],
        "python" => vec!["django", "flask", "pip", "virtualenv"],
        _ => vec![],
    };
    
    for rel_tech in related_groups {
        if all_techs.contains_key(rel_tech) {
            related.push(rel_tech.to_string());
        }
    }
    
    related
}

fn extract_concepts_from_text(text: &str) -> Vec<String> {
    let mut concepts = Vec::new();
    
    // Technology patterns
    let tech_patterns = vec![
        r"(?i)\b(react|vue|angular|javascript|typescript|python|rust|java|golang|php)\b",
        r"(?i)\b(docker|kubernetes|aws|azure|gcp|terraform|jenkins|git)\b", 
        r"(?i)\b(mysql|postgresql|mongodb|redis|elasticsearch|kafka)\b",
        r"(?i)\b(api|rest|graphql|microservices|serverless|websocket)\b",
        r"(?i)\b(authentication|authorization|oauth|jwt|security|ssl|https)\b",
        r"(?i)\b(performance|optimization|caching|scaling|load\s+balancing)\b",
        r"(?i)\b(testing|unit\s+test|integration\s+test|e2e|tdd|bdd)\b",
        r"(?i)\b(ci/cd|deployment|devops|monitoring|logging|alerting)\b",
        r"(?i)\b(database|migration|schema|orm|sql|nosql)\b",
        r"(?i)\b(frontend|backend|full\s+stack|ui|ux|responsive)\b",
    ];

    for pattern in tech_patterns {
        let re = Regex::new(pattern).unwrap();
        for mat in re.find_iter(text) {
            let concept = mat.as_str().to_lowercase();
            if !concepts.contains(&concept) {
                concepts.push(concept);
            }
        }
    }

    // Architecture patterns
    let arch_patterns = vec![
        r"(?i)\b(microservices|monolith|event\s+driven|message\s+queue|pub\s*sub)\b",
        r"(?i)\b(design\s+pattern|singleton|factory|observer|strategy)\b",
        r"(?i)\b(clean\s+architecture|hexagonal|layered|mvc|mvvm)\b",
        r"(?i)\b(domain\s+driven|event\s+sourcing|cqrs|saga)\b",
    ];

    for pattern in arch_patterns {
        let re = Regex::new(pattern).unwrap();
        for mat in re.find_iter(text) {
            let concept = mat.as_str().to_lowercase();
            if !concepts.contains(&concept) {
                concepts.push(concept);
            }
        }
    }

    // Process and methodology concepts
    let process_patterns = vec![
        r"(?i)\b(agile|scrum|kanban|sprint|retrospective|standup)\b",
        r"(?i)\b(code\s+review|pair\s+programming|mob\s+programming)\b",
        r"(?i)\b(refactoring|technical\s+debt|legacy|migration)\b",
        r"(?i)\b(documentation|wiki|knowledge\s+sharing|onboarding)\b",
    ];

    for pattern in process_patterns {
        let re = Regex::new(pattern).unwrap();
        for mat in re.find_iter(text) {
            let concept = mat.as_str().to_lowercase();
            if !concepts.contains(&concept) {
                concepts.push(concept);
            }
        }
    }

    concepts
}

fn categorize_concept(concept: &str) -> ConceptCategory {
    match concept.to_lowercase().as_str() {
        s if s.contains("react") || s.contains("vue") || s.contains("angular") || 
             s.contains("javascript") || s.contains("typescript") || s.contains("python") ||
             s.contains("rust") || s.contains("java") || s.contains("golang") => ConceptCategory::Technical,
             
        s if s.contains("architecture") || s.contains("microservices") || s.contains("design") => ConceptCategory::Architecture,
        
        s if s.contains("security") || s.contains("auth") || s.contains("ssl") => ConceptCategory::Security,
        
        s if s.contains("performance") || s.contains("optimization") || s.contains("caching") => ConceptCategory::Performance,
        
        s if s.contains("testing") || s.contains("test") || s.contains("tdd") => ConceptCategory::Testing,
        
        s if s.contains("devops") || s.contains("deployment") || s.contains("ci") || s.contains("docker") => ConceptCategory::DevOps,
        
        s if s.contains("ui") || s.contains("ux") || s.contains("frontend") || s.contains("responsive") => ConceptCategory::UserExperience,
        
        s if s.contains("agile") || s.contains("scrum") || s.contains("process") => ConceptCategory::Process,
        
        _ => ConceptCategory::Business,
    }
}

fn calculate_concept_confidence(frequency: i32, concept: &str) -> f64 {
    let base_confidence = (frequency as f64 / 10.0).min(1.0);
    let length_bonus = if concept.len() > 5 { 0.1 } else { 0.0 };
    let specificity_bonus = if concept.contains("_") || concept.contains("-") { 0.1 } else { 0.0 };
    
    (base_confidence + length_bonus + specificity_bonus).min(1.0)
}

fn generate_concept_description(concept: &str, contexts: &[String]) -> String {
    if contexts.is_empty() {
        return format!("Technical concept: {}", concept);
    }
    
    let first_context = contexts.first().unwrap();
    format!("Concept '{}' commonly appears in contexts like: {}", concept, 
            first_context.chars().take(150).collect::<String>())
}

fn assess_learning_difficulty(concept: &str) -> String {
    match concept.to_lowercase().as_str() {
        s if s.contains("kubernetes") || s.contains("microservices") || s.contains("architecture") => "advanced".to_string(),
        s if s.contains("docker") || s.contains("api") || s.contains("testing") => "intermediate".to_string(),
        _ => "beginner".to_string(),
    }
}

fn identify_prerequisites(concept: &str) -> Vec<String> {
    match concept.to_lowercase().as_str() {
        s if s.contains("react") => vec!["javascript".to_string(), "html".to_string(), "css".to_string()],
        s if s.contains("kubernetes") => vec!["docker".to_string(), "containerization".to_string(), "networking".to_string()],
        s if s.contains("microservices") => vec!["api design".to_string(), "distributed systems".to_string(), "networking".to_string()],
        s if s.contains("testing") => vec!["programming fundamentals".to_string()],
        _ => vec!["basic programming".to_string()],
    }
}

fn generate_concept_tags(concept: &str) -> Vec<String> {
    let mut tags = Vec::new();
    
    if concept.contains("react") || concept.contains("vue") || concept.contains("angular") {
        tags.push("frontend".to_string());
        tags.push("framework".to_string());
    }
    
    if concept.contains("docker") || concept.contains("kubernetes") {
        tags.push("containerization".to_string());
        tags.push("devops".to_string());
    }
    
    if concept.contains("api") || concept.contains("rest") {
        tags.push("integration".to_string());
        tags.push("backend".to_string());
    }
    
    if concept.contains("test") {
        tags.push("quality".to_string());
        tags.push("automation".to_string());
    }
    
    tags
}

async fn identify_technologies() -> Vec<TechnologyKnowledge> {
    log_step("💻", "Identifying technologies and their usage patterns...");
    
    let mut technologies = Vec::new();
    let mut tech_frequency: HashMap<String, i32> = HashMap::new();
    let mut tech_contexts: HashMap<String, Vec<String>> = HashMap::new();
    let mut tech_issues: HashMap<String, Vec<String>> = HashMap::new();
    
    with_connection("identify_technologies", |conn| {
        let query = r#"
        SELECT 
            key, summary, description, comment, status,
            JSON_EXTRACT(project, '$.name') as project_name
        FROM issues 
        WHERE (description IS NOT NULL AND description != '{}' AND description != '') 
           OR (comment IS NOT NULL AND comment != '{}' AND comment != '')
        ORDER BY created DESC
        LIMIT 300
        "#;

        let mut stmt = conn.prepare(query).expect("Failed to prepare tech query");
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0).unwrap_or_default(),
                row.get::<_, String>(1).unwrap_or_default(),
                row.get::<_, String>(2).unwrap_or_default(),
                row.get::<_, String>(3).unwrap_or_default(),
                row.get::<_, String>(4).unwrap_or_default(),
                row.get::<_, String>(5).unwrap_or_default(),
            ))
        }).expect("Failed to execute tech query");

        for row in rows {
            if let Ok((key, summary, description, comment, status, project)) = row {
                let full_text = format!("{} {} {} {}", summary, description, comment, project);
                let identified_techs = extract_technologies_from_text(&full_text);
                
                for tech in identified_techs {
                    *tech_frequency.entry(tech.clone()).or_insert(0) += 1;
                    tech_contexts.entry(tech.clone()).or_default()
                        .push(format!("{}: {}", key, summary.chars().take(100).collect::<String>()));
                    
                    if status.contains("bug") || status.contains("error") || summary.to_lowercase().contains("error") {
                        tech_issues.entry(tech.clone()).or_default()
                            .push(format!("{}: {}", key, summary));
                    }
                }
            }
        }
    });

    // Convert to structured technology knowledge
    for (tech_name, frequency) in &tech_frequency {
        if *frequency >= 3 {
            let category = categorize_technology(&tech_name);
            let expertise_level = calculate_team_expertise(&tech_name, *frequency);
            let empty_vec = Vec::new();
            let issues = tech_issues.get(tech_name.as_str()).unwrap_or(&empty_vec);
            let contexts = tech_contexts.get(tech_name.as_str()).unwrap_or(&empty_vec);
            
            technologies.push(TechnologyKnowledge {
                id: format!("tech_{}", tech_name.replace(" ", "_").to_lowercase()),
                name: tech_name.clone(),
                category,
                version_info: extract_version_info(&tech_name, contexts),
                usage_patterns: vec![
                    UsagePattern {
                        context: generate_usage_context(&tech_name, contexts),
                        frequency: *frequency,
                        success_rate: calculate_success_rate(&tech_name, issues, *frequency),
                        common_mistakes: extract_common_mistakes(&tech_name, issues),
                    }
                ],
                common_issues: issues.iter().take(5).map(|issue| {
                    CommonIssue {
                        issue: issue.clone(),
                        solutions: generate_solutions(&tech_name, issue),
                        prevention: generate_prevention_tips(&tech_name),
                        frequency: 1,
                    }
                }).collect(),
                best_practices: generate_best_practices(&tech_name),
                learning_resources: generate_learning_resources(&tech_name),
                skill_level_required: assess_skill_level(&tech_name),
                team_expertise_level: expertise_level,
                adoption_trend: assess_adoption_trend(&tech_name, *frequency),
                related_technologies: find_related_technologies(&tech_name, &tech_frequency),
            });
        }
    }

    // Sort by usage frequency
    technologies.sort_by(|a, b| {
        let usage_a = a.usage_patterns.first().map(|p| p.frequency).unwrap_or(0);
        let usage_b = b.usage_patterns.first().map(|p| p.frequency).unwrap_or(0);
        usage_b.cmp(&usage_a)
    });
    
    technologies.truncate(20); // Keep top 20 technologies
    technologies
}

async fn discover_patterns() -> Vec<KnowledgePattern> {
    log_step("🔍", "Discovering knowledge and solution patterns...");
    
    let mut patterns = Vec::new();
    let mut pattern_frequency: HashMap<String, Vec<String>> = HashMap::new();
    
    with_connection("discover_patterns", |conn| {
        let query = r#"
        SELECT key, summary, description, comment, status,
               resolution, JSON_EXTRACT(project, '$.name') as project_name
        FROM issues 
        WHERE status LIKE '%resolved%' OR status LIKE '%closed%' OR status LIKE '%done%'
        ORDER BY created DESC
        LIMIT 200
        "#;

        let mut stmt = conn.prepare(query).expect("Failed to prepare patterns query");
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0).unwrap_or_default(),
                row.get::<_, String>(1).unwrap_or_default(),
                row.get::<_, String>(2).unwrap_or_default(),
                row.get::<_, String>(3).unwrap_or_default(),
                row.get::<_, String>(4).unwrap_or_default(),
                row.get::<_, String>(5).unwrap_or_default(),
                row.get::<_, String>(6).unwrap_or_default(),
            ))
        }).expect("Failed to execute patterns query");

        for row in rows {
            if let Ok((key, summary, description, comment, _status, resolution, project)) = row {
                let full_text = format!("{} {} {} {} {}", summary, description, comment, resolution, project);
                let detected_patterns = detect_solution_patterns(&full_text, &key, &summary);
                
                for pattern in detected_patterns {
                    pattern_frequency.entry(pattern).or_default().push(format!("{}: {}", key, summary));
                }
            }
        }
    });

    // Convert to structured patterns
    for (pattern_name, examples) in pattern_frequency {
        if examples.len() >= 2 {
            let pattern_type = categorize_pattern(&pattern_name);
            let effectiveness = calculate_pattern_effectiveness(&pattern_name, &examples);
            
            patterns.push(KnowledgePattern {
                id: format!("pattern_{}", patterns.len()),
                pattern_type,
                name: pattern_name.clone(),
                description: generate_pattern_description(&pattern_name, &examples),
                examples: examples.iter().take(3).map(|ex| {
                    PatternExample {
                        title: ex.split(":").next().unwrap_or("Example").to_string(),
                        description: ex.clone(),
                        code_example: extract_code_example(&pattern_name),
                        outcome: "Resolved successfully".to_string(),
                        lessons_learned: generate_lessons_learned(&pattern_name),
                    }
                }).collect(),
                effectiveness_score: effectiveness,
                usage_frequency: examples.len() as i32,
                success_rate: 0.85, // Default success rate
                anti_patterns: generate_anti_patterns(&pattern_name),
                when_to_use: generate_when_to_use(&pattern_name),
                when_not_to_use: generate_when_not_to_use(&pattern_name),
            });
        }
    }

    // Sort by usage frequency and effectiveness
    patterns.sort_by(|a, b| {
        let score_a = a.usage_frequency as f64 * a.effectiveness_score;
        let score_b = b.usage_frequency as f64 * b.effectiveness_score;
        score_b.partial_cmp(&score_a).unwrap()
    });
    
    patterns.truncate(15); // Keep top 15 patterns
    patterns
}

async fn generate_learning_materials() -> Vec<LearningMaterial> {
    log_step("📚", "Generating learning materials from issue knowledge...");
    
    let mut materials = Vec::new();
    let mut content_groups: HashMap<String, Vec<String>> = HashMap::new();
    
    with_connection("generate_learning_materials", |conn| {
        let query = r#"
        SELECT key, summary, description, comment, status, resolution,
               JSON_EXTRACT(project, '$.name') as project_name,
               labels
        FROM issues 
        WHERE (description IS NOT NULL AND length(description) > 100)
           OR (comment IS NOT NULL AND length(comment) > 100)
           OR (resolution IS NOT NULL AND length(resolution) > 50)
        ORDER BY created DESC
        LIMIT 150
        "#;

        let mut stmt = conn.prepare(query).expect("Failed to prepare materials query");
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0).unwrap_or_default(),
                row.get::<_, String>(1).unwrap_or_default(),
                row.get::<_, String>(2).unwrap_or_default(),
                row.get::<_, String>(3).unwrap_or_default(),
                row.get::<_, String>(4).unwrap_or_default(),
                row.get::<_, String>(5).unwrap_or_default(),
                row.get::<_, String>(6).unwrap_or_default(),
                row.get::<_, String>(7).unwrap_or_default(),
            ))
        }).expect("Failed to execute materials query");

        for row in rows {
            if let Ok((key, summary, description, comment, status, resolution, project, labels)) = row {
                let _content_type = determine_content_type(&summary, &description, &comment, &status, &labels);
                let topic = extract_topic(&summary, &description, &project);
                
                content_groups.entry(topic).or_default().push(format!(
                    "{} | {} | {} | {} | {}", key, summary, description, comment, resolution
                ));
            }
        }
    });

    // Generate learning materials from content groups
    for (topic, contents) in content_groups {
        if contents.len() >= 2 {
            let synthesized_content = synthesize_learning_content(&topic, &contents);
            let key_learnings = extract_key_learnings(&topic, &contents);
            let difficulty = assess_content_difficulty(&topic, &contents);
            let reading_time = estimate_reading_time(&synthesized_content);
            
            materials.push(LearningMaterial {
                id: format!("material_{}", materials.len()),
                title: format!("Learning Guide: {}", format_topic_title(&topic)),
                content_type: determine_material_type(&topic, &contents),
                source_issues: contents.iter().map(|c| 
                    c.split(" | ").next().unwrap_or("unknown").to_string()
                ).collect(),
                extracted_content: synthesized_content,
                key_learnings,
                difficulty_level: difficulty,
                estimated_reading_time: reading_time,
                prerequisites: identify_content_prerequisites(&topic),
                related_materials: Vec::new(), // Will be linked later
                quality_score: calculate_content_quality(&topic, &contents),
                last_updated: Utc::now().to_rfc3339(),
            });
        }
    }

    // Sort by quality score and relevance
    materials.sort_by(|a, b| {
        let score_a = a.quality_score * (a.source_issues.len() as f64);
        let score_b = b.quality_score * (b.source_issues.len() as f64);
        score_b.partial_cmp(&score_a).unwrap()
    });
    
    materials.truncate(12); // Keep top 12 materials
    materials
}

async fn detect_knowledge_gaps(_concepts: &[KnowledgeConcept], technologies: &[TechnologyKnowledge]) -> Vec<KnowledgeGap> {
    log_step("🔍", "Detecting knowledge gaps and learning opportunities...");
    
    let mut gaps = Vec::new();
    
    // Detect areas with low expertise but high usage
    for tech in technologies {
        if tech.team_expertise_level < 6.0 && tech.usage_patterns.iter().any(|p| p.frequency > 10) {
            gaps.push(KnowledgeGap {
                id: format!("gap_{}", gaps.len()),
                gap_type: GapType::Skills,
                title: format!("Low expertise in high-usage technology: {}", tech.name),
                description: format!("Team expertise level is {:.1}/10 but usage frequency is high", tech.team_expertise_level),
                severity: 8.5,
                affected_areas: vec![tech.name.clone()],
                potential_impact: "Increased bug rates and slower development".to_string(),
                suggested_actions: vec![
                    format!("Organize {} training sessions", tech.name),
                    "Pair experienced developers with learners".to_string(),
                    "Create internal documentation and best practices guide".to_string(),
                ],
                learning_priority: 1,
                estimated_effort: "2-4 weeks".to_string(),
            });
        }
    }
    
    gaps
}

async fn analyze_team_expertise(_concepts: &[KnowledgeConcept], _technologies: &[TechnologyKnowledge]) -> HashMap<String, Vec<ExpertiseArea>> {
    log_step("👥", "Analyzing team expertise distribution...");
    
    // Implementation would analyze who works on what issues to build expertise map
    HashMap::new()
}

async fn build_semantic_graph(_concepts: &[KnowledgeConcept], _patterns: &[KnowledgePattern]) -> KnowledgeSemanticGraph {
    log_step("🕸️", "Building semantic knowledge graph...");
    
    KnowledgeSemanticGraph {
        concept_relationships: Vec::new(),
        learning_paths: Vec::new(),
        knowledge_clusters: Vec::new(),
        expertise_networks: Vec::new(),
    }
}

async fn generate_knowledge_insights(
    concepts: &[KnowledgeConcept], 
    gaps: &[KnowledgeGap], 
    _expertise: &HashMap<String, Vec<ExpertiseArea>>
) -> Vec<KnowledgeInsight> {
    log_step("💡", "Generating knowledge insights...");
    
    let mut insights = Vec::new();
    
    // Learning opportunity insights
    if !concepts.is_empty() {
        let top_concept = &concepts[0];
        insights.push(KnowledgeInsight {
            id: format!("insight_{}", insights.len()),
            insight_type: InsightType::LearningOpportunity,
            title: format!("High-value learning opportunity: {}", top_concept.name),
            description: format!(
                "Concept '{}' appears in {} issues with {:.0}% confidence. Mastering this could significantly accelerate team learning.",
                top_concept.name, top_concept.frequency, top_concept.confidence_score * 100.0
            ),
            confidence: top_concept.confidence_score,
            impact: "high".to_string(),
            action_items: vec![
                format!("Create comprehensive guide for '{}'", top_concept.name),
                "Schedule team learning session".to_string(),
                "Identify internal experts to lead training".to_string(),
            ],
            supporting_evidence: vec![
                format!("Appears in {} different issues", top_concept.frequency),
                format!("Confidence score: {:.0}%", top_concept.confidence_score * 100.0),
                format!("Learning difficulty: {}", top_concept.learning_difficulty),
            ],
            generated_at: Utc::now().to_rfc3339(),
        });
    }
    
    // Knowledge gap insights
    for gap in gaps {
        insights.push(KnowledgeInsight {
            id: format!("insight_{}", insights.len()),
            insight_type: InsightType::KnowledgeRisk,
            title: gap.title.clone(),
            description: format!("Critical knowledge gap detected with severity {:.1}/10: {}", gap.severity, gap.description),
            confidence: 0.85,
            impact: "high".to_string(),
            action_items: gap.suggested_actions.clone(),
            supporting_evidence: vec![
                format!("Severity: {:.1}/10", gap.severity),
                format!("Affected areas: {}", gap.affected_areas.join(", ")),
                format!("Estimated effort: {}", gap.estimated_effort),
            ],
            generated_at: Utc::now().to_rfc3339(),
        });
    }
    
    insights
}

// Helper functions for pattern discovery
fn detect_solution_patterns(text: &str, key: &str, summary: &str) -> Vec<String> {
    let mut patterns = Vec::new();
    
    // Combine all context for comprehensive pattern detection
    let combined_context = format!("{} {} {}", key, summary, text).to_lowercase();
    
    // Use key prefix to determine issue type and likely patterns
    let key_patterns = if key.starts_with("BUG") || key.contains("bug") {
        vec![
            ("Bug Investigation", vec!["debug", "trace", "log", "reproduce"]),
            ("Root Cause Analysis", vec!["cause", "investigation", "analysis"]),
            ("Error Handling", vec!["error", "exception", "failure"]),
        ]
    } else if key.starts_with("FEAT") || key.contains("feature") {
        vec![
            ("Feature Development", vec!["implement", "add", "create", "new"]),
            ("Architecture Design", vec!["design", "structure", "pattern"]),
            ("Integration", vec!["integrate", "connect", "api"]),
        ]
    } else if key.starts_with("PERF") || key.contains("performance") {
        vec![
            ("Performance Optimization", vec!["optimize", "performance", "speed", "memory"]),
            ("Profiling", vec!["profile", "benchmark", "measure"]),
        ]
    } else {
        vec![]
    };
    
    // Check key-based patterns first
    for (pattern_name, keywords) in key_patterns {
        if keywords.iter().any(|kw| combined_context.contains(kw)) {
            patterns.push(pattern_name.to_string());
        }
    }
    
    // Check summary for solution indicators
    let summary_lower = summary.to_lowercase();
    if summary_lower.contains("fix") || summary_lower.contains("resolve") || summary_lower.contains("solve") {
        patterns.push("Problem Resolution".to_string());
    }
    if summary_lower.contains("improve") || summary_lower.contains("enhance") || summary_lower.contains("optimize") {
        patterns.push("Enhancement Strategy".to_string());
    }
    if summary_lower.contains("refactor") || summary_lower.contains("restructure") {
        patterns.push("Code Refactoring".to_string());
    }
    
    // Standard content-based patterns
    if combined_context.contains("config") || combined_context.contains("settings") {
        patterns.push("Configuration Management".to_string());
    }
    if combined_context.contains("api") || combined_context.contains("endpoint") {
        patterns.push("API Integration".to_string());
    }
    if combined_context.contains("database") || combined_context.contains("query") {
        patterns.push("Database Operations".to_string());
    }
    if combined_context.contains("deploy") || combined_context.contains("release") {
        patterns.push("Deployment Process".to_string());
    }
    if combined_context.contains("test") || combined_context.contains("testing") {
        patterns.push("Testing Strategy".to_string());
    }
    if combined_context.contains("security") || combined_context.contains("authentication") {
        patterns.push("Security Implementation".to_string());
    }
    
    patterns.dedup();
    patterns
}

fn categorize_pattern(pattern_name: &str) -> PatternType {
    match pattern_name.to_lowercase().as_str() {
        s if s.contains("design") || s.contains("architecture") => PatternType::Design,
        s if s.contains("problem") || s.contains("error") => PatternType::Problem,
        s if s.contains("solution") || s.contains("optimization") => PatternType::Solution,
        s if s.contains("process") || s.contains("deployment") => PatternType::Process,
        _ => PatternType::Solution,
    }
}

fn calculate_pattern_effectiveness(pattern_name: &str, examples: &[String]) -> f64 {
    let base_effectiveness = 0.7;
    let frequency_bonus = (examples.len() as f64 / 10.0).min(0.2);
    let complexity_modifier = match pattern_name.to_lowercase().as_str() {
        s if s.contains("security") || s.contains("performance") => 0.1,
        s if s.contains("testing") || s.contains("deployment") => 0.05,
        _ => 0.0,
    };
    (base_effectiveness + frequency_bonus + complexity_modifier).min(1.0)
}

fn generate_pattern_description(pattern_name: &str, examples: &[String]) -> String {
    let context = if !examples.is_empty() {
        let first_example = examples.first().unwrap();
        format!(" Commonly seen in contexts like: {}", 
                first_example.chars().take(100).collect::<String>())
    } else {
        String::new()
    };
    
    format!("Pattern '{}' represents a recurring approach to solving similar problems.{}", 
            pattern_name, context)
}

fn extract_code_example(pattern_name: &str) -> Option<String> {
    match pattern_name.to_lowercase().as_str() {
        s if s.contains("api") => Some("fetch('/api/endpoint').then(response => response.json())".to_string()),
        s if s.contains("error") => Some("try { ... } catch (error) { console.error(error); }".to_string()),
        s if s.contains("config") => Some("const config = { apiUrl: process.env.API_URL };".to_string()),
        _ => None,
    }
}

fn generate_lessons_learned(pattern_name: &str) -> Vec<String> {
    match pattern_name.to_lowercase().as_str() {
        s if s.contains("error") => vec![
            "Proper error handling improves user experience".to_string(),
            "Log errors with sufficient context for debugging".to_string(),
        ],
        s if s.contains("performance") => vec![
            "Measure before optimizing".to_string(),
            "Profile to identify actual bottlenecks".to_string(),
        ],
        s if s.contains("security") => vec![
            "Security should be considered from the start".to_string(),
            "Regular security audits are essential".to_string(),
        ],
        _ => vec![
            "Consistent patterns improve maintainability".to_string(),
            "Document decisions and rationale".to_string(),
        ],
    }
}

fn generate_anti_patterns(pattern_name: &str) -> Vec<String> {
    match pattern_name.to_lowercase().as_str() {
        s if s.contains("error") => vec![
            "Silently ignoring errors".to_string(),
            "Generic error messages without context".to_string(),
        ],
        s if s.contains("performance") => vec![
            "Premature optimization".to_string(),
            "Optimizing without measuring".to_string(),
        ],
        _ => vec![
            "Copy-pasting code without understanding".to_string(),
            "Ignoring established patterns without reason".to_string(),
        ],
    }
}

fn generate_when_to_use(pattern_name: &str) -> Vec<String> {
    match pattern_name.to_lowercase().as_str() {
        s if s.contains("error") => vec![
            "When operations can fail".to_string(),
            "When user feedback is important".to_string(),
        ],
        s if s.contains("performance") => vec![
            "When performance issues are identified".to_string(),
            "When scalability is a concern".to_string(),
        ],
        _ => vec![
            "When similar problems occur repeatedly".to_string(),
            "When consistency is important".to_string(),
        ],
    }
}

fn generate_when_not_to_use(pattern_name: &str) -> Vec<String> {
    match pattern_name.to_lowercase().as_str() {
        s if s.contains("optimization") => vec![
            "Before identifying actual performance bottlenecks".to_string(),
            "When it significantly complicates the code".to_string(),
        ],
        _ => vec![
            "When it doesn't fit the specific context".to_string(),
            "When simpler solutions are available".to_string(),
        ],
    }
}

// Helper functions for learning materials
fn determine_content_type(summary: &str, description: &str, comment: &str, status: &str, labels: &str) -> ContentType {
    let full_text = format!("{} {} {} {} {}", summary, description, comment, status, labels).to_lowercase();
    
    if full_text.contains("tutorial") || full_text.contains("how to") {
        ContentType::Tutorial
    } else if full_text.contains("best practice") || full_text.contains("guideline") {
        ContentType::BestPractice
    } else if full_text.contains("troubleshoot") || full_text.contains("debug") {
        ContentType::Troubleshooting
    } else if full_text.contains("architecture") || full_text.contains("design") {
        ContentType::Architecture
    } else if full_text.contains("process") || full_text.contains("workflow") {
        ContentType::ProcessGuide
    } else if full_text.contains("lesson") || full_text.contains("learned") {
        ContentType::LessonsLearned
    } else {
        ContentType::FAQ
    }
}

fn extract_topic(summary: &str, description: &str, project: &str) -> String {
    let combined_text = format!("{} {} {}", summary, description, project);
    let concepts = extract_concepts_from_text(&combined_text);
    
    if !concepts.is_empty() {
        concepts[0].clone()
    } else {
        // Extract first meaningful word as topic
        let words: Vec<&str> = summary.split_whitespace().collect();
        for word in words {
            if word.len() > 3 && !word.to_lowercase().starts_with("the") {
                return word.to_lowercase();
            }
        }
        "general".to_string()
    }
}

fn synthesize_learning_content(topic: &str, contents: &[String]) -> String {
    let mut synthesized = format!("# Learning Guide: {}\n\n", format_topic_title(topic));
    
    synthesized.push_str(&format!("## Overview\n\nThis guide covers key concepts and practices related to {}. ", topic));
    synthesized.push_str(&format!("The following insights are derived from {} real-world issues and their solutions.\n\n", contents.len()));
    
    synthesized.push_str("## Key Points\n\n");
    for (i, content) in contents.iter().take(5).enumerate() {
        let parts: Vec<&str> = content.split(" | ").collect();
        if parts.len() >= 2 {
            synthesized.push_str(&format!("{}. **{}**: {}\n\n", i + 1, parts[0], 
                                        parts[1].chars().take(150).collect::<String>()));
        }
    }
    
    synthesized.push_str("## Common Patterns\n\n");
    synthesized.push_str(&format!("When working with {}, teams frequently encounter similar challenges. ", topic));
    synthesized.push_str("The patterns identified here can help accelerate learning and reduce common mistakes.\n\n");
    
    synthesized
}

fn extract_key_learnings(topic: &str, contents: &[String]) -> Vec<String> {
    let mut learnings = Vec::new();
    
    for content in contents.iter().take(3) {
        let parts: Vec<&str> = content.split(" | ").collect();
        if parts.len() >= 3 && !parts[2].is_empty() {
            let learning = parts[2].chars().take(100).collect::<String>();
            if learning.len() > 20 {
                learnings.push(learning);
            }
        }
    }
    
    if learnings.is_empty() {
        learnings.push(format!("Understanding {} requires hands-on practice", topic));
        learnings.push(format!("Common issues with {} can be prevented with proper setup", topic));
    }
    
    learnings
}

fn assess_content_difficulty(topic: &str, contents: &[String]) -> String {
    let complexity_indicators = contents.iter()
        .map(|c| c.to_lowercase())
        .filter(|c| c.contains("complex") || c.contains("advanced") || c.contains("difficult"))
        .count();
    
    let advanced_topics = ["kubernetes", "microservices", "distributed", "architecture", "security"];
    let is_advanced_topic = advanced_topics.iter().any(|&t| topic.to_lowercase().contains(t));
    
    if complexity_indicators > 1 || is_advanced_topic {
        "advanced".to_string()
    } else if contents.len() > 5 || topic.contains("integration") {
        "intermediate".to_string()
    } else {
        "beginner".to_string()
    }
}

fn estimate_reading_time(content: &str) -> i32 {
    let word_count = content.split_whitespace().count();
    let words_per_minute = 200;
    std::cmp::max(1, word_count / words_per_minute) as i32
}

fn format_topic_title(topic: &str) -> String {
    topic.split_whitespace()
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            if let Some(first) = chars.get_mut(0) {
                *first = first.to_uppercase().nth(0).unwrap_or(*first);
            }
            chars.into_iter().collect()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn determine_material_type(topic: &str, contents: &[String]) -> ContentType {
    let combined = contents.join(" ").to_lowercase();
    let topic_lower = topic.to_lowercase();
    
    // Use topic context to inform material type detection
    if topic_lower.contains("tutorial") || topic_lower.contains("how-to") || topic_lower.contains("guide") {
        return ContentType::Tutorial;
    }
    if topic_lower.contains("troubleshoot") || topic_lower.contains("bug") || topic_lower.contains("error") {
        return ContentType::Troubleshooting;
    }
    if topic_lower.contains("architecture") || topic_lower.contains("design") || topic_lower.contains("pattern") {
        return ContentType::Architecture;
    }
    if topic_lower.contains("best practice") || topic_lower.contains("convention") {
        return ContentType::BestPractice;
    }
    
    // Fall back to content-based detection
    if combined.contains("step by step") || combined.contains("tutorial") || combined.contains("guide") {
        ContentType::Tutorial
    } else if combined.contains("troubleshoot") || combined.contains("error") || combined.contains("fix") || combined.contains("resolve") {
        ContentType::Troubleshooting
    } else if combined.contains("best practice") || combined.contains("recommend") || combined.contains("should") {
        ContentType::BestPractice
    } else if combined.contains("architecture") || combined.contains("design") || combined.contains("pattern") {
        ContentType::Architecture
    } else {
        ContentType::LessonsLearned
    }
}

fn identify_content_prerequisites(topic: &str) -> Vec<String> {
    match topic.to_lowercase().as_str() {
        s if s.contains("react") => vec!["JavaScript basics".to_string(), "HTML/CSS knowledge".to_string()],
        s if s.contains("kubernetes") => vec!["Docker understanding".to_string(), "Container concepts".to_string()],
        s if s.contains("database") => vec!["SQL basics".to_string(), "Data modeling concepts".to_string()],
        s if s.contains("api") => vec!["HTTP protocol understanding".to_string(), "REST concepts".to_string()],
        _ => vec!["Basic programming knowledge".to_string()],
    }
}

fn calculate_content_quality(topic: &str, contents: &[String]) -> f64 {
    let topic_lower = topic.to_lowercase();
    
    // Base quality varies by topic complexity and importance
    let base_quality = if topic_lower.contains("security") || topic_lower.contains("critical") {
        0.8 // Security topics need higher quality bar
    } else if topic_lower.contains("architecture") || topic_lower.contains("design") {
        0.7 // Architecture topics are important
    } else if topic_lower.contains("bug") || topic_lower.contains("error") {
        0.6 // Bug fixes are contextual
    } else if topic_lower.contains("tutorial") || topic_lower.contains("guide") {
        0.75 // Tutorials should be comprehensive
    } else {
        0.6 // Default quality expectation
    };
    
    let content_length_bonus = (contents.len() as f64 / 10.0).min(0.25);
    let detail_bonus = contents.iter()
        .map(|c| if c.len() > 200 { 0.02 } else { 0.0 })
        .sum::<f64>().min(0.15);
    
    // Quality penalties for certain topic types
    let topic_penalty = if topic_lower.contains("quick fix") || topic_lower.contains("workaround") {
        -0.1 // Quick fixes might be lower quality
    } else {
        0.0
    };
    
    (base_quality + content_length_bonus + detail_bonus + topic_penalty).max(0.1).min(1.0)
}