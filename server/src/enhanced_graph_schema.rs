// Enhanced Graph-Optimized Schema for Universal Knowledge Platform
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// ================================
// PEOPLE & SOCIAL GRAPH SCHEMA
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: String,                          // unique identifier
    pub email: String,                       // primary key across platforms
    pub display_names: Vec<String>,          // ["Sarah Smith", "ssmith", "sarah.smith"]
    pub platform_handles: HashMap<String, String>, // {"slack": "U123", "jira": "ssmith"}
    pub expertise_areas: Vec<ExpertiseScore>,
    pub activity_patterns: ActivityPattern,
    pub team_memberships: Vec<String>,       // which teams/projects
    pub first_seen: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpertiseScore {
    pub concept: String,                     // "authentication", "redis", "docker"
    pub authority_score: f64,                // 0.0 to 1.0
    pub evidence_count: u32,                 // how many contributions
    pub recent_activity: DateTime<Utc>,      // still active in this area?
    pub peer_validation: f64,                // how often others reference their work
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityPattern {
    pub response_time_avg_hours: f64,        // how quickly they respond
    pub collaboration_score: f64,            // how often they work with others
    pub knowledge_sharing_score: f64,        // how often they document/explain
    pub problem_solving_score: f64,          // how often their suggestions work
}

// ================================
// ENHANCED INTERACTION TRACKING
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interaction {
    pub id: String,
    pub interaction_type: InteractionType,
    pub source_person_id: String,           // who initiated
    pub target_person_id: Option<String>,   // who they interacted with (if applicable)
    pub content_id: String,                 // what content this relates to
    pub timestamp: DateTime<Utc>,
    pub context: InteractionContext,
    pub sentiment: Option<f64>,             // positive/negative interaction
    pub impact_score: f64,                  // how valuable was this interaction
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InteractionType {
    Created,                                // authored original content
    Commented,                              // added comment/reply
    Reacted {                               // emoji reactions
        reaction_type: String,              // "👍", "❤️", "🚀"
        reaction_count: u32,
    },
    Mentioned {                             // @mentioned someone
        mentioned_person_ids: Vec<String>,
    },
    Referenced,                             // linked to other content
    Resolved,                               // marked as solution
    Implemented,                            // built the solution
    Reviewed,                               // reviewed/approved
    Questioned,                             // asked follow-up questions
    Expanded,                               // built upon the idea
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InteractionContext {
    pub platform: String,                   // "jira", "slack", "google"
    pub thread_id: Option<String>,          // slack thread, comment chain
    pub urgency_level: Option<String>,      // "high", "medium", "low"
    pub expertise_areas: Vec<String>,       // what concepts this interaction relates to
}

// ================================
// ENHANCED CONTENT WITH FULL PARTICIPATION
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnhancedContent {
    pub id: String,
    pub content_type: ContentType,
    pub source_platform: String,
    pub title: String,
    pub body_text: String,
    
    // ENHANCED PEOPLE TRACKING
    pub primary_author: String,             // main creator
    pub contributors: Vec<Contributor>,     // everyone who participated
    pub mention_network: Vec<Mention>,      // who was @mentioned
    pub audience: Vec<String>,              // who had access/could see it
    
    // INTERACTION HISTORY
    pub interaction_timeline: Vec<Interaction>,
    pub collaboration_patterns: CollaborationPattern,
    
    // KNOWLEDGE GRAPH DATA
    pub concepts: Vec<ConceptMention>,      // what topics with confidence scores
    pub technologies: Vec<TechnologyUsage>,
    pub solution_patterns: Vec<SolutionPattern>,
    
    // TEMPORAL DATA
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub peak_activity_period: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contributor {
    pub person_id: String,
    pub contribution_type: ContributionType,
    pub timestamp: DateTime<Utc>,
    pub contribution_weight: f64,           // how significant was their contribution
    pub interaction_count: u32,             // how many times they interacted
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContributionType {
    OriginalAuthor,
    CoAuthor,
    Commenter,
    Reviewer,
    Implementer,
    QuestionAsker,
    SolutionProvider,
    Reactor,                                // provided emoji reactions
    Amplifier,                              // shared/referenced elsewhere
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mention {
    pub mentioned_person_id: String,
    pub mention_context: String,            // surrounding text
    pub mention_type: MentionType,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MentionType {
    DirectMention,                          // @sarah
    ExpertiseReference,                     // "sarah knows about auth"
    HandoffMention,                         // "sarah can you take this?"
    CreditMention,                          // "thanks to sarah's solution"
    QuestionMention,                        // "sarah what do you think?"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollaborationPattern {
    pub frequent_collaborators: Vec<(String, f64)>, // person_id, collaboration_score
    pub collaboration_network_size: u32,
    pub cross_team_collaboration: bool,
    pub knowledge_transfer_events: u32,     // how often this content taught others
}

// ================================
// CONCEPT & SOLUTION TRACKING
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConceptMention {
    pub concept: String,
    pub confidence_score: f64,              // how sure are we this concept applies
    pub mention_context: String,            // surrounding text
    pub mentioned_by: String,               // who introduced this concept
    pub validation_count: u32,              // how many others agreed/built on it
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SolutionPattern {
    pub pattern_id: String,
    pub pattern_description: String,
    pub original_author: String,            // who first used this pattern
    pub adaptations: Vec<SolutionAdaptation>, // how others modified it
    pub success_rate: f64,                  // how often this pattern works
    pub related_patterns: Vec<String>,      // similar solution patterns
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SolutionAdaptation {
    pub adapter_person_id: String,
    pub adaptation_context: String,
    pub changes_made: String,
    pub outcome: AdaptationOutcome,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AdaptationOutcome {
    Successful,
    Failed,
    PartialSuccess,
    ImprovedOriginal,
}

// ================================
// CROSS-PLATFORM RELATIONSHIPS
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnhancedRelationship {
    pub id: String,
    pub source_content_id: String,
    pub target_content_id: String,
    pub relationship_type: RelationshipType,
    pub strength: f64,                      // 0.0 to 1.0
    pub created_by: String,                 // who made this connection
    pub discovery_method: DiscoveryMethod,
    pub temporal_context: TemporalRelationshipContext,
    pub validation_score: f64,              // how many people found this connection useful
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RelationshipType {
    // Content relationships
    References,
    BuildsUpon,
    Contradicts,
    Updates,
    Implements,
    Documents,
    Questions,
    Answers,
    
    // People relationships  
    Collaborates,
    MentorStudent,
    ExpertNovice,
    PeerReview,
    KnowledgeTransfer,
    
    // Solution relationships
    SameProblem,
    SimilarSolution,
    AlternativeApproach,
    PrerequisiteFor,
    CausedBy,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DiscoveryMethod {
    ExplicitLink,                           // direct URL reference
    MentionExtraction,                      // found via @mention
    ConceptualSimilarity,                   // AI/ML discovered
    PersonConnection,                       // same person involved
    TemporalProximity,                      // created around same time
    UserTagged,                             // manually connected by user
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemporalRelationshipContext {
    pub time_gap: i64,                      // seconds between related content
    pub sequence_order: Option<u32>,        // if there's a natural sequence
    pub temporal_relevance: f64,            // how time-sensitive is this relationship
}

// ================================
// DATABASE TABLES FOR GRAPH STORAGE
// ================================

pub const CREATE_PEOPLE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS people (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    display_names TEXT, -- JSON array
    platform_handles TEXT, -- JSON object
    expertise_areas TEXT, -- JSON array of ExpertiseScore
    activity_patterns TEXT, -- JSON object
    team_memberships TEXT, -- JSON array
    first_seen TIMESTAMP NOT NULL,
    last_active TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
"#;

pub const CREATE_INTERACTIONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS interactions (
    id TEXT PRIMARY KEY,
    interaction_type TEXT NOT NULL,
    source_person_id TEXT NOT NULL,
    target_person_id TEXT,
    content_id TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    context TEXT, -- JSON InteractionContext
    sentiment REAL,
    impact_score REAL NOT NULL,
    metadata TEXT, -- JSON for type-specific data
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (source_person_id) REFERENCES people(id),
    FOREIGN KEY (target_person_id) REFERENCES people(id),
    FOREIGN KEY (content_id) REFERENCES extracted_content(id)
);
"#;

pub const CREATE_ENHANCED_CONTENT_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS enhanced_content (
    id TEXT PRIMARY KEY,
    content_type TEXT NOT NULL,
    source_platform TEXT NOT NULL,
    title TEXT NOT NULL,
    body_text TEXT NOT NULL,
    primary_author TEXT NOT NULL,
    contributors TEXT, -- JSON array of Contributor
    mention_network TEXT, -- JSON array of Mention
    audience TEXT, -- JSON array of person IDs
    interaction_timeline TEXT, -- JSON array of Interaction IDs
    collaboration_patterns TEXT, -- JSON CollaborationPattern
    concepts TEXT, -- JSON array of ConceptMention
    technologies TEXT, -- JSON array of TechnologyUsage
    solution_patterns TEXT, -- JSON array of SolutionPattern
    created_at TIMESTAMP NOT NULL,
    last_modified TIMESTAMP NOT NULL,
    peak_activity_period TEXT, -- JSON tuple of timestamps
    
    FOREIGN KEY (primary_author) REFERENCES people(id)
);
"#;

pub const CREATE_ENHANCED_RELATIONSHIPS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS enhanced_relationships (
    id TEXT PRIMARY KEY,
    source_content_id TEXT NOT NULL,
    target_content_id TEXT NOT NULL,
    relationship_type TEXT NOT NULL,
    strength REAL NOT NULL,
    created_by TEXT NOT NULL,
    discovery_method TEXT NOT NULL,
    temporal_context TEXT, -- JSON TemporalRelationshipContext
    validation_score REAL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (source_content_id) REFERENCES enhanced_content(id),
    FOREIGN KEY (target_content_id) REFERENCES enhanced_content(id),
    FOREIGN KEY (created_by) REFERENCES people(id),
    
    UNIQUE(source_content_id, target_content_id, relationship_type)
);
"#;

// Index for graph queries
pub const CREATE_GRAPH_INDEXES: &str = r#"
CREATE INDEX IF NOT EXISTS idx_interactions_person_content ON interactions(source_person_id, content_id);
CREATE INDEX IF NOT EXISTS idx_interactions_timestamp ON interactions(timestamp);
CREATE INDEX IF NOT EXISTS idx_relationships_source ON enhanced_relationships(source_content_id);
CREATE INDEX IF NOT EXISTS idx_relationships_target ON enhanced_relationships(target_content_id);
CREATE INDEX IF NOT EXISTS idx_content_author ON enhanced_content(primary_author);
CREATE INDEX IF NOT EXISTS idx_content_concepts ON enhanced_content(concepts);
CREATE INDEX IF NOT EXISTS idx_people_expertise ON people(expertise_areas);
"#;