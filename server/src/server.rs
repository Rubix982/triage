use crate::advanced_analytics::generate_advanced_metrics;
use crate::analytics::{generate_analytics_dashboard, get_time_series_data};
use crate::graph::{analyze_graph_patterns, generate_knowledge_graph};
use crate::smart_graph::generate_smart_graph;
use crate::knowledge_engine::build_knowledge_base;
use crate::semantic_search::{semantic_search, SemanticSearchQuery};
use crate::unified_search::{unified_search, UnifiedSearchRequest};
use crate::sync_status::get_sync_status;
use crate::user_notes::{create_note, get_user_notes, create_saved_view, get_saved_views, update_view_usage, toggle_view_favorite, CreateNoteRequest, CreateViewRequest};
use crate::google_auth::{GoogleAuthManager, GoogleOAuthConfig};
use crate::slack_auth::{SlackAuthManager, SlackOAuthConfig};
use crate::content_extractor::{ContentExtractionService, create_extraction_jobs_from_links, JobPriority};
use axum::{
    extract::{Query, Path},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct GraphQuery {
    limit: Option<usize>,
    node_types: Option<String>,
}

#[derive(Deserialize)]
struct TimeSeriesQuery {
    metric: String,
    period: String,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    types: Option<String>,
    threshold: Option<f64>,
    limit: Option<usize>,
    include_related: Option<bool>,
}

#[derive(Deserialize)]
struct NotesQuery {
    search: Option<String>,
    user_id: Option<String>,
}

#[derive(Deserialize)]
struct GoogleAuthQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct SlackAuthQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct ContentExtractionQuery {
    ticket_id: String,
    user_id: String,
    priority: Option<String>,
}

pub async fn create_router() -> Router {
    Router::new()
        .route("/api/graph", get(get_knowledge_graph))
        .route("/api/graph/analysis", get(get_graph_analysis))
        .route("/api/graph/smart", get(get_smart_graph))
        .route("/api/analytics", get(get_analytics_dashboard))
        .route("/api/analytics/advanced", get(get_advanced_analytics))
        .route("/api/analytics/timeseries", get(get_time_series))
        .route("/api/knowledge", get(get_knowledge_base))
        .route("/api/search", get(semantic_search_endpoint))
        .route("/api/search/unified", post(unified_search_endpoint))
        .route("/api/notes", get(get_notes_endpoint))
        .route("/api/notes", post(create_note_endpoint))
        .route("/api/views", get(get_views_endpoint))
        .route("/api/views", post(create_view_endpoint))
        .route("/api/views/:id/use", put(use_view_endpoint))
        .route("/api/views/:id/favorite", put(toggle_favorite_endpoint))
        .route("/api/sync/status", get(get_sync_status_endpoint))
        .route("/api/auth/google", get(google_auth_initiate))
        .route("/api/auth/google/callback", get(google_auth_callback))
        .route("/api/auth/slack", get(slack_auth_initiate))
        .route("/api/auth/slack/callback", get(slack_auth_callback))
        .route("/api/content/extract", post(trigger_content_extraction))
        .route("/api/content/status", get(get_extraction_status))
        .merge(crate::people_routes::create_people_routes())
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
}

async fn health_check() -> &'static str {
    "OK"
}

async fn get_knowledge_graph(
    Query(params): Query<GraphQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match generate_knowledge_graph().await {
        graph => {
            // Apply optional filtering
            let mut filtered_graph = graph;
            
            if let Some(limit) = params.limit {
                filtered_graph.nodes.truncate(limit);
                // Filter edges to only include those with both nodes still present
                let node_ids: std::collections::HashSet<String> = filtered_graph
                    .nodes
                    .iter()
                    .map(|n| n.id.clone())
                    .collect();
                
                filtered_graph.edges.retain(|e| {
                    node_ids.contains(&e.source) && node_ids.contains(&e.target)
                });
                
                // Update metadata
                filtered_graph.metadata.total_nodes = filtered_graph.nodes.len();
                filtered_graph.metadata.total_edges = filtered_graph.edges.len();
            }
            
            Ok(Json(serde_json::to_value(filtered_graph).unwrap()))
        }
    }
}

async fn get_graph_analysis() -> Result<Json<serde_json::Value>, StatusCode> {
    let graph = generate_knowledge_graph().await;
    let analysis = analyze_graph_patterns(&graph).await;
    Ok(Json(analysis))
}

async fn get_analytics_dashboard() -> Result<Json<serde_json::Value>, StatusCode> {
    let dashboard = generate_analytics_dashboard().await;
    Ok(Json(serde_json::to_value(dashboard).unwrap()))
}

async fn get_time_series(
    Query(params): Query<TimeSeriesQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let data = get_time_series_data(&params.metric, &params.period).await;
    Ok(Json(serde_json::to_value(data).unwrap()))
}

async fn get_smart_graph() -> Result<Json<serde_json::Value>, StatusCode> {
    let smart_graph = generate_smart_graph().await;
    Ok(Json(serde_json::to_value(smart_graph).unwrap()))
}

async fn get_advanced_analytics() -> Result<Json<serde_json::Value>, StatusCode> {
    let advanced_metrics = generate_advanced_metrics().await;
    Ok(Json(serde_json::to_value(advanced_metrics).unwrap()))
}

async fn get_knowledge_base() -> Result<Json<serde_json::Value>, StatusCode> {
    let knowledge_base = build_knowledge_base().await;
    Ok(Json(serde_json::to_value(knowledge_base).unwrap()))
}

async fn semantic_search_endpoint(
    Query(params): Query<SearchQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let search_types = params.types
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();
    
    let query = SemanticSearchQuery {
        query: params.q,
        search_types: vec![], // Convert string types to enum later
        similarity_threshold: params.threshold.unwrap_or(0.3),
        max_results: params.limit.unwrap_or(20),
        context_filters: Vec::new(),
        include_related: params.include_related.unwrap_or(true),
    };
    
    let results = semantic_search(query).await;
    Ok(Json(serde_json::to_value(results).unwrap()))
}

async fn unified_search_endpoint(
    Json(request): Json<UnifiedSearchRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match unified_search(request).await {
        Ok(results) => Ok(Json(serde_json::to_value(results).unwrap())),
        Err(e) => {
            eprintln!("❌ Unified search failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_notes_endpoint(
    Query(params): Query<NotesQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let notes = get_user_notes(params.user_id, params.search).await;
    Ok(Json(serde_json::to_value(notes).unwrap()))
}

async fn create_note_endpoint(
    Json(request): Json<CreateNoteRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match create_note(request).await {
        Ok(note) => Ok(Json(serde_json::to_value(note).unwrap())),
        Err(e) => {
            eprintln!("Failed to create note: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_views_endpoint(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user_id = params.get("user_id").cloned();
    let views = get_saved_views(user_id).await;
    Ok(Json(serde_json::to_value(views).unwrap()))
}

async fn create_view_endpoint(
    Json(request): Json<CreateViewRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match create_saved_view(request).await {
        Ok(view) => Ok(Json(serde_json::to_value(view).unwrap())),
        Err(e) => {
            eprintln!("Failed to create view: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn use_view_endpoint(
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match update_view_usage(&id).await {
        Ok(_) => Ok(Json(serde_json::json!({"success": true}))),
        Err(e) => {
            eprintln!("Failed to update view usage: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn toggle_favorite_endpoint(
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match toggle_view_favorite(&id).await {
        Ok(is_favorite) => Ok(Json(serde_json::json!({"is_favorite": is_favorite}))),
        Err(e) => {
            eprintln!("Failed to toggle favorite: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_sync_status_endpoint() -> Result<Json<serde_json::Value>, StatusCode> {
    let status = get_sync_status().await;
    Ok(Json(serde_json::to_value(status).unwrap()))
}

async fn google_auth_initiate(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let config = GoogleOAuthConfig::default();
    let auth_manager = GoogleAuthManager::new(config);
    
    let user_id = params.get("user_id").unwrap_or(&"default".to_string()).clone();
    let state = format!("user_id={}", user_id);
    
    let auth_url = auth_manager.get_authorization_url(&state);
    
    Ok(Json(serde_json::json!({
        "auth_url": auth_url,
        "state": state
    })))
}

async fn google_auth_callback(
    Query(params): Query<GoogleAuthQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if let Some(error) = params.error {
        return Ok(Json(serde_json::json!({
            "success": false,
            "error": error
        })));
    }

    let code = params.code.ok_or(StatusCode::BAD_REQUEST)?;
    let state = params.state.unwrap_or_default();
    
    // Extract user_id from state
    let user_id = state.split('=')
        .nth(1)
        .unwrap_or("default")
        .to_string();

    let config = GoogleOAuthConfig::default();
    let mut auth_manager = GoogleAuthManager::new(config);
    
    match auth_manager.exchange_code_for_tokens(&code).await {
        Ok(_) => {
            // TODO: Store tokens securely in database associated with user_id
            println!("✅ Successfully authenticated Google for user: {}", user_id);
            
            Ok(Json(serde_json::json!({
                "success": true,
                "user_id": user_id,
                "message": "Google authentication successful"
            })))
        },
        Err(e) => {
            eprintln!("❌ Google authentication failed: {}", e);
            Ok(Json(serde_json::json!({
                "success": false,
                "error": "Authentication failed"
            })))
        }
    }
}

async fn trigger_content_extraction(
    Json(request): Json<ContentExtractionQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Get extracted links for the ticket from database
    // TODO: Create extraction jobs and add to service queue
    // TODO: This is a placeholder implementation
    
    let priority = match request.priority.as_deref() {
        Some("high") => JobPriority::High,
        Some("low") => JobPriority::Low,
        _ => JobPriority::Medium,
    };
    
    println!("🔄 Content extraction requested for ticket {} by user {} with priority {:?}", 
             request.ticket_id, request.user_id, priority);
    
    Ok(Json(serde_json::json!({
        "success": true,
        "ticket_id": request.ticket_id,
        "user_id": request.user_id,
        "priority": priority,
        "message": "Content extraction jobs queued"
    })))
}

async fn get_extraction_status(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let ticket_id = params.get("ticket_id");
    let user_id = params.get("user_id");
    
    // TODO: Get actual status from ContentExtractionService
    // This is a placeholder implementation
    
    Ok(Json(serde_json::json!({
        "ticket_id": ticket_id,
        "user_id": user_id,
        "jobs": [],
        "status": {
            "pending": 0,
            "processing": 0,
            "completed": 0,
            "failed": 0
        }
    })))
}

async fn slack_auth_initiate(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let config = SlackOAuthConfig::default();
    let auth_manager = SlackAuthManager::new(config);
    
    let user_id = params.get("user_id").unwrap_or(&"default".to_string()).clone();
    let team_id = params.get("team_id");
    let state = format!("user_id={}", user_id);
    
    let auth_url = auth_manager.get_authorization_url(&state, team_id.map(|s| s.as_str()));
    
    Ok(Json(serde_json::json!({
        "auth_url": auth_url,
        "state": state
    })))
}

async fn slack_auth_callback(
    Query(params): Query<SlackAuthQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if let Some(error) = params.error {
        return Ok(Json(serde_json::json!({
            "success": false,
            "error": error
        })));
    }

    let code = params.code.ok_or(StatusCode::BAD_REQUEST)?;
    let state = params.state.unwrap_or_default();
    
    // Extract user_id from state
    let user_id = state.split('=')
        .nth(1)
        .unwrap_or("default")
        .to_string();

    let config = SlackOAuthConfig::default();
    let mut auth_manager = SlackAuthManager::new(config);
    
    match auth_manager.exchange_code_for_tokens(&code).await {
        Ok(_) => {
            let (team_id, team_name) = auth_manager.get_team_info().unwrap_or_default();
            
            // TODO: Store tokens securely in database associated with user_id and team_id
            println!("✅ Successfully authenticated Slack for user: {} in team: {} ({})", 
                     user_id, team_name, team_id);
            
            // Test the authentication
            match auth_manager.test_auth().await {
                Ok(auth_test) => {
                    println!("🔍 Slack auth test successful: {:?}", auth_test);
                },
                Err(e) => {
                    eprintln!("⚠️ Slack auth test failed: {}", e);
                }
            }
            
            Ok(Json(serde_json::json!({
                "success": true,
                "user_id": user_id,
                "team_id": team_id,
                "team_name": team_name,
                "message": "Slack authentication successful"
            })))
        },
        Err(e) => {
            eprintln!("❌ Slack authentication failed: {}", e);
            Ok(Json(serde_json::json!({
                "success": false,
                "error": "Authentication failed"
            })))
        }
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router().await;
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
        
    println!("🚀 Server starting on http://127.0.0.1:3001");
    println!("📊 Knowledge graph available at http://127.0.0.1:3001/api/graph");
    println!("🧠 Knowledge base available at http://127.0.0.1:3001/api/knowledge");
    println!("🔍 Semantic search available at http://127.0.0.1:3001/api/search");
    println!("🌐 Unified cross-platform search at http://127.0.0.1:3001/api/search/unified");
    println!("📝 User notes available at http://127.0.0.1:3001/api/notes");
    println!("📊 Sync status available at http://127.0.0.1:3001/api/sync/status");
    println!("🔐 Google OAuth available at http://127.0.0.1:3001/api/auth/google");
    println!("💬 Slack OAuth available at http://127.0.0.1:3001/api/auth/slack");
    println!("📄 Content extraction available at http://127.0.0.1:3001/api/content/extract");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}