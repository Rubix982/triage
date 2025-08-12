use crate::auth::{authenticate, get_domain};
use crate::people_graph::{IdentityResolver, DetailedInteraction, InteractionType, InteractionContext, ImpactIndicators};
use crate::types::{Issue, IssueFields, IssueStatus, IssueType, IssuePriority, ExtractedLink};
use crate::utils::{log_step, log_success, log_error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// ================================
// ENHANCED JIRA DATA STRUCTURES
// ================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnhancedJiraIssue {
    pub issue: Issue,
    pub comments: Vec<JiraComment>,
    pub transitions: Vec<JiraTransition>,
    pub watchers: Vec<String>,
    pub participants: Vec<ParticipantSummary>,
    pub mention_network: Vec<MentionEvent>,
    pub link_shares: Vec<LinkShare>,
    pub collaborative_metadata: CollaborativeMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JiraComment {
    pub id: String,
    pub author: JiraUser,
    pub body: String,
    pub created: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>,
    pub mentions: Vec<MentionEvent>,
    pub attachments: Vec<JiraAttachment>,
    pub visibility: Option<CommentVisibility>,
    pub parent_comment_id: Option<String>, // if this is a reply
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JiraUser {
    pub account_id: String,
    pub display_name: String,
    pub email_address: Option<String>,
    pub avatar_urls: Option<HashMap<String, String>>,
    pub profile_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JiraTransition {
    pub id: String,
    pub from_status: String,
    pub to_status: String,
    pub author: JiraUser,
    pub timestamp: DateTime<Utc>,
    pub comment: Option<String>,
    pub transition_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParticipantSummary {
    pub person_id: String,
    pub roles: Vec<ParticipantRole>,
    pub interaction_count: u32,
    pub first_interaction: DateTime<Utc>,
    pub last_interaction: DateTime<Utc>,
    pub influence_score: f64, // calculated based on their contributions
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ParticipantRole {
    Reporter,
    Assignee,
    Commenter,
    Watcher,
    Transitioner, // moved ticket through workflow
    Linker, // shared external links
    Mentioner, // mentioned others
    Mentioned, // was mentioned by others
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MentionEvent {
    pub mentioned_user: JiraUser,
    pub mentioned_by: JiraUser,
    pub context: String, // surrounding text
    pub location: MentionLocation,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MentionLocation {
    Description,
    Comment { comment_id: String },
    Summary,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkShare {
    pub url: String,
    pub shared_by: JiraUser,
    pub shared_in: LinkLocation,
    pub context: String, // surrounding text
    pub timestamp: DateTime<Utc>,
    pub link_type: ExtractedLink,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LinkLocation {
    Description,
    Comment { comment_id: String },
    Attachment,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JiraAttachment {
    pub id: String,
    pub filename: String,
    pub author: JiraUser,
    pub created: DateTime<Utc>,
    pub size: Option<u64>,
    pub mime_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentVisibility {
    pub visibility_type: String, // "group", "role", "public"
    pub value: Option<String>, // group name or role name
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollaborativeMetadata {
    pub total_participants: u32,
    pub comment_threads: Vec<CommentThread>,
    pub collaboration_patterns: Vec<CollaborationPattern>,
    pub knowledge_transfer_events: Vec<KnowledgeTransferEvent>,
    pub problem_resolution_chain: Vec<ResolutionStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentThread {
    pub thread_id: String,
    pub participants: Vec<String>, // person IDs
    pub message_count: u32,
    pub started_by: String,
    pub last_activity: DateTime<Utc>,
    pub topic_keywords: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollaborationPattern {
    pub pattern_type: CollaborationPatternType,
    pub participants: Vec<String>,
    pub frequency: u32,
    pub effectiveness_score: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CollaborationPatternType {
    QuestionAnswer, // one person asks, another answers
    ReviewCycle, // back and forth review discussion
    Implementation, // discussion followed by implementation
    Escalation, // passed from person to person
    KnowledgeShare, // someone explains concept to others
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnowledgeTransferEvent {
    pub teacher: String, // person ID who shared knowledge
    pub learners: Vec<String>, // person IDs who received knowledge
    pub knowledge_topic: String,
    pub transfer_method: TransferMethod,
    pub evidence_strength: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferMethod {
    DetailedExplanation, // long comment explaining concept
    LinkSharing, // shared relevant documentation
    StepByStepGuide, // provided instructions
    PairProgramming, // worked together (inferred)
    Mentioning, // connected person with expertise
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResolutionStep {
    pub step_number: u32,
    pub actor: String, // person ID
    pub action: ResolutionAction,
    pub timestamp: DateTime<Utc>,
    pub outcome: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResolutionAction {
    IdentifiedProblem,
    ProposedSolution,
    ImplementedFix,
    TestedSolution,
    VerifiedResolution,
    DocumentedSolution,
}

// ================================
// ENHANCED JIRA EXTRACTOR
// ================================

pub struct EnhancedJiraExtractor {
    client: Client,
    token: String,
    domain: String,
    identity_resolver: IdentityResolver,
}

impl EnhancedJiraExtractor {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            token: "".to_string(), // Will be set during initialization
            domain: get_domain(),
            identity_resolver: IdentityResolver::new(),
        }
    }

    pub async fn initialize(&mut self) {
        self.token = authenticate().await;
    }

    pub async fn extract_enhanced_issue(&self, issue_key: &str) -> Result<EnhancedJiraIssue, Box<dyn std::error::Error>> {
        log_step("🔍", &format!("Extracting enhanced data for issue {}", issue_key));

        // Get basic issue data
        let issue = self.fetch_issue_with_fields(issue_key).await?;
        
        // Get detailed comments with full metadata
        let comments = self.fetch_detailed_comments(&issue.id).await?;
        
        // Get issue transitions (workflow history)
        let transitions = self.fetch_issue_transitions(&issue.id).await?;
        
        // Get watchers (if available)
        let watchers = self.fetch_issue_watchers(&issue.id).await.unwrap_or_default();
        
        // Extract mentions from all content
        let mention_network = self.extract_mentions(&issue, &comments).await?;
        
        // Extract shared links from all content  
        let link_shares = self.extract_link_shares(&issue, &comments).await?;
        
        // Build participant summary
        let participants = self.build_participant_summary(&issue, &comments, &transitions, &watchers).await?;
        
        // Analyze collaborative patterns
        let collaborative_metadata = self.analyze_collaboration_patterns(&issue, &comments, &transitions, &participants).await?;

        let enhanced_issue = EnhancedJiraIssue {
            issue,
            comments,
            transitions,
            watchers,
            participants,
            mention_network,
            link_shares,
            collaborative_metadata,
        };

        log_success(&format!("Enhanced extraction complete for {}", issue_key));
        Ok(enhanced_issue)
    }

    async fn fetch_issue_with_fields(&self, issue_key: &str) -> Result<Issue, Box<dyn std::error::Error>> {
        let url = format!(
            "https://{}.atlassian.net/rest/api/3/issue/{}?expand=names,schema,transitions,editmeta,changelog,versionedRepresentations",
            self.domain, issue_key
        );

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Basic {}", self.token))
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to fetch issue {}: {}", issue_key, response.status()).into());
        }

        let json: Value = response.json().await?;
        
        // Convert to your existing Issue struct
        let issue = Issue {
            id: json["id"].as_str().unwrap_or_default().to_string(),
            key: json["key"].as_str().unwrap_or_default().to_string(),
            fields: IssueFields {
                summary: json["fields"]["summary"].as_str().map(|s| s.to_string()),
                description: json["fields"]["description"].as_object().map(|v| serde_json::Value::Object(v.clone())),
                status: json["fields"]["status"].as_object().map(|status_obj| {
                    IssueStatus {
                        name: status_obj.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string()
                    }
                }),
                created: json["fields"]["created"].as_str().map(|s| s.to_string()),
                updated: json["fields"]["updated"].as_str().map(|s| s.to_string()),
                issuetype: json["fields"]["issuetype"].as_object().map(|it| {
                    IssueType {
                        name: it.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                        id: it.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                        description: it.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        subtask: it.get("subtask").and_then(|v| v.as_bool()),
                        hierarchyLevel: it.get("hierarchyLevel").and_then(|v| v.as_i64()).map(|i| i as i32),
                    }
                }),
                priority: json["fields"]["priority"].as_object().map(|p| {
                    IssuePriority {
                        name: p.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                        id: p.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                    }
                }),
                assignee: json["fields"]["assignee"].clone(),
                reporter: json["fields"]["reporter"].clone(),
                labels: json["fields"]["labels"].as_array().map(|arr| {
                    arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
                }),
                components: json["fields"]["components"].clone(),
                parent: json["fields"]["parent"].clone(),
            }
        };

        Ok(issue)
    }

    async fn fetch_detailed_comments(&self, issue_id: &str) -> Result<Vec<JiraComment>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://{}.atlassian.net/rest/api/3/issue/{}/comment?expand=renderedBody",
            self.domain, issue_id
        );

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Basic {}", self.token))
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(Vec::new()); // Return empty if comments can't be fetched
        }

        let json: Value = response.json().await?;
        let mut comments = Vec::new();

        if let Some(comment_array) = json["comments"].as_array() {
            for comment_json in comment_array {
                let comment = self.parse_jira_comment(comment_json).await?;
                comments.push(comment);
            }
        }

        Ok(comments)
    }

    async fn parse_jira_comment(&self, json: &Value) -> Result<JiraComment, Box<dyn std::error::Error>> {
        let author_json = &json["author"];
        let author = JiraUser {
            account_id: author_json["accountId"].as_str().unwrap_or_default().to_string(),
            display_name: author_json["displayName"].as_str().unwrap_or_default().to_string(),
            email_address: author_json["emailAddress"].as_str().map(|s| s.to_string()),
            avatar_urls: None, // Could parse avatar URLs if needed
            profile_url: None,
        };

        let body = json["body"].as_str().unwrap_or_default().to_string();
        let created_str = json["created"].as_str().unwrap_or_default();
        let created = DateTime::parse_from_rfc3339(created_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let updated = json["updated"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        // Extract mentions from comment body
        let mentions = self.extract_mentions_from_text(&body, &author, MentionLocation::Comment { 
            comment_id: json["id"].as_str().unwrap_or_default().to_string() 
        }).await;

        // Parse attachments if any
        let attachments = if let Some(attachment_array) = json["attachments"].as_array() {
            attachment_array.iter().filter_map(|att_json| {
                self.parse_jira_attachment(att_json).ok()
            }).collect()
        } else {
            Vec::new()
        };

        Ok(JiraComment {
            id: json["id"].as_str().unwrap_or_default().to_string(),
            author,
            body,
            created,
            updated,
            mentions,
            attachments,
            visibility: None, // Could parse visibility if needed
            parent_comment_id: None, // Jira doesn't have nested comments typically
        })
    }

    fn parse_jira_attachment(&self, json: &Value) -> Result<JiraAttachment, Box<dyn std::error::Error>> {
        let author_json = &json["author"];
        let author = JiraUser {
            account_id: author_json["accountId"].as_str().unwrap_or_default().to_string(),
            display_name: author_json["displayName"].as_str().unwrap_or_default().to_string(),
            email_address: author_json["emailAddress"].as_str().map(|s| s.to_string()),
            avatar_urls: None,
            profile_url: None,
        };

        let created_str = json["created"].as_str().unwrap_or_default();
        let created = DateTime::parse_from_rfc3339(created_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(JiraAttachment {
            id: json["id"].as_str().unwrap_or_default().to_string(),
            filename: json["filename"].as_str().unwrap_or_default().to_string(),
            author,
            created,
            size: json["size"].as_u64(),
            mime_type: json["mimeType"].as_str().map(|s| s.to_string()),
        })
    }

    async fn fetch_issue_transitions(&self, issue_id: &str) -> Result<Vec<JiraTransition>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://{}.atlassian.net/rest/api/3/issue/{}/changelog",
            self.domain, issue_id
        );

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Basic {}", self.token))
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let json: Value = response.json().await?;
        let mut transitions = Vec::new();

        if let Some(values) = json["values"].as_array() {
            for change_json in values {
                if let Some(items) = change_json["items"].as_array() {
                    for item in items {
                        if item["field"] == "status" {
                            let transition = self.parse_status_transition(change_json, item).await?;
                            transitions.push(transition);
                        }
                    }
                }
            }
        }

        Ok(transitions)
    }

    async fn parse_status_transition(&self, change_json: &Value, item_json: &Value) -> Result<JiraTransition, Box<dyn std::error::Error>> {
        let author_json = &change_json["author"];
        let author = JiraUser {
            account_id: author_json["accountId"].as_str().unwrap_or_default().to_string(),
            display_name: author_json["displayName"].as_str().unwrap_or_default().to_string(),
            email_address: author_json["emailAddress"].as_str().map(|s| s.to_string()),
            avatar_urls: None,
            profile_url: None,
        };

        let timestamp_str = change_json["created"].as_str().unwrap_or_default();
        let timestamp = DateTime::parse_from_rfc3339(timestamp_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(JiraTransition {
            id: change_json["id"].as_str().unwrap_or_default().to_string(),
            from_status: item_json["fromString"].as_str().unwrap_or_default().to_string(),
            to_status: item_json["toString"].as_str().unwrap_or_default().to_string(),
            author,
            timestamp,
            comment: None, // Could extract transition comments if available
            transition_name: format!("{} → {}", 
                item_json["fromString"].as_str().unwrap_or_default(),
                item_json["toString"].as_str().unwrap_or_default()
            ),
        })
    }

    async fn fetch_issue_watchers(&self, issue_id: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://{}.atlassian.net/rest/api/3/issue/{}/watchers",
            self.domain, issue_id
        );

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Basic {}", self.token))
            .header("Accept", "application/json")
            .send()
            .await;

        // Watchers endpoint might not be accessible to all users
        match response {
            Ok(resp) if resp.status().is_success() => {
                let json: Value = resp.json().await?;
                let mut watchers = Vec::new();
                
                if let Some(watcher_array) = json["watchers"].as_array() {
                    for watcher_json in watcher_array {
                        if let Some(account_id) = watcher_json["accountId"].as_str() {
                            // Resolve to person ID
                            let person_id = self.identity_resolver.resolve_identity(
                                watcher_json["emailAddress"].as_str(),
                                watcher_json["displayName"].as_str().unwrap_or_default(),
                                "jira",
                                account_id
                            ).await?;
                            watchers.push(person_id);
                        }
                    }
                }
                Ok(watchers)
            },
            _ => Ok(Vec::new()) // Return empty if we can't access watchers
        }
    }

    async fn extract_mentions(&self, issue: &Issue, comments: &[JiraComment]) -> Result<Vec<MentionEvent>, Box<dyn std::error::Error>> {
        let mut mentions = Vec::new();

        // Extract mentions from issue description
        if let Some(description) = &issue.fields.description {
            let desc_mentions = self.extract_mentions_from_text(
                &serde_json::to_string(description).unwrap_or_default(),
                &JiraUser {
                    account_id: issue.fields.reporter.as_ref()
                        .and_then(|r| r.get("accountId"))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default().to_string(),
                    display_name: issue.fields.reporter.as_ref()
                        .and_then(|r| r.get("displayName"))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default().to_string(),
                    email_address: issue.fields.reporter.as_ref()
                        .and_then(|r| r.get("emailAddress"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    avatar_urls: None,
                    profile_url: None,
                },
                MentionLocation::Description
            ).await;
            mentions.extend(desc_mentions);
        }

        // Extract mentions from comments
        for comment in comments {
            let comment_mentions = self.extract_mentions_from_text(
                &comment.body,
                &comment.author,
                MentionLocation::Comment { comment_id: comment.id.clone() }
            ).await;
            mentions.extend(comment_mentions);
        }

        Ok(mentions)
    }

    async fn extract_mentions_from_text(&self, text: &str, author: &JiraUser, location: MentionLocation) -> Vec<MentionEvent> {
        let mut mentions = Vec::new();
        
        // Simple @mention detection - could be enhanced with more sophisticated parsing
        let mention_regex = regex::Regex::new(r"@(\w+)").unwrap();
        
        for capture in mention_regex.captures_iter(text) {
            if let Some(username) = capture.get(1) {
                // You'd need to resolve the username to a full JiraUser
                // This is a simplified version
                let mentioned_user = JiraUser {
                    account_id: username.as_str().to_string(), // This would need proper resolution
                    display_name: username.as_str().to_string(),
                    email_address: None,
                    avatar_urls: None,
                    profile_url: None,
                };
                
                mentions.push(MentionEvent {
                    mentioned_user,
                    mentioned_by: author.clone(),
                    context: self.extract_context_around_mention(text, username.start()),
                    location: location.clone(),
                    timestamp: Utc::now(), // Would use actual timestamp from content
                });
            }
        }
        
        mentions
    }

    fn extract_context_around_mention(&self, text: &str, position: usize) -> String {
        let start = position.saturating_sub(50);
        let end = std::cmp::min(text.len(), position + 50);
        text[start..end].to_string()
    }

    async fn extract_link_shares(&self, issue: &Issue, comments: &[JiraComment]) -> Result<Vec<LinkShare>, Box<dyn std::error::Error>> {
        let mut link_shares = Vec::new();
        
        // Extract links from issue description
        if let Some(description) = &issue.fields.description {
            let desc_text = serde_json::to_string(description).unwrap_or_default();
            let desc_links = self.extract_links_from_text(
                &desc_text,
                &JiraUser {
                    account_id: issue.fields.reporter.as_ref()
                        .and_then(|r| r.get("accountId"))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default().to_string(),
                    display_name: issue.fields.reporter.as_ref()
                        .and_then(|r| r.get("displayName"))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default().to_string(),
                    email_address: issue.fields.reporter.as_ref()
                        .and_then(|r| r.get("emailAddress"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    avatar_urls: None,
                    profile_url: None,
                },
                LinkLocation::Description,
                Utc::now() // Would use actual issue creation time
            ).await;
            link_shares.extend(desc_links);
        }

        // Extract links from comments
        for comment in comments {
            let comment_links = self.extract_links_from_text(
                &comment.body,
                &comment.author,
                LinkLocation::Comment { comment_id: comment.id.clone() },
                comment.created
            ).await;
            link_shares.extend(comment_links);
        }

        Ok(link_shares)
    }

    async fn extract_links_from_text(&self, text: &str, author: &JiraUser, location: LinkLocation, timestamp: DateTime<Utc>) -> Vec<LinkShare> {
        let mut link_shares = Vec::new();
        
        // Use your existing link detector
        let link_detector = crate::link_detector::LinkDetector::new();
        let extracted_links = link_detector.extract_links_from_content(text);
        
        for extracted_link in extracted_links {
            let context = self.extract_context_around_url(text, &extracted_link.url);
            
            link_shares.push(LinkShare {
                url: extracted_link.url,
                shared_by: author.clone(),
                shared_in: location.clone(),
                context,
                timestamp,
                link_type: extracted_link,
            });
        }
        
        link_shares
    }

    fn extract_context_around_url(&self, text: &str, url: &str) -> String {
        if let Some(pos) = text.find(url) {
            let start = pos.saturating_sub(100);
            let end = std::cmp::min(text.len(), pos + url.len() + 100);
            text[start..end].to_string()
        } else {
            "".to_string()
        }
    }

    async fn build_participant_summary(&self, 
        issue: &Issue, 
        comments: &[JiraComment], 
        transitions: &[JiraTransition],
        watchers: &[String]
    ) -> Result<Vec<ParticipantSummary>, Box<dyn std::error::Error>> {
        
        let mut participants: HashMap<String, ParticipantSummary> = HashMap::new();
        
        // Add reporter
        if let Some(reporter) = &issue.fields.reporter {
            let person_id = self.identity_resolver.resolve_identity(
                reporter.get("emailAddress").and_then(|v| v.as_str()),
                reporter.get("displayName").and_then(|v| v.as_str()).unwrap_or_default(),
                "jira",
                reporter.get("accountId").and_then(|v| v.as_str()).unwrap_or_default()
            ).await?;
            
            participants.insert(person_id.clone(), ParticipantSummary {
                person_id: person_id.clone(),
                roles: vec![ParticipantRole::Reporter],
                interaction_count: 1,
                first_interaction: Utc::now(), // Would use actual issue creation time
                last_interaction: Utc::now(),
                influence_score: 0.8, // Reporters have high influence
            });
        }

        // Add assignee
        if let Some(assignee) = &issue.fields.assignee {
            let person_id = self.identity_resolver.resolve_identity(
                assignee.get("emailAddress").and_then(|v| v.as_str()),
                assignee.get("displayName").and_then(|v| v.as_str()).unwrap_or_default(),
                "jira",
                assignee.get("accountId").and_then(|v| v.as_str()).unwrap_or_default()
            ).await?;
            
            if let Some(participant) = participants.get_mut(&person_id) {
                participant.roles.push(ParticipantRole::Assignee);
                participant.influence_score += 0.6;
            } else {
                participants.insert(person_id.clone(), ParticipantSummary {
                    person_id: person_id.clone(),
                    roles: vec![ParticipantRole::Assignee],
                    interaction_count: 1,
                    first_interaction: Utc::now(),
                    last_interaction: Utc::now(),
                    influence_score: 0.6,
                });
            }
        }

        // Add commenters
        for comment in comments {
            let person_id = self.identity_resolver.resolve_identity(
                comment.author.email_address.as_deref(),
                &comment.author.display_name,
                "jira",
                &comment.author.account_id
            ).await?;
            
            if let Some(participant) = participants.get_mut(&person_id) {
                if !participant.roles.contains(&ParticipantRole::Commenter) {
                    participant.roles.push(ParticipantRole::Commenter);
                }
                participant.interaction_count += 1;
                participant.last_interaction = comment.created;
                participant.influence_score += 0.2; // Each comment adds to influence
            } else {
                participants.insert(person_id.clone(), ParticipantSummary {
                    person_id: person_id.clone(),
                    roles: vec![ParticipantRole::Commenter],
                    interaction_count: 1,
                    first_interaction: comment.created,
                    last_interaction: comment.created,
                    influence_score: 0.2,
                });
            }
        }

        // Add transition makers
        for transition in transitions {
            let person_id = self.identity_resolver.resolve_identity(
                transition.author.email_address.as_deref(),
                &transition.author.display_name,
                "jira",
                &transition.author.account_id
            ).await?;
            
            if let Some(participant) = participants.get_mut(&person_id) {
                if !participant.roles.contains(&ParticipantRole::Transitioner) {
                    participant.roles.push(ParticipantRole::Transitioner);
                }
                participant.interaction_count += 1;
                participant.influence_score += 0.1;
            } else {
                participants.insert(person_id.clone(), ParticipantSummary {
                    person_id: person_id.clone(),
                    roles: vec![ParticipantRole::Transitioner],
                    interaction_count: 1,
                    first_interaction: transition.timestamp,
                    last_interaction: transition.timestamp,
                    influence_score: 0.1,
                });
            }
        }

        // Add watchers
        for watcher_id in watchers {
            if let Some(participant) = participants.get_mut(watcher_id) {
                if !participant.roles.contains(&ParticipantRole::Watcher) {
                    participant.roles.push(ParticipantRole::Watcher);
                }
                participant.influence_score += 0.05; // Watchers have minimal influence
            } else {
                participants.insert(watcher_id.clone(), ParticipantSummary {
                    person_id: watcher_id.clone(),
                    roles: vec![ParticipantRole::Watcher],
                    interaction_count: 0,
                    first_interaction: Utc::now(),
                    last_interaction: Utc::now(),
                    influence_score: 0.05,
                });
            }
        }

        Ok(participants.into_values().collect())
    }

    async fn analyze_collaboration_patterns(&self, 
        _issue: &Issue, 
        comments: &[JiraComment], 
        _transitions: &[JiraTransition],
        participants: &[ParticipantSummary]
    ) -> Result<CollaborativeMetadata, Box<dyn std::error::Error>> {
        
        // Build comment threads
        let comment_threads = self.identify_comment_threads(comments).await;
        
        // Identify collaboration patterns
        let collaboration_patterns = self.identify_collaboration_patterns(comments, participants).await;
        
        // Detect knowledge transfer events
        let knowledge_transfer_events = self.detect_knowledge_transfer(comments).await;
        
        // Build problem resolution chain
        let problem_resolution_chain = self.build_resolution_chain(comments).await;

        Ok(CollaborativeMetadata {
            total_participants: participants.len() as u32,
            comment_threads,
            collaboration_patterns,
            knowledge_transfer_events,
            problem_resolution_chain,
        })
    }

    async fn identify_comment_threads(&self, comments: &[JiraComment]) -> Vec<CommentThread> {
        // Simple threading based on temporal proximity and participant patterns
        // In a real implementation, you'd use more sophisticated thread detection
        vec![CommentThread {
            thread_id: "main_thread".to_string(),
            participants: comments.iter().map(|c| c.author.account_id.clone()).collect(),
            message_count: comments.len() as u32,
            started_by: comments.first().map(|c| c.author.account_id.clone()).unwrap_or_default(),
            last_activity: comments.last().map(|c| c.created).unwrap_or_else(Utc::now),
            topic_keywords: Vec::new(), // Would extract using NLP
        }]
    }

    async fn identify_collaboration_patterns(&self, _comments: &[JiraComment], _participants: &[ParticipantSummary]) -> Vec<CollaborationPattern> {
        // Analyze comment patterns to identify collaboration types
        // This would be more sophisticated in practice
        Vec::new()
    }

    async fn detect_knowledge_transfer(&self, _comments: &[JiraComment]) -> Vec<KnowledgeTransferEvent> {
        // Detect when someone explains concepts or shares knowledge
        // Would use NLP to identify explanatory language patterns
        Vec::new()
    }

    async fn build_resolution_chain(&self, _comments: &[JiraComment]) -> Vec<ResolutionStep> {
        // Identify the sequence of steps that led to problem resolution
        // Would analyze comment sentiment, solution language, etc.
        Vec::new()
    }
}

// Add Default trait for Issue if it doesn't exist
impl Default for Issue {
    fn default() -> Self {
        Issue {
            id: String::new(),
            key: String::new(),
            fields: IssueFields {
                summary: None,
                description: None,
                status: None,
                created: None,
                updated: None,
                issuetype: None,
                priority: None,
                assignee: None,
                reporter: None,
                labels: None,
                components: None,
                parent: None,
            }
        }
    }
}