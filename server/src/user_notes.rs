use crate::db_utils::with_connection;
use crate::utils::{log_step, log_success};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserNote {
    pub id: String,
    pub title: String,
    pub content: String,
    pub note_type: NoteType,
    pub tags: Vec<String>,
    pub linked_items: Vec<LinkedItem>,
    pub created_at: String,
    pub updated_at: String,
    pub user_id: String,
    pub is_private: bool,
    pub metadata: NoteMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NoteType {
    PersonalNote,
    LearningNote,
    SolutionNote,
    ReferenceNote,
    QuickNote,
    ProjectNote,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkedItem {
    pub item_id: String,
    pub item_type: String,
    pub title: String,
    pub relationship: String, // "relates_to", "solution_for", "inspired_by", etc.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteMetadata {
    pub color: Option<String>,
    pub priority: i32, // 1-5 scale
    pub completion_status: CompletionStatus,
    pub estimated_time: Option<i32>, // minutes
    pub difficulty_level: String,
    pub source_context: Option<String>, // where the note was created from
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CompletionStatus {
    NotStarted,
    InProgress,
    Completed,
    OnHold,
    Archived,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SavedView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub view_type: ViewType,
    pub configuration: ViewConfiguration,
    pub created_at: String,
    pub updated_at: String,
    pub user_id: String,
    pub is_favorite: bool,
    pub usage_count: i32,
    pub last_accessed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ViewType {
    Search,
    Filter,
    Dashboard,
    Analysis,
    Learning,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ViewConfiguration {
    pub search_query: Option<String>,
    pub filters: HashMap<String, serde_json::Value>,
    pub display_options: DisplayOptions,
    pub sort_preferences: SortPreferences,
    pub custom_fields: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayOptions {
    pub layout: String, // "grid", "list", "cards", "timeline"
    pub items_per_page: i32,
    pub show_previews: bool,
    pub group_by: Option<String>,
    pub color_coding: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SortPreferences {
    pub primary_sort: String,
    pub secondary_sort: Option<String>,
    pub sort_order: String, // "asc" or "desc"
    pub custom_weights: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
    pub note_type: NoteType,
    pub tags: Vec<String>,
    pub linked_items: Vec<LinkedItem>,
    pub is_private: bool,
    pub metadata: Option<NoteMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateViewRequest {
    pub name: String,
    pub description: String,
    pub view_type: ViewType,
    pub configuration: ViewConfiguration,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotesResponse {
    pub notes: Vec<UserNote>,
    pub total_count: usize,
    pub tags: Vec<String>,
    pub note_types: HashMap<String, usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ViewsResponse {
    pub views: Vec<SavedView>,
    pub favorites: Vec<SavedView>,
    pub recent: Vec<SavedView>,
    pub total_count: usize,
}

pub async fn initialize_notes_tables() {
    log_step("🗃️", "Initializing user notes and views tables...");

    with_connection("init_notes_tables", |conn| {
        // Create user_notes table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS user_notes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                note_type TEXT NOT NULL,
                tags TEXT, -- JSON array
                linked_items TEXT, -- JSON array
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                user_id TEXT NOT NULL DEFAULT 'default_user',
                is_private BOOLEAN NOT NULL DEFAULT 0,
                metadata TEXT, -- JSON object
                full_text_search TEXT -- For full-text search
            )
            "#,
            [],
        )
        .expect("Failed to create user_notes table");

        // Create saved_views table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS saved_views (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                view_type TEXT NOT NULL,
                configuration TEXT NOT NULL, -- JSON object
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                user_id TEXT NOT NULL DEFAULT 'default_user',
                is_favorite BOOLEAN NOT NULL DEFAULT 0,
                usage_count INTEGER NOT NULL DEFAULT 0,
                last_accessed TEXT NOT NULL
            )
            "#,
            [],
        )
        .expect("Failed to create saved_views table");

        // Create indexes for performance
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_notes_user_id ON user_notes(user_id)",
            [],
        )
        .expect("Failed to create notes user_id index");

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_notes_created_at ON user_notes(created_at DESC)",
            [],
        )
        .expect("Failed to create notes created_at index");

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_views_user_id ON saved_views(user_id)",
            [],
        )
        .expect("Failed to create views user_id index");

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_views_favorite ON saved_views(is_favorite, user_id)",
            [],
        )
        .expect("Failed to create views favorite index");
    });

    log_success("User notes and views tables initialized");
}

pub async fn create_note(request: CreateNoteRequest) -> Result<UserNote, String> {
    log_step("📝", &format!("Creating note: {}", request.title));

    let note_id = format!("note_{}", uuid::Uuid::new_v4());
    let now = Utc::now().to_rfc3339();
    let user_id = "default_user"; // In a real app, this would come from authentication

    let metadata = request.metadata.unwrap_or(NoteMetadata {
        color: None,
        priority: 3,
        completion_status: CompletionStatus::NotStarted,
        estimated_time: None,
        difficulty_level: "intermediate".to_string(),
        source_context: None,
    });

    let note = UserNote {
        id: note_id.clone(),
        title: request.title.clone(),
        content: request.content.clone(),
        note_type: request.note_type,
        tags: request.tags,
        linked_items: request.linked_items,
        created_at: now.clone(),
        updated_at: now,
        user_id: user_id.to_string(),
        is_private: request.is_private,
        metadata,
    };

    with_connection("create_note", |conn| {
        let full_text_search = format!(
            "{} {} {}",
            request.title,
            request.content,
            note.tags.join(" ")
        );

        conn.execute(
            r#"
            INSERT INTO user_notes 
            (id, title, content, note_type, tags, linked_items, created_at, updated_at, user_id, is_private, metadata, full_text_search)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            "#,
            duckdb::params![
                note.id,
                note.title,
                note.content,
                serde_json::to_string(&note.note_type).unwrap(),
                serde_json::to_string(&note.tags).unwrap(),
                serde_json::to_string(&note.linked_items).unwrap(),
                note.created_at,
                note.updated_at,
                note.user_id,
                note.is_private as i32,
                serde_json::to_string(&note.metadata).unwrap(),
                full_text_search,
            ],
        ).map_err(|e| format!("Failed to insert note: {}", e));
    });

    log_success(&format!("Note created: {}", note_id));
    Ok(note)
}

pub async fn get_user_notes(user_id: Option<String>, search: Option<String>) -> NotesResponse {
    log_step("📋", "Retrieving user notes...");

    let user_id = user_id.unwrap_or_else(|| "default_user".to_string());
    let mut notes = Vec::new();
    let mut all_tags = Vec::new();
    let mut note_type_counts = HashMap::new();

    with_connection("get_notes", |conn| {
        let query = if let Some(search_term) = &search {
            r#"
            SELECT id, title, content, note_type, tags, linked_items, created_at, updated_at, user_id, is_private, metadata
            FROM user_notes 
            WHERE user_id = ?1 AND full_text_search LIKE ?2
            ORDER BY updated_at DESC
            "#
        } else {
            r#"
            SELECT id, title, content, note_type, tags, linked_items, created_at, updated_at, user_id, is_private, metadata
            FROM user_notes 
            WHERE user_id = ?1
            ORDER BY updated_at DESC
            "#
        };

        let mut stmt = conn.prepare(query).expect("Failed to prepare notes query");

        let rows = if let Some(search_term) = &search {
            let search_pattern = format!("%{}%", search_term);
            stmt.query_map([&user_id, &search_pattern], |row| {
                Ok(build_note_from_row(row)?)
            })
            .expect("Failed to execute search query")
        } else {
            stmt.query_map([&user_id], |row| {
                Ok(build_note_from_row(row)?)
            })
            .expect("Failed to execute notes query")
        };

        for row in rows {
            if let Ok(note) = row {
                // Collect tags
                all_tags.extend(note.tags.clone());

                // Count note types
                let note_type_str = format!("{:?}", note.note_type);
                *note_type_counts.entry(note_type_str).or_insert(0) += 1;

                notes.push(note);
            }
        }
    });

    // Remove duplicate tags and sort
    all_tags.sort();
    all_tags.dedup();

    log_success(&format!("Retrieved {} notes", notes.len()));

    NotesResponse {
        total_count: notes.len(),
        notes,
        tags: all_tags,
        note_types: note_type_counts,
    }
}

pub async fn create_saved_view(request: CreateViewRequest) -> Result<SavedView, String> {
    log_step("👁️", &format!("Creating saved view: {}", request.name));

    let view_id = format!("view_{}", uuid::Uuid::new_v4());
    let now = Utc::now().to_rfc3339();
    let user_id = "default_user";

    let view = SavedView {
        id: view_id.clone(),
        name: request.name.clone(),
        description: request.description,
        view_type: request.view_type,
        configuration: request.configuration,
        created_at: now.clone(),
        updated_at: now.clone(),
        user_id: user_id.to_string(),
        is_favorite: false,
        usage_count: 0,
        last_accessed: now,
    };

    with_connection("create_view", |conn| {
        conn.execute(
            r#"
            INSERT INTO saved_views 
            (id, name, description, view_type, configuration, created_at, updated_at, user_id, is_favorite, usage_count, last_accessed)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            duckdb::params![
                view.id,
                view.name,
                view.description,
                serde_json::to_string(&view.view_type).unwrap(),
                serde_json::to_string(&view.configuration).unwrap(),
                view.created_at,
                view.updated_at,
                view.user_id,
                view.is_favorite as i32,
                view.usage_count,
                view.last_accessed,
            ],
        ).map_err(|e| format!("Failed to insert view: {}", e));
    });

    log_success(&format!("View created: {}", view_id));
    Ok(view)
}

pub async fn get_saved_views(user_id: Option<String>) -> ViewsResponse {
    log_step("👁️", "Retrieving saved views...");

    let user_id = user_id.unwrap_or_else(|| "default_user".to_string());
    let mut views = Vec::new();
    let mut favorites = Vec::new();
    let mut recent = Vec::new();

    with_connection("get_views", |conn| {
        let query = r#"
        SELECT id, name, description, view_type, configuration, created_at, updated_at, user_id, is_favorite, usage_count, last_accessed
        FROM saved_views 
        WHERE user_id = ?1
        ORDER BY last_accessed DESC
        "#;

        let mut stmt = conn.prepare(query).expect("Failed to prepare views query");
        let rows = stmt
            .query_map([&user_id], |row| {
                let view = SavedView {
                    id: row.get::<_, String>(0)?,
                    name: row.get::<_, String>(1)?,
                    description: row.get::<_, String>(2).unwrap_or_default(),
                    view_type: serde_json::from_str(&row.get::<_, String>(3)?)
                        .unwrap_or(ViewType::Search),
                    configuration: serde_json::from_str(&row.get::<_, String>(4)?)
                        .unwrap_or_default(),
                    created_at: row.get::<_, String>(5)?,
                    updated_at: row.get::<_, String>(6)?,
                    user_id: row.get::<_, String>(7)?,
                    is_favorite: row.get::<_, i32>(8)? != 0,
                    usage_count: row.get::<_, i32>(9)?,
                    last_accessed: row.get::<_, String>(10)?,
                };
                Ok(view)
            })
            .expect("Failed to execute views query");

        for row in rows {
            if let Ok(view) = row {
                if view.is_favorite {
                    favorites.push(view.clone());
                }
                if recent.len() < 5 {
                    recent.push(view.clone());
                }
                views.push(view);
            }
        }
    });

    log_success(&format!("Retrieved {} views", views.len()));

    ViewsResponse {
        total_count: views.len(),
        views,
        favorites,
        recent,
    }
}

pub async fn update_view_usage(view_id: &str) -> Result<(), String> {
    let now = Utc::now().to_rfc3339();

    with_connection("update_view_usage", |conn| {
        conn.execute(
            r#"
            UPDATE saved_views 
            SET usage_count = usage_count + 1, last_accessed = ?1
            WHERE id = ?2
            "#,
            duckdb::params![now, view_id],
        )
        .map_err(|e| format!("Failed to update view usage: {}", e));
    });

    Ok(())
}

pub async fn toggle_view_favorite(view_id: &str) -> Result<bool, String> {
    let mut is_favorite = false;

    with_connection("toggle_favorite", |conn| {
        // First get current state
        let mut stmt = conn.prepare("SELECT is_favorite FROM saved_views WHERE id = ?1").expect("Failed to prepare query");
        let current_state = stmt.query_row([view_id], |row| Ok(row.get::<_, i32>(0)? != 0))
            .unwrap_or(false);

        is_favorite = !current_state;

        // Update the state
        conn.execute(
            "UPDATE saved_views SET is_favorite = ?1 WHERE id = ?2",
            duckdb::params![is_favorite as i32, view_id],
        )
        .map_err(|e| format!("Failed to update favorite status: {}", e));
    });

    Ok(is_favorite)
}

fn build_note_from_row(row: &duckdb::Row) -> Result<UserNote, duckdb::Error> {
    Ok(UserNote {
        id: row.get(0)?,
        title: row.get(1)?,
        content: row.get(2)?,
        note_type: serde_json::from_str(&row.get::<_, String>(3)?)
            .unwrap_or(NoteType::PersonalNote),
        tags: serde_json::from_str(&row.get::<_, String>(4).unwrap_or_default())
            .unwrap_or_default(),
        linked_items: serde_json::from_str(&row.get::<_, String>(5).unwrap_or_default())
            .unwrap_or_default(),
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
        user_id: row.get(8)?,
        is_private: row.get::<_, i32>(9)? != 0,
        metadata: serde_json::from_str(&row.get::<_, String>(10).unwrap_or_default()).unwrap_or(
            NoteMetadata {
                color: None,
                priority: 3,
                completion_status: CompletionStatus::NotStarted,
                estimated_time: None,
                difficulty_level: "intermediate".to_string(),
                source_context: None,
            },
        ),
    })
}

impl Default for ViewConfiguration {
    fn default() -> Self {
        Self {
            search_query: None,
            filters: HashMap::new(),
            display_options: DisplayOptions {
                layout: "grid".to_string(),
                items_per_page: 20,
                show_previews: true,
                group_by: None,
                color_coding: None,
            },
            sort_preferences: SortPreferences {
                primary_sort: "updated_at".to_string(),
                secondary_sort: None,
                sort_order: "desc".to_string(),
                custom_weights: HashMap::new(),
            },
            custom_fields: Vec::new(),
        }
    }
}
