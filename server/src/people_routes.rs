use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::people_integration::{
    get_person_network_insights, get_collaboration_recommendations,
    PersonNetworkProfile, CollaborationRecommendation, PeopleIntegrationSystem
};

// ================================
// API REQUEST/RESPONSE TYPES
// ================================

#[derive(Deserialize)]
pub struct PersonQuery {
    pub person_id: String,
}

#[derive(Deserialize)]
pub struct CollaborationQuery {
    pub person_id: String,
    pub topic: Option<String>,
}

#[derive(Deserialize)]
pub struct ContentAnalysisRequest {
    pub platform: String,     // "jira", "google", "slack"
    pub content_id: String,   // issue key, document ID, or thread ID
    pub channel_id: Option<String>, // for Slack threads
}

#[derive(Serialize)]
pub struct PeopleInsightsResponse {
    pub success: bool,
    pub insights: Option<PeopleNetworkInsights>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct PersonProfileResponse {
    pub success: bool,
    pub profile: Option<PersonNetworkProfile>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct CollaborationRecommendationsResponse {
    pub success: bool,
    pub recommendations: Vec<CollaborationRecommendation>,
    pub error: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct PeopleNetworkInsights {
    pub platform: String,
    pub content_id: String,
    pub participants: Vec<String>,
    pub collaboration_patterns: usize,
    pub knowledge_transfers: usize,
    pub engagement_score: f64,
}

// ================================
// ROUTE HANDLERS
// ================================

/// Analyze content for people insights
pub async fn analyze_content(
    Json(request): Json<ContentAnalysisRequest>,
) -> Result<Json<PeopleInsightsResponse>, StatusCode> {
    let mut integration_system = PeopleIntegrationSystem::new();
    
    if let Err(e) = integration_system.initialize().await {
        return Ok(Json(PeopleInsightsResponse {
            success: false,
            insights: None,
            error: Some(format!("Failed to initialize system: {}", e)),
        }));
    }

    let result = match request.platform.as_str() {
        "jira" => {
            match integration_system.process_jira_issue(&request.content_id).await {
                Ok(insights) => Ok(insights),
                Err(e) => Err(e),
            }
        },
        "google" => {
            match integration_system.process_google_document(&request.content_id).await {
                Ok(insights) => Ok(insights),
                Err(e) => Err(e),
            }
        },
        "slack" => {
            if let Some(channel_id) = &request.channel_id {
                match integration_system.process_slack_thread(channel_id, &request.content_id).await {
                    Ok(insights) => Ok(insights),
                    Err(e) => Err(e),
                }
            } else {
                Err("Channel ID required for Slack threads".into())
            }
        },
        _ => Err(format!("Unsupported platform: {}", request.platform).into()),
    };

    match result {
        Ok(integration_insights) => {
            let insights = PeopleNetworkInsights {
                platform: integration_insights.platform,
                content_id: integration_insights.content_id,
                participants: integration_insights.participants,
                collaboration_patterns: integration_insights.collaboration_patterns,
                knowledge_transfers: integration_insights.knowledge_transfers,
                engagement_score: integration_insights.engagement_score,
            };
            
            Ok(Json(PeopleInsightsResponse {
                success: true,
                insights: Some(insights),
                error: None,
            }))
        },
        Err(e) => {
            Ok(Json(PeopleInsightsResponse {
                success: false,
                insights: None,
                error: Some(e.to_string()),
            }))
        }
    }
}

/// Get comprehensive profile for a person
pub async fn get_person_profile(
    Path(person_id): Path<String>,
) -> Result<Json<PersonProfileResponse>, StatusCode> {
    match get_person_network_insights(&person_id).await {
        Ok(profile) => Ok(Json(PersonProfileResponse {
            success: true,
            profile: Some(profile),
            error: None,
        })),
        Err(e) => Ok(Json(PersonProfileResponse {
            success: false,
            profile: None,
            error: Some(e.to_string()),
        })),
    }
}

/// Get collaboration recommendations for a person
pub async fn get_person_recommendations(
    Path(person_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<CollaborationRecommendationsResponse>, StatusCode> {
    let topic = params.get("topic").unwrap_or(&"".to_string()).clone();
    
    match get_collaboration_recommendations(&person_id, &topic).await {
        Ok(recommendations) => Ok(Json(CollaborationRecommendationsResponse {
            success: true,
            recommendations,
            error: None,
        })),
        Err(e) => Ok(Json(CollaborationRecommendationsResponse {
            success: false,
            recommendations: Vec::new(),
            error: Some(e.to_string()),
        })),
    }
}

/// Get network statistics and overview
pub async fn get_network_overview() -> Result<Json<NetworkOverviewResponse>, StatusCode> {
    // This would query the database for comprehensive network statistics
    let overview = NetworkOverviewResponse {
        total_people: 0,
        total_interactions: 0,
        platforms_active: vec!["jira".to_string(), "google".to_string(), "slack".to_string()],
        top_collaborators: Vec::new(),
        recent_knowledge_transfers: Vec::new(),
    };

    Ok(Json(overview))
}

#[derive(Serialize)]
pub struct NetworkOverviewResponse {
    pub total_people: usize,
    pub total_interactions: usize,
    pub platforms_active: Vec<String>,
    pub top_collaborators: Vec<TopCollaborator>,
    pub recent_knowledge_transfers: Vec<RecentTransfer>,
}

#[derive(Serialize)]
pub struct TopCollaborator {
    pub person_id: String,
    pub display_name: String,
    pub collaboration_count: usize,
    pub influence_score: f64,
}

#[derive(Serialize)]
pub struct RecentTransfer {
    pub teacher_id: String,
    pub learner_id: String,
    pub topic: String,
    pub platform: String,
    pub timestamp: String,
}

// ================================
// ROUTER SETUP
// ================================

pub fn create_people_routes() -> Router {
    Router::new()
        .route("/people/analyze", axum::routing::post(analyze_content))
        .route("/people/profile/:person_id", axum::routing::get(get_person_profile))
        .route("/people/recommendations/:person_id", axum::routing::get(get_person_recommendations))
        .route("/people/overview", axum::routing::get(get_network_overview))
}