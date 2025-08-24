use crate::advanced_analytics::generate_advanced_metrics;
use crate::analytics::{generate_analytics_dashboard, get_time_series_data};
use crate::graph::{analyze_graph_patterns, generate_knowledge_graph};
use crate::smart_graph::generate_smart_graph;
use crate::knowledge_engine::build_knowledge_base;
use crate::semantic_search::{semantic_search, SemanticSearchQuery};
use crate::unified_search::{unified_search, UnifiedSearchRequest};
use crate::sync_status::get_sync_status;
use crate::user_notes::{create_note, get_user_notes, create_saved_view, get_saved_views, update_view_usage, toggle_view_favorite, CreateNoteRequest, CreateViewRequest};
use crate::google_auth::{GoogleAuthManager, GoogleOAuthConfig, GoogleTokens};
use crate::slack_auth::{SlackAuthManager, SlackOAuthConfig, SlackTokens};
use crate::content_extractor::JobPriority;
use axum::{
    extract::{Query, Path},
    http::StatusCode,
    response::{Json},
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
        .route("/api/auth/google/status", get(google_auth_status))
        .route("/api/auth/google/refresh", post(google_refresh_tokens))
        .route("/api/auth/slack", get(slack_auth_initiate))
        .route("/api/auth/slack/callback", get(slack_auth_callback))
        .route("/api/auth/slack/status", get(slack_auth_status))
        .route("/api/auth/slack/test", post(slack_test_auth))
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
    let _search_types = params.types
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>())
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
    
    // Simplified authentication flow
    if let Err(e) = auth_manager.exchange_code_for_tokens(&code).await {
        eprintln!("❌ Google authentication failed: {}", e);
        return Ok(Json(serde_json::json!({
            "success": false,
            "error": "Authentication failed"
        })));
    }
    
    if !auth_manager.is_authenticated() {
        return Ok(Json(serde_json::json!({
            "success": false,
            "error": "Authentication completed but not verified"
        })));
    }
    
    // Store tokens if available
    if let Some(tokens) = auth_manager.get_tokens() {
        store_google_tokens(&user_id, tokens).await;
        
        println!("✅ Successfully authenticated Google for user: {} with valid tokens", user_id);
        
        Ok(Json(serde_json::json!({
            "success": true,
            "user_id": user_id,
            "authenticated": true,
            "message": "Google authentication successful",
            "has_access_token": !tokens.access_token.is_empty(),
            "has_refresh_token": tokens.refresh_token.is_some(),
            "expires_at": tokens.expires_at.to_rfc3339()
        })))
    } else {
        Ok(Json(serde_json::json!({
            "success": false,
            "error": "Failed to retrieve authentication tokens"
        })))
    }
}

/// Check Google authentication status for a user
async fn google_auth_status(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user_id = params.get("user_id").unwrap_or(&"default".to_string()).clone();
    
    // Try to load stored tokens
    if let Some(tokens) = load_google_tokens(&user_id).await {
        let config = GoogleOAuthConfig::default();
        let mut auth_manager = GoogleAuthManager::new(config);
        auth_manager.set_tokens(tokens);
        
        let is_authenticated = auth_manager.is_authenticated();
        let current_tokens = auth_manager.get_tokens();
        
        Ok(Json(serde_json::json!({
            "success": true,
            "user_id": user_id,
            "authenticated": is_authenticated,
            "has_access_token": current_tokens.map(|t| !t.access_token.is_empty()).unwrap_or(false),
            "has_refresh_token": current_tokens.map(|t| t.refresh_token.is_some()).unwrap_or(false),
            "expires_at": current_tokens.map(|t| t.expires_at.to_rfc3339()),
            "token_expired": current_tokens.map(|t| chrono::Utc::now() > t.expires_at).unwrap_or(true)
        })))
    } else {
        Ok(Json(serde_json::json!({
            "success": true,
            "user_id": user_id,
            "authenticated": false,
            "message": "No stored authentication tokens found"
        })))
    }
}

/// Refresh Google authentication tokens for a user
async fn google_refresh_tokens(
    Json(request): Json<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user_id = request.get("user_id").unwrap_or(&"default".to_string()).clone();
    
    println!("🔄 Refreshing Google tokens for user: {}", user_id);
    
    // Try to load stored tokens
    let Some(tokens) = load_google_tokens(&user_id).await else {
        return Ok(Json(serde_json::json!({
            "success": false,
            "error": "No stored authentication tokens found to refresh"
        })));
    };
    
    let config = GoogleOAuthConfig::default();
    let mut auth_manager = GoogleAuthManager::new(config);
    auth_manager.set_tokens(tokens);
    
    // Refresh tokens
    if let Err(e) = auth_manager.refresh_tokens().await {
        println!("❌ Failed to refresh Google tokens for user {}: {}", user_id, e);
        return Ok(Json(serde_json::json!({
            "success": false,
            "error": format!("Token refresh failed: {}", e)
        })));
    }
    
    // Store the refreshed tokens
    if let Some(refreshed_tokens) = auth_manager.get_tokens() {
        store_google_tokens(&user_id, refreshed_tokens).await;
        
        Ok(Json(serde_json::json!({
            "success": true,
            "user_id": user_id,
            "message": "Tokens refreshed successfully",
            "authenticated": auth_manager.is_authenticated(),
            "expires_at": refreshed_tokens.expires_at.to_rfc3339()
        })))
    } else {
        Ok(Json(serde_json::json!({
            "success": false,
            "error": "Token refresh succeeded but tokens not accessible"
        })))
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
    // Handle OAuth errors first
    if let Some(error) = params.error {
        return Ok(Json(serde_json::json!({
            "success": false,
            "error": error
        })));
    }

    // Validate required parameters
    let code = params.code.ok_or(StatusCode::BAD_REQUEST)?;
    let state = params.state.unwrap_or_default();
    
    // Extract user_id from state parameter
    let user_id = state
        .split('=')
        .nth(1)
        .unwrap_or("default")
        .to_string();

    // Create auth manager and exchange code for tokens
    let config = SlackOAuthConfig::default();
    let mut auth_manager = SlackAuthManager::new(config);
    
    // Exchange code for tokens
    if let Err(e) = auth_manager.exchange_code_for_tokens(&code).await {
        eprintln!("❌ Slack authentication failed: {}", e);
        return Ok(Json(serde_json::json!({
            "success": false,
            "error": "Authentication failed"
        })));
    }
    
    // Check if authentication was successful
    if !auth_manager.is_authenticated() {
        return Ok(Json(serde_json::json!({
            "success": false,
            "error": "Authentication completed but not verified"
        })));
    }
    
    // Get team info
    let (team_id, team_name) = auth_manager.get_team_info().unwrap_or((
        "unknown_team".to_string(), 
        "Unknown Team".to_string()
    ));
    
    // Store tokens securely
    if let Some(tokens) = auth_manager.get_tokens() {
        store_slack_tokens(&user_id, tokens).await;
    }
    
    println!("✅ Slack authentication successful for user: {} in team: {}", user_id, team_name);
    
    Ok(Json(serde_json::json!({
        "success": true,
        "user_id": user_id,
        "team_id": team_id,
        "team_name": team_name,
        "authenticated": true,
        "has_bot_token": auth_manager.get_bot_token().is_some(),
        "has_user_token": auth_manager.get_user_token().is_some(),
        "message": "Slack authentication successful"
    })))
}

// Helper function to store Google authentication tokens
async fn store_google_tokens(user_id: &str, tokens: &GoogleTokens) {
    // In a real implementation, this would store tokens in a secure database
    // For now, we'll just log the successful storage
    println!("📦 Storing Google tokens for user: {}", user_id);
    println!("   - Access token present: {}", !tokens.access_token.is_empty());
    println!("   - Refresh token present: {}", tokens.refresh_token.is_some());
    println!("   - Expires at: {}", tokens.expires_at.to_rfc3339());
    
    // TODO: Implement actual database storage using content_storage.rs
    // Example:
    // crate::db_utils::with_connection("store_google_tokens", |conn| {
    //     conn.execute(INSERT_USER_AUTH_TOKEN, [user_id, "google", serde_json::to_string(tokens)?])
    //         .expect("Failed to store Google tokens");
    // });
}

// Helper function to retrieve stored Google tokens
async fn load_google_tokens(user_id: &str) -> Option<GoogleTokens> {
    // In a real implementation, this would load tokens from database
    println!("📥 Loading Google tokens for user: {}", user_id);
    
    // TODO: Implement actual database retrieval
    // For now, return None (no stored tokens)
    None
}

// Function to get a valid Google access token for API calls
async fn get_valid_google_access_token(user_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Load stored tokens
    if let Some(stored_tokens) = load_google_tokens(user_id).await {
        let config = GoogleOAuthConfig::default();
        let mut auth_manager = GoogleAuthManager::new(config);
        
        // Set the stored tokens
        auth_manager.set_tokens(stored_tokens);
        
        // Get a valid access token (this will refresh if needed)
        match auth_manager.get_valid_access_token().await {
            Ok(token) => {
                println!("✅ Retrieved valid Google access token for user: {}", user_id);
                
                // If tokens were refreshed, store the new ones
                if let Some(updated_tokens) = auth_manager.get_tokens() {
                    store_google_tokens(user_id, updated_tokens).await;
                }
                
                Ok(token)
            }
            Err(e) => {
                println!("❌ Failed to get valid Google access token for user {}: {}", user_id, e);
                Err(format!("Failed to refresh Google access token: {}", e).into())
            }
        }
    } else {
        Err("No stored Google authentication tokens found for user".into())
    }
}

// Helper function to store Slack authentication tokens
async fn store_slack_tokens(user_id: &str, tokens: &SlackTokens) {
    // In a real implementation, this would store tokens in a secure database
    println!("📦 Storing Slack tokens for user: {}", user_id);
    println!("   - Access token present: {}", !tokens.access_token.is_empty());
    println!("   - User token present: {}", tokens.user_token.is_some());
    println!("   - Team: {} ({})", tokens.team_name, tokens.team_id);
    
    // TODO: Implement actual database storage
}

// Helper function to retrieve stored Slack tokens
async fn load_slack_tokens(user_id: &str) -> Option<SlackTokens> {
    // In a real implementation, this would load tokens from database
    println!("📥 Loading Slack tokens for user: {}", user_id);
    
    // TODO: Implement actual database retrieval
    None
}

/// Check Slack authentication status for a user
async fn slack_auth_status(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user_id = params.get("user_id").unwrap_or(&"default".to_string()).clone();
    
    // Try to load stored tokens
    if let Some(tokens) = load_slack_tokens(&user_id).await {
        let config = SlackOAuthConfig::default();
        let mut auth_manager = SlackAuthManager::new(config);
        auth_manager.set_tokens(tokens);
        
        let is_authenticated = auth_manager.is_authenticated();
        let (team_id, team_name) = auth_manager.get_team_info().unwrap_or((
            "unknown".to_string(), 
            "Unknown".to_string()
        ));
        
        Ok(Json(serde_json::json!({
            "success": true,
            "user_id": user_id,
            "authenticated": is_authenticated,
            "team_id": team_id,
            "team_name": team_name,
            "has_bot_token": auth_manager.get_bot_token().is_some(),
            "has_user_token": auth_manager.get_user_token().is_some()
        })))
    } else {
        Ok(Json(serde_json::json!({
            "success": true,
            "user_id": user_id,
            "authenticated": false,
            "message": "No stored Slack authentication tokens found"
        })))
    }
}

/// Test Slack authentication for a user
async fn slack_test_auth(
    Json(request): Json<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user_id = request.get("user_id").unwrap_or(&"default".to_string()).clone();
    
    println!("🧪 Testing Slack authentication for user: {}", user_id);
    
    // Try to load stored tokens
    if let Some(tokens) = load_slack_tokens(&user_id).await {
        let config = SlackOAuthConfig::default();
        let mut auth_manager = SlackAuthManager::new(config);
        auth_manager.set_tokens(tokens);
        
        match auth_manager.test_auth().await {
            Ok(auth_test) => {
                println!("✅ Slack auth test successful for user: {}", user_id);
                
                Ok(Json(serde_json::json!({
                    "success": true,
                    "user_id": user_id,
                    "auth_test": {
                        "ok": auth_test.ok,
                        "url": auth_test.url,
                        "team": auth_test.team,
                        "user": auth_test.user,
                        "team_id": auth_test.team_id,
                        "user_id": auth_test.user_id,
                        "bot_id": auth_test.bot_id
                    },
                    "message": "Slack authentication test successful"
                })))
            }
            Err(e) => {
                println!("❌ Slack auth test failed for user {}: {}", user_id, e);
                Ok(Json(serde_json::json!({
                    "success": false,
                    "error": format!("Slack auth test failed: {}", e)
                })))
            }
        }
    } else {
        Ok(Json(serde_json::json!({
            "success": false,
            "error": "No stored Slack authentication tokens found to test"
        })))
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
    println!("🔍 Google auth status at http://127.0.0.1:3001/api/auth/google/status?user_id=USER_ID");
    println!("🔄 Google token refresh at http://127.0.0.1:3001/api/auth/google/refresh");
    println!("💬 Slack OAuth available at http://127.0.0.1:3001/api/auth/slack");
    println!("🔍 Slack auth status at http://127.0.0.1:3001/api/auth/slack/status?user_id=USER_ID");
    println!("🧪 Slack auth test at http://127.0.0.1:3001/api/auth/slack/test");
    println!("📄 Content extraction available at http://127.0.0.1:3001/api/content/extract");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}