use crate::queries::{
    CREATE_ISSUES_TABLE, CREATE_PROJECT_TABLE, GET_PROJECT_IDS, INSERT_ISSUES, INSERT_PROJECT,
};
use crate::types::{Issue, Project};
use colored::*;
use dirs::home_dir;
use duckdb::{Connection, Result as DuckResult};
use once_cell::sync::Lazy;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

static IS_CONNECTION_MSG_LOGGED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

fn get_db_path() -> PathBuf {
    let mut dir = home_dir().expect("❌ Could not find home directory");
    dir.push(".triage");
    if !dir.exists() {
        fs::create_dir_all(&dir).expect("❌ Failed to create .triage dir");
    }
    dir.push("triage.duckdb");
    dir
}

fn get_connection() -> DuckResult<Connection> {
    let db_path = get_db_path();
    let mut is_connection_msg_logged = IS_CONNECTION_MSG_LOGGED.lock().unwrap();
    if !*is_connection_msg_logged {
        println!(
            "{} {}",
            "🔌 Connecting to DuckDB at:".bright_black(),
            db_path.display()
        );
        *is_connection_msg_logged = true;
    }
    Connection::open(db_path)
}

pub async fn create_project_table() {
    println!("{}", "📦 Creating `projects` table...".yellow());
    let conn: Connection = get_connection().expect("❌ Failed to connect to DB");
    conn.execute_batch(CREATE_PROJECT_TABLE)
        .expect("❌ Failed to create `projects` table");
    println!("{}", "✅ `projects` table created.".green());
}

pub async fn create_issues_table() {
    println!("{}", "🛠️ Creating `issues` table...".yellow());
    let conn: Connection = get_connection().expect("❌ Failed to get connection");
    conn.execute_batch(CREATE_ISSUES_TABLE)
        .expect("❌ Failed to create `issues` table");
    println!("{}", "✅ `issues` table created.".green());
}

pub async fn save_projects_to_duckdb(projects: &Vec<Project>) {
    println!(
        "{}",
        format!("📥 Saving {} projects to DuckDB...", projects.len()).yellow()
    );

    let mut conn = get_connection().expect("❌ Failed to connect to DB");

    conn.execute_batch(CREATE_PROJECT_TABLE)
        .expect("❌ Failed to ensure `projects` table exists");

    let tx = conn.transaction().expect("❌ Failed to start transaction");
    {
        let mut stmt = tx
            .prepare(INSERT_PROJECT)
            .expect("❌ Failed to prepare insert");

        for project in projects {
            stmt.execute([&project.id, &project.key, &project.name])
                .expect("❌ Failed to insert project");
            println!(
                "{}",
                format!("➡ Inserted project: [{}] {}", project.key, project.name).blue()
            );
        }
    }

    tx.commit().expect("❌ Failed to commit transaction");
    println!("{}", "✅ All projects committed to DuckDB.".green());
}

pub async fn get_stored_project_ids() -> Vec<String> {
    println!(
        "{}",
        "📤 Fetching stored project IDs from DuckDB...".yellow()
    );

    let conn = get_connection().expect("❌ Failed to connect to DB");

    let mut stmt = conn
        .prepare(GET_PROJECT_IDS)
        .expect("❌ Failed to prepare select");

    let rows = stmt
        .query_map([], |row| row.get(0))
        .expect("❌ Failed to read rows");

    let ids: Vec<String> = rows.map(|r| r.expect("❌ Row read fail")).collect();

    println!(
        "{}",
        format!("✅ Retrieved {} stored project IDs.", ids.len()).green()
    );
    ids
}

pub async fn save_issues_to_duckdb(issues: &[Issue]) {
    println!(
        "{}",
        format!("📥 Saving {} issues to DuckDB...", issues.len()).yellow()
    );

    let mut conn = get_connection().expect("❌ Failed to connect to DB");
    let tx = conn.transaction().expect("❌ Failed to start transaction");

    let mut stmt = tx
        .prepare(INSERT_ISSUES)
        .expect("❌ Failed to prepare insert");

    for issue in issues {
        let fields = &issue.fields;
        let description_str = fields
            .description
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or_default();

        stmt.execute([
            &issue.id,
            &issue.key,
            fields.summary.as_deref().unwrap_or(""),
            &description_str,
            &fields
                .status
                .as_ref()
                .map(|s| s.name.as_str())
                .unwrap_or(""),
            &fields.created.as_deref().unwrap_or(""),
            &fields.updated.as_deref().unwrap_or(""),
        ])
        .expect("❌ Failed to insert issue");

        println!(
            "{}",
            format!(
                "➡ Inserted issue: [{}] {}",
                issue.key,
                fields.summary.as_deref().unwrap_or("")
            )
            .blue()
        );
    }

    tx.commit().expect("❌ Failed to commit transaction");
    println!("{}", "✅ All issues committed to DuckDB.".green());
}
