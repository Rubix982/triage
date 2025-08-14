use crate::google_auth::GoogleAuthManager;
use crate::google_client::GoogleApiClient;
use crate::slack_auth::SlackAuthManager;
use crate::slack_client::SlackApiClient;
use crate::types::{ExtractedLink, PlatformType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::{mpsc, Semaphore};
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentExtractionJob {
    pub id: Uuid,
    pub source_ticket_id: String,
    pub extracted_link: ExtractedLink,
    pub user_id: String,
    pub priority: JobPriority,
    pub retry_count: u32,
    pub created_at: DateTime<Utc>,
    pub scheduled_for: DateTime<Utc>,
    pub status: JobStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum JobPriority {
    High,    // Recently updated tickets or user-requested
    Medium,  // Standard processing
    Low,     // Bulk historical processing
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
    Retrying,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtractedContent {
    pub id: Uuid,
    pub job_id: Uuid,
    pub platform_type: PlatformType,
    pub source_url: String,
    pub title: String,
    pub body_text: String,
    pub metadata: ContentMetadata,
    pub extracted_at: DateTime<Utc>,
    pub source_ticket_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentMetadata {
    pub author: Option<String>,
    pub created_time: Option<String>,
    pub modified_time: Option<String>,
    pub sharing_info: Option<serde_json::Value>,
    pub comments_count: u32,
    pub suggestions_count: u32,
    pub revisions_count: u32,
    pub content_length: u32,
    pub content_type_specific: serde_json::Value,
}

pub struct ContentExtractionService {
    job_queue: Arc<tokio::sync::Mutex<Vec<ContentExtractionJob>>>,
    google_clients: HashMap<String, GoogleApiClient>, // user_id -> client
    slack_clients: HashMap<String, SlackApiClient>, // team_id -> client
    rate_limiters: HashMap<PlatformType, Arc<Semaphore>>,
    worker_count: usize,
}

impl ContentExtractionService {
    pub fn new(worker_count: usize) -> Self {
        let mut rate_limiters = HashMap::new();
        
        // Google APIs have specific rate limits
        // Documents API: 100 requests per 100 seconds per user
        // Drive API: 1000 requests per 100 seconds per user
        // Sheets API: 300 requests per 100 seconds per user
        rate_limiters.insert(
            PlatformType::GoogleDocs { document_id: "".to_string() }, 
            Arc::new(Semaphore::new(50)) // Conservative limit
        );
        rate_limiters.insert(
            PlatformType::GoogleSheets { spreadsheet_id: "".to_string() }, 
            Arc::new(Semaphore::new(100))
        );
        rate_limiters.insert(
            PlatformType::GoogleSlides { presentation_id: "".to_string() }, 
            Arc::new(Semaphore::new(50))
        );

        // Slack API rate limits
        // Most methods: Tier 2 = 20+ requests per minute
        // conversations.history: Tier 3 = 50+ requests per minute
        // User token methods have stricter limits
        rate_limiters.insert(
            PlatformType::SlackThread { workspace: "".to_string(), channel: "".to_string(), thread_ts: "".to_string() },
            Arc::new(Semaphore::new(20)) // Conservative limit for thread extraction
        );
        rate_limiters.insert(
            PlatformType::SlackMessage { workspace: "".to_string(), channel: "".to_string(), message_ts: "".to_string() },
            Arc::new(Semaphore::new(20)) // Conservative limit for message extraction
        );

        Self {
            job_queue: Arc::new(tokio::sync::Mutex::new(Vec::new())),
            google_clients: HashMap::new(),
            slack_clients: HashMap::new(),
            rate_limiters,
            worker_count,
        }
    }

    pub async fn add_extraction_jobs(&self, jobs: Vec<ContentExtractionJob>) {
        let mut queue = self.job_queue.lock().await;
        queue.extend(jobs);
        
        // Sort by priority and creation time
        queue.sort_by(|a, b| {
            let priority_order = match (&a.priority, &b.priority) {
                (JobPriority::High, JobPriority::High) => std::cmp::Ordering::Equal,
                (JobPriority::High, _) => std::cmp::Ordering::Less,
                (_, JobPriority::High) => std::cmp::Ordering::Greater,
                (JobPriority::Medium, JobPriority::Medium) => std::cmp::Ordering::Equal,
                (JobPriority::Medium, JobPriority::Low) => std::cmp::Ordering::Less,
                (JobPriority::Low, JobPriority::Medium) => std::cmp::Ordering::Greater,
                (JobPriority::Low, JobPriority::Low) => std::cmp::Ordering::Equal,
            };
            
            if priority_order == std::cmp::Ordering::Equal {
                a.created_at.cmp(&b.created_at)
            } else {
                priority_order
            }
        });
    }

    pub fn add_google_client(&mut self, user_id: String, auth_manager: GoogleAuthManager) {
        let google_client = GoogleApiClient::new(auth_manager);
        self.google_clients.insert(user_id, google_client);
    }

    pub fn add_slack_client(&mut self, team_id: String, auth_manager: SlackAuthManager) {
        let slack_client = SlackApiClient::new(auth_manager);
        self.slack_clients.insert(team_id, slack_client);
    }

    pub async fn start_workers(&mut self) -> mpsc::Receiver<ExtractedContent> {
        let (content_tx, content_rx) = mpsc::channel(1000);
        
        for worker_id in 0..self.worker_count {
            let queue = Arc::clone(&self.job_queue);
            let tx = content_tx.clone();
            let rate_limiters = self.rate_limiters.clone();
            
            tokio::spawn(async move {
                Self::content_extraction_worker(
                    worker_id,
                    queue,
                    tx,
                    rate_limiters,
                ).await;
            });
        }
        
        content_rx
    }

    async fn content_extraction_worker(
        worker_id: usize,
        job_queue: Arc<tokio::sync::Mutex<Vec<ContentExtractionJob>>>,
        content_sender: mpsc::Sender<ExtractedContent>,
        rate_limiters: HashMap<PlatformType, Arc<Semaphore>>,
    ) {
        println!("🔄 Content extraction worker {} started", worker_id);
        
        loop {
            // Get next job from queue
            let job = {
                let mut queue = job_queue.lock().await;
                queue.iter().position(|job| {
                    job.status == JobStatus::Pending && 
                    job.scheduled_for <= Utc::now()
                }).and_then(|index| Some(queue.remove(index)))
            };
            
            if let Some(mut job) = job {
                println!("📋 Worker {} processing job {} for {}", worker_id, job.id, job.extracted_link.url);
                
                // Update job status
                job.status = JobStatus::Processing;
                
                // Get appropriate rate limiter
                let platform_key = Self::get_platform_key_for_rate_limiting(&job.extracted_link.platform_type);
                let rate_limiter = rate_limiters.get(&platform_key);
                
                if let Some(semaphore) = rate_limiter {
                    let _permit = semaphore.acquire().await.unwrap();
                    
                    match Self::process_extraction_job(&job).await {
                        Ok(content) => {
                            println!("✅ Worker {} successfully extracted content from {}", worker_id, job.extracted_link.url);
                            let _ = content_sender.send(content).await;
                        },
                        Err(e) => {
                            eprintln!("❌ Worker {} failed to extract content from {}: {}", worker_id, job.extracted_link.url, e);
                            
                            // Handle retry logic
                            job.retry_count += 1;
                            if job.retry_count < 3 {
                                job.status = JobStatus::Retrying;
                                job.scheduled_for = Utc::now() + chrono::Duration::minutes(job.retry_count as i64 * 5);
                                
                                // Re-add to queue for retry
                                let mut queue = job_queue.lock().await;
                                queue.push(job);
                            } else {
                                job.status = JobStatus::Failed(e.to_string());
                                eprintln!("💀 Job {} failed permanently after {} retries", job.id, job.retry_count);
                            }
                        }
                    }
                } else {
                    eprintln!("⚠️ No rate limiter found for platform type: {:?}", job.extracted_link.platform_type);
                }
            } else {
                // No jobs available, sleep for a bit
                sleep(Duration::from_secs(5)).await;
            }
        }
    }

    async fn process_extraction_job(job: &ContentExtractionJob) -> Result<ExtractedContent, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: This would need access to the appropriate Google client for the user
        // For now, we'll create a mock implementation that would be replaced with actual Google API calls
        
        // Simulate content extraction
        let title = format!("Document from {}", job.extracted_link.url);
        let body_text = "Mock extracted content".to_string();
        
        let metadata = ContentMetadata {
            author: Some("Unknown".to_string()),
            created_time: None,
            modified_time: None,
            sharing_info: None,
            comments_count: 0,
            suggestions_count: 0,
            revisions_count: 0,
            content_length: body_text.len() as u32,
            content_type_specific: serde_json::json!({}),
        };
        
        Ok(ExtractedContent {
            id: Uuid::new_v4(),
            job_id: job.id,
            platform_type: job.extracted_link.platform_type.clone(),
            source_url: job.extracted_link.url.clone(),
            title,
            body_text,
            metadata,
            extracted_at: Utc::now(),
            source_ticket_ids: vec![job.source_ticket_id.clone()],
        })
    }

    fn get_platform_key_for_rate_limiting(platform_type: &PlatformType) -> PlatformType {
        match platform_type {
            PlatformType::GoogleDocs { .. } => PlatformType::GoogleDocs { document_id: "".to_string() },
            PlatformType::GoogleSheets { .. } => PlatformType::GoogleSheets { spreadsheet_id: "".to_string() },
            PlatformType::GoogleSlides { .. } => PlatformType::GoogleSlides { presentation_id: "".to_string() },
            PlatformType::SlackThread { .. } => PlatformType::SlackThread { workspace: "".to_string(), channel: "".to_string(), thread_ts: "".to_string() },
            PlatformType::SlackMessage { .. } => PlatformType::SlackMessage { workspace: "".to_string(), channel: "".to_string(), message_ts: "".to_string() },
            other => other.clone(),
        }
    }

    pub async fn get_queue_status(&self) -> (usize, HashMap<JobStatus, usize>) {
        let queue = self.job_queue.lock().await;
        let total_jobs = queue.len();
        
        let mut status_counts = HashMap::new();
        for job in queue.iter() {
            let count = status_counts.entry(job.status.clone()).or_insert(0);
            *count += 1;
        }
        
        (total_jobs, status_counts)
    }

    pub async fn get_jobs_for_ticket(&self, ticket_id: &str) -> Vec<ContentExtractionJob> {
        let queue = self.job_queue.lock().await;
        queue.iter()
            .filter(|job| job.source_ticket_id == ticket_id)
            .cloned()
            .collect()
    }
}

// Helper function to create jobs from extracted links
pub fn create_extraction_jobs_from_links(
    ticket_id: &str,
    extracted_links: &[ExtractedLink],
    user_id: &str,
    priority: JobPriority,
) -> Vec<ContentExtractionJob> {
    let now = Utc::now();
    
    extracted_links.iter()
        .filter(|link| {
            // Only create jobs for supported platform types
            matches!(
                link.platform_type,
                PlatformType::GoogleDocs { .. } |
                PlatformType::GoogleSheets { .. } |
                PlatformType::GoogleSlides { .. } |
                PlatformType::SlackThread { .. } |
                PlatformType::SlackMessage { .. }
            )
        })
        .map(|link| ContentExtractionJob {
            id: Uuid::new_v4(),
            source_ticket_id: ticket_id.to_string(),
            extracted_link: link.clone(),
            user_id: user_id.to_string(),
            priority: priority.clone(),
            retry_count: 0,
            created_at: now,
            scheduled_for: now,
            status: JobStatus::Pending,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_creation_from_links() {
        let links = vec![
            ExtractedLink {
                url: "https://docs.google.com/document/d/123".to_string(),
                platform_type: PlatformType::GoogleDocs { document_id: "123".to_string() },
                link_context: "description".to_string(),
                extraction_metadata: None,
            },
            ExtractedLink {
                url: "https://slack.com/channel/123".to_string(),
                platform_type: PlatformType::SlackMessage { 
                    workspace: "test".to_string(), 
                    channel: "123".to_string(), 
                    message_ts: "456".to_string() 
                },
                link_context: "comment".to_string(),
                extraction_metadata: None,
            },
        ];

        let jobs = create_extraction_jobs_from_links("TICKET-123", &links, "user1", JobPriority::Medium);
        
        // Should only create job for Google Docs, not Slack
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].source_ticket_id, "TICKET-123");
        assert!(matches!(jobs[0].extracted_link.platform_type, PlatformType::GoogleDocs { .. }));
    }

    #[tokio::test]
    async fn test_content_extraction_service() {
        let mut service = ContentExtractionService::new(2);
        
        let jobs = vec![
            ContentExtractionJob {
                id: Uuid::new_v4(),
                source_ticket_id: "TEST-123".to_string(),
                extracted_link: ExtractedLink {
                    url: "https://docs.google.com/document/d/123".to_string(),
                    platform_type: PlatformType::GoogleDocs { document_id: "123".to_string() },
                    link_context: "description".to_string(),
                    extraction_metadata: None,
                },
                user_id: "user1".to_string(),
                priority: JobPriority::High,
                retry_count: 0,
                created_at: Utc::now(),
                scheduled_for: Utc::now(),
                status: JobStatus::Pending,
            }
        ];

        service.add_extraction_jobs(jobs).await;
        
        let (total, status_counts) = service.get_queue_status().await;
        assert_eq!(total, 1);
        assert_eq!(*status_counts.get(&JobStatus::Pending).unwrap_or(&0), 1);
    }
}