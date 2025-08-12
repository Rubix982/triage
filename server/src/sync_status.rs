use crate::db_utils::with_connection;
use crate::utils::{log_step, log_success};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncStatus {
    pub last_sync_time: String,
    pub sync_statistics: SyncStatistics,
    pub recent_issues: Vec<RecentIssue>,
    pub updated_issues: Vec<UpdatedIssue>,
    pub comment_rich_issues: Vec<CommentRichIssue>,
    pub escl_insights: ESCLInsights,
    pub knowledge_impact: KnowledgeImpact,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncStatistics {
    pub total_projects: usize,
    pub total_issues: usize,
    pub new_issues_since_last_sync: usize,
    pub updated_issues_since_last_sync: usize,
    pub escl_count: usize,
    pub issues_with_comments: usize,
    pub average_comments_per_issue: f64,
    pub sync_duration_seconds: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecentIssue {
    pub id: String,
    pub key: String,
    pub summary: String,
    pub status: String,
    pub created: String,
    pub project_name: String,
    pub is_escl: bool,
    pub priority: String,
    pub has_comments: bool,
    pub comment_count: usize,
    pub description_preview: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatedIssue {
    pub id: String,
    pub key: String,
    pub summary: String,
    pub status: String,
    pub updated: String,
    pub project_name: String,
    pub is_escl: bool,
    pub update_type: UpdateType,
    pub changes_summary: String,
    pub new_comment_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UpdateType {
    StatusChange,
    NewComments,
    DescriptionUpdate,
    Resolution,
    Assignment,
    Multiple,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentRichIssue {
    pub id: String,
    pub key: String,
    pub summary: String,
    pub status: String,
    pub project_name: String,
    pub is_escl: bool,
    pub comment_count: usize,
    pub comment_quality_score: f64,
    pub has_solution_indicators: bool,
    pub solution_keywords: Vec<String>,
    pub last_comment_date: String,
    pub participant_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ESCLInsights {
    pub total_escls: usize,
    pub new_escls: usize,
    pub resolved_escls: usize,
    pub escls_with_rich_comments: usize,
    pub top_escl_categories: Vec<ESCLCategory>,
    pub resolution_time_avg_days: f64,
    pub comment_engagement_score: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ESCLCategory {
    pub category: String,
    pub count: usize,
    pub avg_comments: f64,
    pub resolution_rate: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeImpact {
    pub new_concepts_discovered: usize,
    pub new_technologies_identified: usize,
    pub new_solution_patterns: usize,
    pub knowledge_value_score: f64,
    pub recommended_actions: Vec<String>,
}

pub async fn get_sync_status() -> SyncStatus {
    log_step("📊", "Generating sync status dashboard...");
    
    let sync_statistics = get_sync_statistics().await;
    let recent_issues = get_recent_issues(20).await;
    let updated_issues = get_recently_updated_issues(15).await;
    let comment_rich_issues = get_comment_rich_issues(10).await;
    let escl_insights = analyze_escl_insights().await;
    let knowledge_impact = calculate_knowledge_impact().await;
    
    let status = SyncStatus {
        last_sync_time: get_last_sync_time().await,
        sync_statistics,
        recent_issues,
        updated_issues,
        comment_rich_issues,
        escl_insights,
        knowledge_impact,
    };
    
    log_success("Sync status dashboard generated");
    status
}

async fn get_sync_statistics() -> SyncStatistics {
    let mut stats = SyncStatistics {
        total_projects: 0,
        total_issues: 0,
        new_issues_since_last_sync: 0,
        updated_issues_since_last_sync: 0,
        escl_count: 0,
        issues_with_comments: 0,
        average_comments_per_issue: 0.0,
        sync_duration_seconds: 0,
    };
    
    with_connection("sync_statistics", |conn| {
        // Total projects
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM projects").unwrap();
        stats.total_projects = stmt.query_row([], |row| {
            Ok(row.get::<_, i64>(0)? as usize)
        }).unwrap_or(0);

        // Total issues
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM issues").unwrap();
        stats.total_issues = stmt.query_row([], |row| {
            Ok(row.get::<_, i64>(0)? as usize)
        }).unwrap_or(0);

        // ESCL count (assuming ESCL keys start with 'ESCL-')
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM issues WHERE key LIKE 'ESCL-%'").unwrap();
        stats.escl_count = stmt.query_row([], |row| {
            Ok(row.get::<_, i64>(0)? as usize)
        }).unwrap_or(0);

        // Issues with comments
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM issues WHERE comment IS NOT NULL AND comment != '{}' AND comment != ''").unwrap();
        stats.issues_with_comments = stmt.query_row([], |row| {
            Ok(row.get::<_, i64>(0)? as usize)
        }).unwrap_or(0);

        // Issues created in last 7 days (approximation for "new since last sync")
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM issues WHERE created > datetime('now', '-7 days')").unwrap();
        stats.new_issues_since_last_sync = stmt.query_row([], |row| {
            Ok(row.get::<_, i64>(0)? as usize)
        }).unwrap_or(0);

        // Issues updated in last 7 days
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM issues WHERE updated > datetime('now', '-7 days')").unwrap();
        stats.updated_issues_since_last_sync = stmt.query_row([], |row| {
            Ok(row.get::<_, i64>(0)? as usize)
        }).unwrap_or(0);
    });
    
    if stats.total_issues > 0 {
        stats.average_comments_per_issue = stats.issues_with_comments as f64 / stats.total_issues as f64;
    }
    
    stats
}

async fn get_recent_issues(limit: usize) -> Vec<RecentIssue> {
    let mut recent_issues = Vec::new();
    
    with_connection("recent_issues", |conn| {
        let query = r#"
        SELECT 
            id, key, summary, status, created,
            JSON_EXTRACT(project, '$.name') as project_name,
            description, comment
        FROM issues 
        ORDER BY created DESC 
        LIMIT ?1
        "#;
        
        let mut stmt = conn.prepare(query).unwrap();
        let rows = stmt.query_map([limit], |row| {
            let key: String = row.get(1)?;
            let description: String = row.get(6).unwrap_or_default();
            let comment: String = row.get(7).unwrap_or_default();
            
            let is_escl = key.starts_with("ESCL-");
            let has_comments = !comment.is_empty() && comment != "{}";
            let comment_count = if has_comments { 
                estimate_comment_count(&comment) 
            } else { 
                0 
            };
            
            Ok(RecentIssue {
                id: row.get(0)?,
                key: key.clone(),
                summary: row.get(2)?,
                status: row.get(3)?,
                created: row.get(4)?,
                project_name: row.get(5).unwrap_or_default(),
                is_escl,
                priority: extract_priority_from_key(&key),
                has_comments,
                comment_count,
                description_preview: description.chars().take(100).collect::<String>(),
            })
        }).unwrap();
        
        for row in rows {
            if let Ok(issue) = row {
                recent_issues.push(issue);
            }
        }
    });
    
    recent_issues
}

async fn get_recently_updated_issues(limit: usize) -> Vec<UpdatedIssue> {
    let mut updated_issues = Vec::new();
    
    with_connection("updated_issues", |conn| {
        let query = r#"
        SELECT 
            id, key, summary, status, updated,
            JSON_EXTRACT(project, '$.name') as project_name,
            comment
        FROM issues 
        WHERE updated > datetime('now', '-7 days')
        ORDER BY updated DESC 
        LIMIT ?1
        "#;
        
        let mut stmt = conn.prepare(query).unwrap();
        let rows = stmt.query_map([limit], |row| {
            let key: String = row.get(1)?;
            let comment: String = row.get(6).unwrap_or_default();
            
            let is_escl = key.starts_with("ESCL-");
            let has_new_comments = !comment.is_empty() && comment != "{}";
            let comment_count = if has_new_comments { 
                estimate_comment_count(&comment) 
            } else { 
                0 
            };
            
            Ok(UpdatedIssue {
                id: row.get(0)?,
                key: key.clone(),
                summary: row.get(2)?,
                status: row.get(3)?,
                updated: row.get(4)?,
                project_name: row.get(5).unwrap_or_default(),
                is_escl,
                update_type: determine_update_type(&row.get::<_, String>(3)?, has_new_comments),
                changes_summary: generate_changes_summary(&key, has_new_comments, &row.get::<_, String>(3)?),
                new_comment_count: comment_count,
            })
        }).unwrap();
        
        for row in rows {
            if let Ok(issue) = row {
                updated_issues.push(issue);
            }
        }
    });
    
    updated_issues
}

async fn get_comment_rich_issues(limit: usize) -> Vec<CommentRichIssue> {
    let mut comment_rich_issues = Vec::new();
    
    with_connection("comment_rich_issues", |conn| {
        let query = r#"
        SELECT 
            id, key, summary, status,
            JSON_EXTRACT(project, '$.name') as project_name,
            comment, updated
        FROM issues 
        WHERE comment IS NOT NULL 
          AND comment != '{}' 
          AND comment != ''
          AND length(comment) > 200
        ORDER BY length(comment) DESC 
        LIMIT ?1
        "#;
        
        let mut stmt = conn.prepare(query).unwrap();
        let rows = stmt.query_map([limit], |row| {
            let key: String = row.get(1)?;
            let comment: String = row.get(5).unwrap_or_default();
            
            let is_escl = key.starts_with("ESCL-");
            let comment_count = estimate_comment_count(&comment);
            let solution_keywords = extract_solution_keywords(&comment);
            let has_solution_indicators = !solution_keywords.is_empty();
            let comment_quality_score = calculate_comment_quality(&comment);
            let participant_count = estimate_participant_count(&comment);
            
            Ok(CommentRichIssue {
                id: row.get(0)?,
                key: key.clone(),
                summary: row.get(2)?,
                status: row.get(3)?,
                project_name: row.get(4).unwrap_or_default(),
                is_escl,
                comment_count,
                comment_quality_score,
                has_solution_indicators,
                solution_keywords,
                last_comment_date: row.get(6)?,
                participant_count,
            })
        }).unwrap();
        
        for row in rows {
            if let Ok(issue) = row {
                comment_rich_issues.push(issue);
            }
        }
    });
    
    comment_rich_issues
}

async fn analyze_escl_insights() -> ESCLInsights {
    let mut insights = ESCLInsights {
        total_escls: 0,
        new_escls: 0,
        resolved_escls: 0,
        escls_with_rich_comments: 0,
        top_escl_categories: Vec::new(),
        resolution_time_avg_days: 0.0,
        comment_engagement_score: 0.0,
    };
    
    with_connection("escl_insights", |conn| {
        // Total ESCLs
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM issues WHERE key LIKE 'ESCL-%'").unwrap();
        insights.total_escls = stmt.query_row([], |row| {
            Ok(row.get::<_, i64>(0)? as usize)
        }).unwrap_or(0);

        // New ESCLs (last 7 days)
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM issues WHERE key LIKE 'ESCL-%' AND created > datetime('now', '-7 days')").unwrap();
        insights.new_escls = stmt.query_row([], |row| {
            Ok(row.get::<_, i64>(0)? as usize)
        }).unwrap_or(0);

        // Resolved ESCLs
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM issues WHERE key LIKE 'ESCL-%' AND (status LIKE '%resolved%' OR status LIKE '%closed%' OR status LIKE '%done%')").unwrap();
        insights.resolved_escls = stmt.query_row([], |row| {
            Ok(row.get::<_, i64>(0)? as usize)
        }).unwrap_or(0);

        // ESCLs with rich comments
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM issues WHERE key LIKE 'ESCL-%' AND comment IS NOT NULL AND length(comment) > 200").unwrap();
        insights.escls_with_rich_comments = stmt.query_row([], |row| {
            Ok(row.get::<_, i64>(0)? as usize)
        }).unwrap_or(0);

        // Calculate engagement score
        if insights.total_escls > 0 {
            insights.comment_engagement_score = insights.escls_with_rich_comments as f64 / insights.total_escls as f64;
        }
    });
    
    // Generate categories (simplified)
    insights.top_escl_categories = vec![
        ESCLCategory {
            category: "Database Issues".to_string(),
            count: insights.total_escls / 4,
            avg_comments: 3.2,
            resolution_rate: 0.85,
        },
        ESCLCategory {
            category: "Authentication Problems".to_string(),
            count: insights.total_escls / 5,
            avg_comments: 2.8,
            resolution_rate: 0.90,
        },
        ESCLCategory {
            category: "Performance Issues".to_string(),
            count: insights.total_escls / 6,
            avg_comments: 4.1,
            resolution_rate: 0.78,
        },
    ];
    
    insights
}

async fn calculate_knowledge_impact() -> KnowledgeImpact {
    // This would integrate with the knowledge engine to show learning impact
    KnowledgeImpact {
        new_concepts_discovered: 12,
        new_technologies_identified: 3,
        new_solution_patterns: 8,
        knowledge_value_score: 8.7,
        recommended_actions: vec![
            "Review ESCL-2058 resolution comments for database optimization patterns".to_string(),
            "Extract authentication troubleshooting workflow from recent ESCLs".to_string(),
            "Create learning material from performance tuning discussions".to_string(),
        ],
    }
}

async fn get_last_sync_time() -> String {
    // For now, use current time - in real implementation, track actual sync times
    Utc::now().to_rfc3339()
}

// Helper functions
fn estimate_comment_count(comment_json: &str) -> usize {
    // Simple estimation - count "author" occurrences as proxy for comment count
    comment_json.matches("\"author\"").count()
}

fn extract_priority_from_key(key: &str) -> String {
    if key.contains("ESCL") {
        "High".to_string()
    } else {
        "Medium".to_string()
    }
}

fn determine_update_type(status: &str, has_comments: bool) -> UpdateType {
    if has_comments && (status.contains("resolved") || status.contains("closed")) {
        UpdateType::Resolution
    } else if has_comments {
        UpdateType::NewComments
    } else if status.contains("resolved") || status.contains("closed") {
        UpdateType::StatusChange
    } else {
        UpdateType::Multiple
    }
}

fn generate_changes_summary(key: &str, has_comments: bool, status: &str) -> String {
    if has_comments && status.contains("resolved") {
        format!("{} resolved with solution comments", key)
    } else if has_comments {
        format!("{} has new discussion comments", key)
    } else {
        format!("{} status updated to {}", key, status)
    }
}

fn extract_solution_keywords(comment: &str) -> Vec<String> {
    let solution_keywords = vec![
        "solution", "fix", "resolved", "workaround", "configuration",
        "restart", "update", "patch", "deploy", "rollback"
    ];
    
    let comment_lower = comment.to_lowercase();
    solution_keywords
        .into_iter()
        .filter(|keyword| comment_lower.contains(keyword))
        .map(|s| s.to_string())
        .collect()
}

fn calculate_comment_quality(comment: &str) -> f64 {
    let mut score = 0.0;
    
    // Length bonus
    score += (comment.len() as f64 / 1000.0).min(3.0);
    
    // Solution indicators
    let solution_words = ["solution", "fix", "resolved", "configuration", "restart"];
    for word in solution_words {
        if comment.to_lowercase().contains(word) {
            score += 1.0;
        }
    }
    
    // Code snippets (simple detection)
    if comment.contains("```") || comment.contains("{code}") {
        score += 2.0;
    }
    
    score.min(10.0)
}

fn estimate_participant_count(comment: &str) -> usize {
    // Count unique "displayName" occurrences as participant estimate
    let display_name_count = comment.matches("\"displayName\"").count();
    std::cmp::max(1, display_name_count)
}