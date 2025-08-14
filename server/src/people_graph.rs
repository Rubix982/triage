use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::db_utils::with_connection;
use crate::utils::{log_step, log_success};

// ================================
// PEOPLE & IDENTITY MANAGEMENT
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: String,                          // UUID
    pub email: String,                       // primary identifier
    pub display_names: Vec<String>,          // all known names/handles
    pub platform_identities: HashMap<String, PlatformIdentity>,
    pub expertise_areas: Vec<ExpertiseScore>,
    pub activity_metrics: ActivityMetrics,
    pub collaboration_network: Vec<CollaborationEdge>,
    pub first_seen: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlatformIdentity {
    pub platform: String,                    // "jira", "slack", "google", "github"
    pub platform_user_id: String,           // "ssmith", "U123456", etc.
    pub display_name: String,                // how they appear on that platform
    pub email: Option<String>,               // platform-specific email if different
    pub profile_url: Option<String>,         // link to their profile
    pub verified: bool,                      // confidence this is the same person
    pub first_seen_on_platform: DateTime<Utc>,
    pub last_active_on_platform: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpertiseScore {
    pub concept: String,                     // "authentication", "redis", "docker"
    pub authority_score: f64,                // 0.0 to 1.0
    pub contribution_count: u32,             // number of contributions
    pub solution_success_rate: f64,          // how often their solutions work
    pub peer_recognition_score: f64,         // how often others reference their work
    pub recency_factor: f64,                 // recent activity in this area
    pub evidence_items: Vec<String>,         // content IDs supporting this expertise
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityMetrics {
    pub response_time_avg_hours: f64,        // average response time
    pub collaboration_frequency: f64,        // how often they work with others
    pub knowledge_sharing_score: f64,        // documentation and explanation tendency
    pub problem_resolution_rate: f64,        // success rate solving problems
    pub cross_platform_activity: u32,       // how many platforms they're active on
    pub mentorship_indicators: u32,          // signs of helping others learn
    pub total_contributions: u32,            // overall activity level
    pub last_calculated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollaborationEdge {
    pub collaborator_id: String,             // other person's ID
    pub collaboration_strength: f64,         // 0.0 to 1.0
    pub shared_projects: Vec<String>,        // projects they've worked on together
    pub interaction_count: u32,              // total interactions
    pub last_collaboration: DateTime<Utc>,
    pub collaboration_types: Vec<CollaborationType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CollaborationType {
    CoAuthored,                              // worked on same content
    ReviewerReviewee,                        // one reviewed other's work
    MentorMentee,                           // teaching/learning relationship
    ProblemSolver,                          // one helped other solve problem
    FrequentReplier,                        // often respond to each other
    ProjectTeammate,                        // same project/team
    CrossPlatformBridge,                    // bridge each other across platforms
}

// ================================
// DETAILED INTERACTION TRACKING
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetailedInteraction {
    pub id: String,
    pub interaction_type: InteractionType,
    pub source_person_id: String,           // who initiated
    pub target_person_id: Option<String>,   // who they interacted with
    pub content_id: String,                 // what content this relates to
    pub platform: String,                   // where this happened
    pub timestamp: DateTime<Utc>,
    pub context: InteractionContext,
    pub impact_indicators: ImpactIndicators,
    pub extracted_data: serde_json::Value,  // platform-specific raw data
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InteractionType {
    // Content creation
    CreatedContent,
    EditedContent,
    CommentedOn { parent_content_id: String },
    RepliedTo { original_comment_id: String },
    
    // Social interactions
    Mentioned { mentioned_person_ids: Vec<String> },
    ReactedWith { reaction_type: String },
    SharedContent { shared_content_id: String },
    TaggedPerson { tagged_person_ids: Vec<String> },
    
    // Problem solving
    AskedQuestion { question_text: String },
    ProvidedSolution { solution_text: String },
    ImplementedSolution { implementation_details: String },
    ValidatedSolution { validation_outcome: bool },
    
    // Workflow
    AssignedTask { task_description: String },
    ReviewedWork { review_outcome: String },
    ApprovedChange { approval_details: String },
    TransitionedStatus { from_status: String, to_status: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InteractionContext {
    pub thread_id: Option<String>,          // slack thread, jira comment chain
    pub urgency_indicators: Vec<String>,    // "urgent", "high priority", etc.
    pub topic_keywords: Vec<String>,        // detected concepts/technologies
    pub audience_size: Option<u32>,         // how many people could see this
    pub visibility_level: String,           // "public", "team", "private"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImpactIndicators {
    pub reply_count: u32,                   // how many replies this generated
    pub reaction_count: u32,                // how many reactions received
    pub reference_count: u32,               // how often this gets referenced later
    pub implementation_count: u32,          // if solution, how often implemented
    pub view_count: Option<u32>,           // if available from platform
    pub share_count: u32,                  // how often shared/forwarded
    pub problem_resolution: Option<bool>,   // did this solve the problem?
}

// ================================
// DATABASE SCHEMA
// ================================

pub const CREATE_PEOPLE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS people (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    display_names TEXT NOT NULL, -- JSON array
    platform_identities TEXT NOT NULL, -- JSON HashMap
    expertise_areas TEXT, -- JSON array
    activity_metrics TEXT, -- JSON object
    collaboration_network TEXT, -- JSON array
    first_seen TIMESTAMP NOT NULL,
    last_active TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
"#;

pub const CREATE_INTERACTIONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS detailed_interactions (
    id TEXT PRIMARY KEY,
    interaction_type TEXT NOT NULL,
    source_person_id TEXT NOT NULL,
    target_person_id TEXT,
    content_id TEXT NOT NULL,
    platform TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    context TEXT, -- JSON InteractionContext
    impact_indicators TEXT, -- JSON ImpactIndicators
    extracted_data TEXT, -- JSON platform-specific data
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (source_person_id) REFERENCES people(id),
    FOREIGN KEY (target_person_id) REFERENCES people(id)
);
"#;

pub const CREATE_PEOPLE_INDEXES: &str = r#"
CREATE INDEX IF NOT EXISTS idx_people_email ON people(email);
CREATE INDEX IF NOT EXISTS idx_people_last_active ON people(last_active);
CREATE INDEX IF NOT EXISTS idx_interactions_source_person ON detailed_interactions(source_person_id);
CREATE INDEX IF NOT EXISTS idx_interactions_target_person ON detailed_interactions(target_person_id);
CREATE INDEX IF NOT EXISTS idx_interactions_content ON detailed_interactions(content_id);
CREATE INDEX IF NOT EXISTS idx_interactions_platform ON detailed_interactions(platform);
CREATE INDEX IF NOT EXISTS idx_interactions_timestamp ON detailed_interactions(timestamp);
CREATE INDEX IF NOT EXISTS idx_interactions_type ON detailed_interactions(interaction_type);
"#;

// ================================
// IDENTITY RESOLUTION SERVICE
// ================================

pub struct IdentityResolver {
    confidence_threshold: f64,
}

impl IdentityResolver {
    pub fn new() -> Self {
        Self {
            confidence_threshold: 0.8, // 80% confidence required to merge identities
        }
    }

    pub async fn resolve_identity(&self, 
        email: Option<&str>, 
        display_name: &str, 
        platform: &str,
        platform_user_id: &str
    ) -> Result<String, Box<dyn std::error::Error>> {
        
        // First try exact email match
        if let Some(email_addr) = email {
            if let Some(person_id) = self.find_by_email(email_addr).await? {
                self.add_platform_identity(&person_id, platform, platform_user_id, display_name, email).await?;
                return Ok(person_id);
            }
        }

        // Try fuzzy matching on display names
        let candidates = self.find_similar_people(display_name, platform).await?;
        
        for candidate in candidates {
            let confidence = self.calculate_identity_confidence(&candidate, display_name, email).await?;
            if confidence >= self.confidence_threshold {
                self.add_platform_identity(&candidate.id, platform, platform_user_id, display_name, email).await?;
                return Ok(candidate.id);
            }
        }

        // No match found, create new person
        self.create_new_person(email, display_name, platform, platform_user_id).await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut person_id = None;
        
        with_connection("find_by_email", |conn| {
            let mut stmt = conn.prepare("SELECT id FROM people WHERE email = ?1").expect("Failed to prepare query");
            let mut rows = stmt.query_map([email], |row| {
                Ok(row.get::<_, String>(0)?)
            }).expect("Failed to execute query");
            
            if let Some(row) = rows.next() {
                person_id = Some(row.expect("Failed to get row data"));
            }
        });
        
        Ok(person_id)
    }

    async fn find_similar_people(&self, display_name: &str, platform: &str) -> Result<Vec<Person>, Box<dyn std::error::Error>> {
        let mut candidates = Vec::new();
        
        // Simple similarity matching - could be enhanced with fuzzy string matching
        let name_patterns = vec![
            display_name.to_lowercase(),
            display_name.replace(".", "").to_lowercase(),
            display_name.replace(" ", "").to_lowercase(),
        ];
        
        with_connection("find_similar_people", |conn| {
            for pattern in name_patterns {
                let query = format!("%{}%", pattern);
                let mut stmt = conn.prepare("
                    SELECT id, email, display_names, platform_identities 
                    FROM people 
                    WHERE display_names LIKE ?1
                    LIMIT 10
                ")?;
                
                let rows = stmt.query_map([&query], |row| {
                    let display_names_json: String = row.get(2)?;
                    let platform_identities_json: String = row.get(3)?;
                    
                    Ok(Person {
                        id: row.get(0)?,
                        email: row.get(1)?,
                        display_names: serde_json::from_str(&display_names_json).unwrap_or_default(),
                        platform_identities: serde_json::from_str(&platform_identities_json).unwrap_or_default(),
                        expertise_areas: Vec::new(), // Don't need full data for matching
                        activity_metrics: ActivityMetrics::default(),
                        collaboration_network: Vec::new(),
                        first_seen: Utc::now(),
                        last_active: Utc::now(),
                    })
                })?;
                
                for row in rows {
                    if let Ok(person) = row {
                        candidates.push(person);
                    }
                }
            }
            Ok(())
        })?;
        
        Ok(candidates)
    }

    async fn calculate_identity_confidence(&self, candidate: &Person, display_name: &str, email: Option<&str>) -> Result<f64, Box<dyn std::error::Error>> {
        let mut confidence = 0.0;
        
        // Email match is strongest signal
        if let Some(email_addr) = email {
            if candidate.email.to_lowercase() == email_addr.to_lowercase() {
                confidence += 0.9;
            }
        }
        
        // Display name similarity
        let name_lower = display_name.to_lowercase();
        for existing_name in &candidate.display_names {
            let existing_lower = existing_name.to_lowercase();
            if existing_lower == name_lower {
                confidence += 0.7;
            } else if existing_lower.contains(&name_lower) || name_lower.contains(&existing_lower) {
                confidence += 0.4;
            }
        }
        
        // Platform diversity (same person likely to be on multiple platforms)
        if candidate.platform_identities.len() > 1 {
            confidence += 0.1;
        }
        
        Ok(confidence.min(1.0))
    }

    async fn add_platform_identity(&self, person_id: &str, platform: &str, platform_user_id: &str, display_name: &str, email: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        
        with_connection("add_platform_identity", |conn| {
            // Get current platform identities
            let mut stmt = conn.prepare("SELECT platform_identities FROM people WHERE id = ?1")?;
            let mut current_identities: HashMap<String, PlatformIdentity> = HashMap::new();
            
            let mut rows = stmt.query_map([person_id], |row| {
                let json_str: String = row.get(0)?;
                Ok(json_str)
            })?;
            
            if let Some(row) = rows.next() {
                let json_str = row?;
                current_identities = serde_json::from_str(&json_str).unwrap_or_default();
            }
            
            // Add/update platform identity
            let platform_identity = PlatformIdentity {
                platform: platform.to_string(),
                platform_user_id: platform_user_id.to_string(),
                display_name: display_name.to_string(),
                email: email.map(|e| e.to_string()),
                profile_url: None, // Could be populated later
                verified: true,
                first_seen_on_platform: Utc::now(),
                last_active_on_platform: Utc::now(),
            };
            
            current_identities.insert(platform.to_string(), platform_identity);
            
            // Update the person record
            let identities_json = serde_json::to_string(&current_identities)?;
            let mut update_stmt = conn.prepare("
                UPDATE people 
                SET platform_identities = ?1, 
                    last_active = ?2,
                    updated_at = ?2
                WHERE id = ?3
            ")?;
            
            update_stmt.execute([&identities_json, &Utc::now().to_rfc3339(), person_id])?;
            
            Ok(())
        })?;
        
        Ok(())
    }

    async fn create_new_person(&self, email: Option<&str>, display_name: &str, platform: &str, platform_user_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let person_id = uuid::Uuid::new_v4().to_string();
        let email_addr = email.unwrap_or(&format!("unknown+{}@{}.local", platform_user_id, platform));
        
        let mut platform_identities = HashMap::new();
        platform_identities.insert(platform.to_string(), PlatformIdentity {
            platform: platform.to_string(),
            platform_user_id: platform_user_id.to_string(),
            display_name: display_name.to_string(),
            email: email.map(|e| e.to_string()),
            profile_url: None,
            verified: true,
            first_seen_on_platform: Utc::now(),
            last_active_on_platform: Utc::now(),
        });
        
        let person = Person {
            id: person_id.clone(),
            email: email_addr.to_string(),
            display_names: vec![display_name.to_string()],
            platform_identities,
            expertise_areas: Vec::new(),
            activity_metrics: ActivityMetrics::default(),
            collaboration_network: Vec::new(),
            first_seen: Utc::now(),
            last_active: Utc::now(),
        };
        
        with_connection("create_new_person", |conn| {
            let mut stmt = conn.prepare("
                INSERT INTO people (
                    id, email, display_names, platform_identities, 
                    expertise_areas, activity_metrics, collaboration_network,
                    first_seen, last_active
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ").expect("Failed to prepare person insert statement");
            
            stmt.execute([
                &person.id,
                &person.email,
                &serde_json::to_string(&person.display_names).expect("Failed to serialize display_names"),
                &serde_json::to_string(&person.platform_identities).expect("Failed to serialize platform_identities"),
                &serde_json::to_string(&person.expertise_areas).expect("Failed to serialize expertise_areas"),
                &serde_json::to_string(&person.activity_metrics).expect("Failed to serialize activity_metrics"),
                &serde_json::to_string(&person.collaboration_network).expect("Failed to serialize collaboration_network"),
                &person.first_seen.to_rfc3339(),
                &person.last_active.to_rfc3339(),
            ]).expect("Failed to insert person");
        });
        
        log_success(&format!("Created new person: {} ({})", display_name, person_id));
        Ok(person_id)
    }
}

impl Default for ActivityMetrics {
    fn default() -> Self {
        Self {
            response_time_avg_hours: 24.0,
            collaboration_frequency: 0.0,
            knowledge_sharing_score: 0.0,
            problem_resolution_rate: 0.0,
            cross_platform_activity: 1,
            mentorship_indicators: 0,
            total_contributions: 0,
            last_calculated: Utc::now(),
        }
    }
}

// ================================
// TABLE CREATION
// ================================

pub async fn initialize_people_tables() {
    log_step("👥", "Initializing people and interactions tables...");
    
    with_connection("create_people_tables", |conn| {
        // Create tables
        conn.execute_batch(CREATE_PEOPLE_TABLE).expect("Failed to create people table");
        conn.execute_batch(CREATE_INTERACTIONS_TABLE).expect("Failed to create interactions table");
        conn.execute_batch(CREATE_PEOPLE_INDEXES).expect("Failed to create people indexes");
    });
    
    log_success("People graph tables ready.");
}