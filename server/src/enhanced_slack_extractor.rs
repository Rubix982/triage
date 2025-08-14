use crate::slack_client::SlackApiClient;
use crate::people_graph::IdentityResolver;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// ================================
// ENHANCED SLACK THREAD DYNAMICS
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackThreadDynamics {
    pub thread_ts: String,
    pub channel_id: String,
    pub channel_name: String,
    pub thread_title: Option<String>, // derived from first message
    pub participants: Vec<SlackParticipant>,
    pub message_flow: Vec<EnhancedSlackMessage>,
    pub reactions_analysis: ReactionsAnalysis,
    pub shared_content: Vec<SharedContent>,
    pub topic_evolution: Vec<TopicShift>,
    pub collaboration_patterns: Vec<SlackCollaborationPattern>,
    pub problem_resolution: Option<ProblemResolutionFlow>,
    pub knowledge_transfer_events: Vec<SlackKnowledgeTransfer>,
    pub thread_metadata: ThreadMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackParticipant {
    pub person_id: String,
    pub slack_user_id: String,
    pub display_name: String,
    pub roles: Vec<SlackParticipantRole>,
    pub participation_metrics: ParticipationMetrics,
    pub influence_indicators: InfluenceIndicators,
    pub expertise_demonstrated: Vec<String>,
    pub collaboration_style: SlackCollaborationStyle,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SlackParticipantRole {
    ThreadStarter,    // initiated the conversation
    ProblemReporter,  // described the problem
    SolutionProvider, // provided solutions
    Implementer,      // worked on implementing solutions
    Reviewer,         // reviewed/tested solutions
    Supporter,        // provided encouragement/reactions
    Questioner,       // asked clarifying questions
    Documenter,       // summarized or documented outcomes
    Connector,        // brought in other people (@mentions)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParticipationMetrics {
    pub message_count: u32,
    pub reaction_given_count: u32,
    pub reaction_received_count: u32,
    pub mention_count: u32,
    pub file_share_count: u32,
    pub link_share_count: u32,
    pub first_message_time: DateTime<Utc>,
    pub last_message_time: DateTime<Utc>,
    pub average_response_time_minutes: f64,
    pub message_length_avg: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfluenceIndicators {
    pub solution_acceptance_rate: f64,    // how often their solutions are accepted
    pub reaction_magnetism: f64,          // how often their messages get reactions
    pub mention_frequency: f64,           // how often they get mentioned
    pub thread_resolution_contribution: f64, // how much they contribute to resolving issues
    pub knowledge_authority_score: f64,   // calculated expertise authority
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SlackCollaborationStyle {
    ProblemSolver,    // jumps in to solve technical issues
    Facilitator,      // helps organize discussion
    Supporter,        // provides encouragement and reactions
    Questioner,       // asks good clarifying questions
    Documenter,       // summarizes and documents
    Connector,        // brings in the right people
    Observer,         // participates minimally but stays informed
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnhancedSlackMessage {
    pub message_id: String,
    pub user: SlackUser,
    pub timestamp: DateTime<Utc>,
    pub text: String,
    pub message_type: SlackMessageType,
    pub reactions: Vec<MessageReaction>,
    pub mentions: Vec<UserMention>,
    pub shared_content: Vec<MessageContent>,
    pub thread_context: ThreadContext,
    pub sentiment_indicators: SentimentIndicators,
    pub knowledge_indicators: MessageKnowledgeIndicators,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SlackMessageType {
    ThreadStart,      // first message in thread
    Response,         // response to previous messages
    Question,         // asking for help/clarification
    Solution,         // providing solution/answer
    Update,           // status update or progress report
    Acknowledgment,   // thanking, confirming, agreeing
    Clarification,    // asking for or providing clarification
    Summary,          // summarizing discussion
    Escalation,       // bringing in more help
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageReaction {
    pub emoji: String,
    pub users: Vec<SlackUser>,
    pub reaction_type: ReactionType,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReactionType {
    Approval,       // thumbs up, check mark
    Appreciation,   // heart, clap, fire
    Agreement,      // +1, yes
    Attention,      // eyes, point up
    Celebration,    // party, tada
    Concern,        // thinking, warning
    Custom,         // custom emoji
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserMention {
    pub mentioned_user: SlackUser,
    pub mention_context: String,
    pub mention_purpose: MentionPurpose,
    pub response_received: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MentionPurpose {
    AskingForHelp,     // @sarah can you help with this?
    RequestingReview,  // @mike can you review this?
    FYI,              // @team just so you know
    BringingExpertise, // @expert you know about this
    Escalation,       // @manager we need help
    Attribution,      // thanks @sarah for this
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageContent {
    pub content_type: ContentType,
    pub content_data: ContentData,
    pub share_context: String,
    pub discussion_generated: u32, // how many replies this content generated
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContentType {
    FileUpload,
    ImageShare,
    CodeSnippet,
    ExternalLink,
    SlackLink,     // link to other slack messages/threads
    GoogleDocLink,
    JiraLink,
    GitHubLink,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContentData {
    File {
        filename: String,
        file_type: String,
        size: u64,
    },
    Link {
        url: String,
        title: Option<String>,
        description: Option<String>,
    },
    Code {
        language: Option<String>,
        code_content: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadContext {
    pub position_in_thread: u32,
    pub responds_to_messages: Vec<String>, // message IDs this responds to
    pub generates_responses: Vec<String>,  // message IDs that respond to this
    pub conversation_branch: Option<String>, // if thread splits into sub-conversations
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SentimentIndicators {
    pub urgency_level: UrgencyLevel,
    pub emotional_tone: EmotionalTone,
    pub problem_severity: ProblemSeverity,
    pub confidence_level: ConfidenceLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UrgencyLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EmotionalTone {
    Frustrated,
    Confused,
    Confident,
    Helpful,
    Grateful,
    Concerned,
    Excited,
    Neutral,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProblemSeverity {
    Minor,      // small issue or question
    Moderate,   // affects some functionality
    Major,      // significant impact
    Critical,   // system down, urgent
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConfidenceLevel {
    Uncertain,  // asking questions, unsure
    Exploring,  // trying different approaches
    Confident,  // know what they're doing
    Authoritative, // definitive answers/solutions
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageKnowledgeIndicators {
    pub contains_solution: bool,
    pub contains_explanation: bool,
    pub contains_code_example: bool,
    pub contains_documentation_reference: bool,
    pub knowledge_level_demonstrated: KnowledgeLevel,
    pub teaching_indicators: Vec<TeachingIndicator>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KnowledgeLevel {
    Beginner,    // basic questions, learning
    Intermediate, // some knowledge, mixed asking/helping
    Advanced,    // mostly helping others
    Expert,      // definitive solutions, teaching
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TeachingIndicator {
    StepByStepExplanation,
    CodeExample,
    DocumentationReference,
    ConceptExplanation,
    BestPracticeSharing,
    TroubleshootingGuidance,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactionsAnalysis {
    pub total_reactions: u32,
    pub reaction_diversity: u32, // how many different emoji types
    pub reaction_distribution: HashMap<String, u32>, // emoji -> count
    pub most_reacted_message: Option<String>,
    pub reaction_patterns: Vec<ReactionPattern>,
    pub social_validation_score: f64, // how much social approval the thread generated
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactionPattern {
    pub pattern_type: ReactionPatternType,
    pub frequency: u32,
    pub participants: Vec<String>, // person IDs
    pub significance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReactionPatternType {
    SolutionValidation,  // solutions get thumbs up
    AppreciationChain,   // lots of thank you reactions
    ConsensusBuilding,   // everyone agrees with +1
    CelebrationMoment,   // problem solved, celebration reactions
    AttentionGrabbing,   // eyes/point reactions to draw attention
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SharedContent {
    pub shared_by: String, // person ID
    pub content: MessageContent,
    pub sharing_context: SharingContext,
    pub impact_metrics: ContentImpactMetrics,
    pub cross_platform_connections: Vec<CrossPlatformConnection>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SharingContext {
    pub reason_for_sharing: SharingReason,
    pub target_audience: Vec<String>, // person IDs if specific people targeted
    pub sharing_timing: SharingTiming,
    pub follow_up_discussion: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SharingReason {
    ProvidingSolution,    // sharing docs/links that solve the problem
    GivingContext,       // providing background information
    ShowingExample,      // demonstrating how something works
    RequestingFeedback,  // asking others to review shared content
    Documentation,       // sharing for future reference
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SharingTiming {
    EarlyInThread,      // shared near beginning of discussion
    MidConversation,    // shared during active discussion
    AsResolution,       // shared as final solution
    FollowUp,          // shared after main discussion ended
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentImpactMetrics {
    pub click_count: Option<u32>,        // if available from Slack analytics
    pub replies_generated: u32,          // how many replies this content generated
    pub reactions_received: u32,         // reactions to the shared content
    pub referenced_later: u32,           // how often this content gets referenced
    pub problem_resolution_contribution: f64, // how much this helped solve the problem
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrossPlatformConnection {
    pub platform: String,               // "google", "jira", "github"
    pub content_id: String,             // ID of the connected content
    pub connection_type: ConnectionType,
    pub discovered_method: String,      // how the connection was found
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConnectionType {
    DirectReference,     // explicit link shared
    TopicSimilarity,    // similar topics discussed
    SamePeople,         // same people involved
    SolutionContinuation, // solution continues in other platform
    ProblemSource,      // problem originated from other platform
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicShift {
    pub shift_point: u32,               // message number where topic shifted
    pub previous_topic: String,
    pub new_topic: String,
    pub shift_trigger: TopicShiftTrigger,
    pub participants_involved: Vec<String>, // person IDs
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TopicShiftTrigger {
    NewQuestionAsked,    // someone asked a different question
    SolutionFound,       // original problem solved, moved to implementation
    EscalationOccurred, // problem escalated to different level
    ExpertJoined,       // expert joined and redirected conversation
    RelatedIssueFound,  // discovered related but different issue
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackCollaborationPattern {
    pub pattern_type: SlackCollaborationPatternType,
    pub participants: Vec<String>, // person IDs
    pub frequency: u32,
    pub effectiveness_score: f64,
    pub typical_duration_minutes: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SlackCollaborationPatternType {
    QuestionAnswerCycle, // someone asks, others answer
    CollaborativeDebugging, // multiple people troubleshoot together
    KnowledgeTransfer,   // expert teaches others
    PeerReview,         // collaborative review of solutions
    Brainstorming,      // generating ideas together
    StatusUpdates,      // regular progress sharing
    Escalation,         // systematic escalation through levels
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProblemResolutionFlow {
    pub problem_statement: String,
    pub resolution_steps: Vec<ResolutionStep>,
    pub final_resolution: Option<String>,
    pub resolution_time_minutes: f64,
    pub resolution_success: bool,
    pub contributors: Vec<ResolutionContributor>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResolutionStep {
    pub step_number: u32,
    pub actor: String, // person ID
    pub action: ResolutionAction,
    pub message_id: String,
    pub timestamp: DateTime<Utc>,
    pub outcome: Option<String>,
    pub validation: Option<ValidationInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResolutionAction {
    ProblemIdentification,
    InformationGathering,
    SolutionProposal,
    SolutionImplementation,
    SolutionTesting,
    SolutionValidation,
    DocumentationCreation,
    KnowledgeSharing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationInfo {
    pub validated_by: Vec<String>, // person IDs
    pub validation_method: ValidationMethod,
    pub validation_outcome: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ValidationMethod {
    PeerConfirmation,  // other team members confirmed
    Testing,          // solution was tested
    Implementation,   // solution was implemented successfully
    Documentation,    // solution was documented
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResolutionContributor {
    pub person_id: String,
    pub contribution_type: Vec<ContributionType>,
    pub contribution_weight: f64,
    pub expertise_demonstrated: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContributionType {
    ProblemReporting,
    InformationProviding,
    SolutionProposing,
    SolutionImplementing,
    SolutionValidating,
    ProcessFacilitating,
    KnowledgeSharing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackKnowledgeTransfer {
    pub teacher: String, // person ID
    pub learners: Vec<String>, // person IDs
    pub knowledge_topic: String,
    pub transfer_evidence: Vec<TransferEvidence>,
    pub transfer_effectiveness: f64,
    pub follow_up_actions: Vec<FollowUpAction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferEvidence {
    pub evidence_type: TransferEvidenceType,
    pub message_id: String,
    pub confidence_score: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferEvidenceType {
    DetailedExplanation,
    CodeExample,
    StepByStepGuide,
    ConceptClarification,
    ResourceSharing,
    QuestionAnswering,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FollowUpAction {
    pub action_type: FollowUpActionType,
    pub actor: String, // person ID
    pub completed: bool,
    pub evidence_message_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FollowUpActionType {
    ImplementationAttempt,
    DocumentationCreation,
    KnowledgeApplication,
    FurtherQuestioning,
    TeachingOthers,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadMetadata {
    pub total_messages: u32,
    pub unique_participants: u32,
    pub thread_duration_hours: f64,
    pub peak_activity_period: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub thread_category: ThreadCategory,
    pub business_impact: BusinessImpact,
    pub knowledge_value: KnowledgeValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ThreadCategory {
    TechnicalSupport,
    ProblemSolving,
    KnowledgeSharing,
    ProjectCoordination,
    DecisionMaking,
    StatusUpdate,
    Brainstorming,
    SocialInteraction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BusinessImpact {
    Low,      // routine question or update
    Medium,   // affects team productivity
    High,     // affects project timeline or quality
    Critical, // affects system availability or customer experience
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KnowledgeValue {
    Low,      // limited reusability or learning value
    Medium,   // some concepts that could help others
    High,     // significant learning or best practices shared
    Archive,  // valuable enough to archive for future reference
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackUser {
    pub user_id: String, // Slack user ID
    pub display_name: String,
    pub real_name: Option<String>,
    pub email: Option<String>,
    pub profile: Option<SlackUserProfile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackUserProfile {
    pub title: Option<String>,
    pub team: Option<String>,
    pub avatar_url: Option<String>,
}

// ================================
// ENHANCED SLACK EXTRACTOR
// ================================

pub struct EnhancedSlackExtractor {
    client: SlackApiClient,
    identity_resolver: IdentityResolver,
}

impl EnhancedSlackExtractor {
    pub fn new(client: SlackApiClient) -> Self {
        Self {
            client,
            identity_resolver: IdentityResolver::new(),
        }
    }

    pub async fn extract_thread_dynamics(&mut self, channel_id: &str, thread_ts: &str) -> Result<SlackThreadDynamics, Box<dyn std::error::Error>> {
        crate::utils::log_step("💬", &format!("Extracting thread dynamics for Slack thread {}", thread_ts));

        // Get all messages in the thread
        let thread_messages = self.client.get_thread_messages(channel_id, thread_ts).await?;
        
        // Get channel info
        let channel_info = self.client.get_channel_info(channel_id).await?;
        
        // Extract enhanced message data
        let message_flow = self.process_thread_messages(&thread_messages).await?;
        
        // Build participant analysis
        let participants = self.analyze_participants(&message_flow).await?;
        
        // Analyze reactions across all messages
        let reactions_analysis = self.analyze_reactions(&message_flow).await?;
        
        // Extract shared content
        let shared_content = self.extract_shared_content(&message_flow).await?;
        
        // Identify topic shifts
        let topic_evolution = self.identify_topic_shifts(&message_flow).await?;
        
        // Analyze collaboration patterns
        let collaboration_patterns = self.analyze_collaboration_patterns(&message_flow, &participants).await?;
        
        // Detect problem resolution flow
        let problem_resolution = self.analyze_problem_resolution(&message_flow).await?;
        
        // Identify knowledge transfer events
        let knowledge_transfer_events = self.identify_knowledge_transfer(&message_flow, &participants).await?;
        
        // Calculate thread metadata
        let thread_metadata = self.calculate_thread_metadata(&message_flow, &participants).await?;

        let thread_dynamics = SlackThreadDynamics {
            thread_ts: thread_ts.to_string(),
            channel_id: channel_id.to_string(),
            channel_name: channel_info.name.unwrap_or_default(),
            thread_title: self.derive_thread_title(&message_flow),
            participants,
            message_flow,
            reactions_analysis,
            shared_content,
            topic_evolution,
            collaboration_patterns,
            problem_resolution,
            knowledge_transfer_events,
            thread_metadata,
        };

        crate::utils::log_success(&format!("Thread dynamics analysis complete for {}", thread_ts));
        Ok(thread_dynamics)
    }

    async fn process_thread_messages(&self, messages: &[crate::slack_client::SlackMessage]) -> Result<Vec<EnhancedSlackMessage>, Box<dyn std::error::Error>> {
        let mut enhanced_messages = Vec::new();
        
        for (i, message) in messages.iter().enumerate() {
            let enhanced_message = self.enhance_message(message, i as u32).await?;
            enhanced_messages.push(enhanced_message);
        }
        
        Ok(enhanced_messages)
    }

    async fn enhance_message(&self, message: &crate::slack_client::SlackMessage, position: u32) -> Result<EnhancedSlackMessage, Box<dyn std::error::Error>> {
        // Convert basic SlackMessage to EnhancedSlackMessage
        // This is a placeholder - you'd need to implement full conversion
        
        let user = SlackUser {
            user_id: message.user.clone().unwrap_or_default(),
            display_name: message.username.clone().unwrap_or_default(),
            real_name: None,
            email: None,
            profile: None,
        };

        // Analyze message content for type classification
        let message_type = self.classify_message_type(&message.text, position).await;
        
        // Extract mentions from message text
        let mentions = self.extract_mentions_from_slack_message(&message.text).await?;
        
        // Extract shared content (files, links, etc.)
        let shared_content = self.extract_message_content(message).await?;
        
        // Analyze sentiment and knowledge indicators
        let sentiment_indicators = self.analyze_message_sentiment(&message.text).await;
        let knowledge_indicators = self.analyze_message_knowledge(&message.text).await;
        
        Ok(EnhancedSlackMessage {
            message_id: message.ts.clone(),
            user,
            timestamp: message.timestamp,
            text: message.text.clone(),
            message_type,
            reactions: Vec::new(), // Would be populated from Slack API reactions
            mentions,
            shared_content,
            thread_context: ThreadContext {
                position_in_thread: position,
                responds_to_messages: Vec::new(), // Would analyze conversation flow
                generates_responses: Vec::new(),
                conversation_branch: None,
            },
            sentiment_indicators,
            knowledge_indicators,
        })
    }

    async fn classify_message_type(&self, text: &str, position: u32) -> SlackMessageType {
        if position == 0 {
            SlackMessageType::ThreadStart
        } else if text.contains("?") && !text.to_lowercase().contains("thanks") {
            SlackMessageType::Question
        } else if text.to_lowercase().contains("solution") || text.to_lowercase().contains("try this") {
            SlackMessageType::Solution
        } else if text.to_lowercase().contains("thanks") || text.to_lowercase().contains("got it") {
            SlackMessageType::Acknowledgment
        } else {
            SlackMessageType::Response
        }
    }

    async fn extract_mentions_from_slack_message(&self, text: &str) -> Result<Vec<UserMention>, Box<dyn std::error::Error>> {
        // Extract @mentions and <!channel> mentions from Slack message format
        // Slack mentions are in format <@U123456|username>
        let mut mentions = Vec::new();
        
        // Simple regex for Slack mentions - would be more sophisticated in practice
        let mention_regex = regex::Regex::new(r"<@(U\w+)\|?([^>]*)>").unwrap();
        
        for capture in mention_regex.captures_iter(text) {
            let user_id = capture.get(1).unwrap().as_str();
            let username = capture.get(2).map(|m| m.as_str()).unwrap_or("");
            
            let mentioned_user = SlackUser {
                user_id: user_id.to_string(),
                display_name: username.to_string(),
                real_name: None,
                email: None,
                profile: None,
            };
            
            let mention_purpose = self.determine_mention_purpose(text, user_id).await;
            
            mentions.push(UserMention {
                mentioned_user,
                mention_context: self.extract_mention_context(text, user_id),
                mention_purpose,
                response_received: false, // Would check if mentioned user responded
            });
        }
        
        Ok(mentions)
    }

    async fn determine_mention_purpose(&self, text: &str, _user_id: &str) -> MentionPurpose {
        let text_lower = text.to_lowercase();
        
        if text_lower.contains("can you help") || text_lower.contains("any ideas") {
            MentionPurpose::AskingForHelp
        } else if text_lower.contains("review") || text_lower.contains("look at") {
            MentionPurpose::RequestingReview
        } else if text_lower.contains("fyi") || text_lower.contains("just so you know") {
            MentionPurpose::FYI
        } else if text_lower.contains("thanks") || text_lower.contains("credit") {
            MentionPurpose::Attribution
        } else {
            MentionPurpose::BringingExpertise
        }
    }

    fn extract_mention_context(&self, text: &str, user_id: &str) -> String {
        // Extract context around the mention
        if let Some(pos) = text.find(user_id) {
            let start = pos.saturating_sub(50);
            let end = std::cmp::min(text.len(), pos + user_id.len() + 50);
            text[start..end].to_string()
        } else {
            text.to_string()
        }
    }

    async fn extract_message_content(&self, message: &crate::slack_client::SlackMessage) -> Result<Vec<MessageContent>, Box<dyn std::error::Error>> {
        let mut content = Vec::new();
        
        // Extract files if present
        if !message.files.is_empty() {
            for file in &message.files {
                content.push(MessageContent {
                    content_type: ContentType::FileUpload,
                    content_data: ContentData::File {
                        filename: file.name.clone().unwrap_or_default(),
                        file_type: file.mimetype.clone().unwrap_or_default(),
                        size: file.size.unwrap_or(0) as u64,
                    },
                    share_context: "File shared in discussion".to_string(),
                    discussion_generated: 0, // Would analyze responses to this file
                });
            }
        }
        
        // Extract links from text
        let link_regex = regex::Regex::new(r"https?://[^\s<>]+").unwrap();
        for link_match in link_regex.find_iter(&message.text) {
            let url = link_match.as_str();
            let content_type = self.classify_link_type(url);
            
            content.push(MessageContent {
                content_type,
                content_data: ContentData::Link {
                    url: url.to_string(),
                    title: None, // Would fetch URL metadata
                    description: None,
                },
                share_context: self.extract_link_context(&message.text, url),
                discussion_generated: 0,
            });
        }
        
        Ok(content)
    }

    fn classify_link_type(&self, url: &str) -> ContentType {
        if url.contains("docs.google.com") {
            ContentType::GoogleDocLink
        } else if url.contains("atlassian.net") || url.contains("jira") {
            ContentType::JiraLink
        } else if url.contains("github.com") {
            ContentType::GitHubLink
        } else if url.contains("slack.com") {
            ContentType::SlackLink
        } else {
            ContentType::ExternalLink
        }
    }

    fn extract_link_context(&self, text: &str, url: &str) -> String {
        if let Some(pos) = text.find(url) {
            let start = pos.saturating_sub(100);
            let end = std::cmp::min(text.len(), pos + url.len() + 100);
            text[start..end].to_string()
        } else {
            "".to_string()
        }
    }

    async fn analyze_message_sentiment(&self, text: &str) -> SentimentIndicators {
        // Simple sentiment analysis - would use more sophisticated NLP in practice
        let text_lower = text.to_lowercase();
        
        let urgency_level = if text_lower.contains("urgent") || text_lower.contains("asap") {
            UrgencyLevel::High
        } else if text_lower.contains("when you can") {
            UrgencyLevel::Low
        } else {
            UrgencyLevel::Medium
        };

        let emotional_tone = if text_lower.contains("frustrated") || text_lower.contains("stuck") {
            EmotionalTone::Frustrated
        } else if text_lower.contains("thanks") || text_lower.contains("appreciate") {
            EmotionalTone::Grateful
        } else if text_lower.contains("confused") || text_lower.contains("not sure") {
            EmotionalTone::Confused
        } else {
            EmotionalTone::Neutral
        };

        let confidence_level = if text_lower.contains("definitely") || text_lower.contains("sure") {
            ConfidenceLevel::Confident
        } else if text_lower.contains("think") || text_lower.contains("maybe") {
            ConfidenceLevel::Uncertain
        } else {
            ConfidenceLevel::Exploring
        };

        SentimentIndicators {
            urgency_level,
            emotional_tone,
            problem_severity: ProblemSeverity::Moderate, // Would analyze more thoroughly
            confidence_level,
        }
    }

    async fn analyze_message_knowledge(&self, text: &str) -> MessageKnowledgeIndicators {
        let text_lower = text.to_lowercase();
        
        MessageKnowledgeIndicators {
            contains_solution: text_lower.contains("solution") || text_lower.contains("try this"),
            contains_explanation: text_lower.contains("because") || text_lower.contains("reason"),
            contains_code_example: text.contains("```") || text.contains("`"),
            contains_documentation_reference: text_lower.contains("docs") || text_lower.contains("documentation"),
            knowledge_level_demonstrated: if text_lower.contains("i think") {
                KnowledgeLevel::Beginner
            } else if text_lower.contains("you should") {
                KnowledgeLevel::Advanced
            } else {
                KnowledgeLevel::Intermediate
            },
            teaching_indicators: Vec::new(), // Would analyze for teaching patterns
        }
    }

    // Placeholder implementations for the remaining methods
    async fn analyze_participants(&self, _messages: &[EnhancedSlackMessage]) -> Result<Vec<SlackParticipant>, Box<dyn std::error::Error>> {
        Ok(Vec::new())
    }

    async fn analyze_reactions(&self, _messages: &[EnhancedSlackMessage]) -> Result<ReactionsAnalysis, Box<dyn std::error::Error>> {
        Ok(ReactionsAnalysis {
            total_reactions: 0,
            reaction_diversity: 0,
            reaction_distribution: HashMap::new(),
            most_reacted_message: None,
            reaction_patterns: Vec::new(),
            social_validation_score: 0.0,
        })
    }

    async fn extract_shared_content(&self, _messages: &[EnhancedSlackMessage]) -> Result<Vec<SharedContent>, Box<dyn std::error::Error>> {
        Ok(Vec::new())
    }

    async fn identify_topic_shifts(&self, _messages: &[EnhancedSlackMessage]) -> Result<Vec<TopicShift>, Box<dyn std::error::Error>> {
        Ok(Vec::new())
    }

    async fn analyze_collaboration_patterns(&self, _messages: &[EnhancedSlackMessage], _participants: &[SlackParticipant]) -> Result<Vec<SlackCollaborationPattern>, Box<dyn std::error::Error>> {
        Ok(Vec::new())
    }

    async fn analyze_problem_resolution(&self, _messages: &[EnhancedSlackMessage]) -> Result<Option<ProblemResolutionFlow>, Box<dyn std::error::Error>> {
        Ok(None)
    }

    async fn identify_knowledge_transfer(&self, _messages: &[EnhancedSlackMessage], _participants: &[SlackParticipant]) -> Result<Vec<SlackKnowledgeTransfer>, Box<dyn std::error::Error>> {
        Ok(Vec::new())
    }

    async fn calculate_thread_metadata(&self, messages: &[EnhancedSlackMessage], participants: &[SlackParticipant]) -> Result<ThreadMetadata, Box<dyn std::error::Error>> {
        let thread_duration = if let (Some(first), Some(last)) = (messages.first(), messages.last()) {
            (last.timestamp - first.timestamp).num_seconds() as f64 / 3600.0
        } else {
            0.0
        };

        Ok(ThreadMetadata {
            total_messages: messages.len() as u32,
            unique_participants: participants.len() as u32,
            thread_duration_hours: thread_duration,
            peak_activity_period: None, // Would analyze message timing patterns
            thread_category: ThreadCategory::TechnicalSupport, // Would classify based on content
            business_impact: BusinessImpact::Medium,
            knowledge_value: KnowledgeValue::Medium,
        })
    }

    fn derive_thread_title(&self, messages: &[EnhancedSlackMessage]) -> Option<String> {
        // Extract title from first message, limited to reasonable length
        messages.first().map(|msg| {
            let text = &msg.text;
            if text.len() > 100 {
                format!("{}...", &text[..97])
            } else {
                text.clone()
            }
        })
    }
}