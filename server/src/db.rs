// src/db.rs (refactored)

use crate::db_utils::{with_connection, with_transaction};
use crate::queries::{
    CREATE_ISSUES_TABLE, CREATE_PROJECT_TABLE, GET_PROJECT_IDS, INSERT_ISSUE_METADATA,
    INSERT_PROJECT,
};
// Content storage tables are now initialized separately
use crate::types::{IssueFieldMetadata, Project};
use crate::utils::{json_opt_to_string, log_error, log_step, log_success};
use colored::*;

pub async fn create_project_table() {
    log_step("📦", "Initializing `projects` table...");
    with_connection("create_project_table", |conn| {
        conn.execute_batch(CREATE_PROJECT_TABLE)
            .unwrap_or_else(|_| {
                panic!(
                    "{} Failed to execute CREATE_PROJECT_TABLE",
                    log_error("create_project_table")
                )
            });
        log_success("`projects` table ready.");
    });
}

pub async fn create_issues_table() {
    log_step("🛠️", "Initializing `issues` table...");
    with_connection("create_issues_table", |conn| {
        conn.execute_batch(CREATE_ISSUES_TABLE).unwrap_or_else(|_| {
            panic!(
                "{} Failed to execute CREATE_ISSUES_TABLE",
                log_error("create_issues_table")
            )
        });
        log_success("`issues` table ready.");
    });
}

pub async fn save_projects_to_duckdb(projects: &Vec<Project>) {
    log_step(
        "📥",
        &format!("Saving {} project(s) to DuckDB...", projects.len()),
    );
    with_connection("save_projects", |conn| {
        conn.execute_batch(CREATE_PROJECT_TABLE)
            .unwrap_or_else(|_| {
                panic!(
                    "{} Ensure `projects` table exists",
                    log_error("save_projects")
                )
            });

        with_transaction("save_projects", |tx| {
            let mut stmt = tx
                .prepare(INSERT_PROJECT)
                .unwrap_or_else(|_| panic!("{} Prepare insert", log_error("save_projects")));

            for project in projects {
                stmt.execute([&project.id, &project.key, &project.name])
                    .unwrap_or_else(|_| panic!("{} Insert project", log_error("save_projects")));
                println!(
                    "{}",
                    format!("➡ Inserted project: [{}] {}", project.key, project.name).blue()
                );
            }
        });

        log_success("All projects saved.");
    });
}

pub async fn get_stored_project_ids() -> Vec<String> {
    log_step("📤", "Retrieving stored project IDs...");
    let mut ids = Vec::new();

    with_connection("get_stored_project_ids", |conn| {
        let mut stmt = conn
            .prepare(GET_PROJECT_IDS)
            .unwrap_or_else(|_| panic!("{} Prepare select", log_error("get_stored_project_ids")));

        let rows = stmt
            .query_map([], |row| row.get(0))
            .unwrap_or_else(|_| panic!("{} Read rows", log_error("get_stored_project_ids")));

        ids = rows.map(|r| r.unwrap()).collect();
        log_success(&format!("Retrieved {} stored project ID(s).", ids.len()));
    });

    ids
}

pub async fn save_issues_batch_to_duckdb(issues: &[IssueFieldMetadata]) {
    log_step(
        "📥",
        &format!("Inserting {} issues into DuckDB...", issues.len()),
    );

    with_transaction("save_issues_batch", |tx| {
        let mut stmt = tx
            .prepare(INSERT_ISSUE_METADATA)
            .unwrap_or_else(|_| panic!("{} Prepare insert", log_error("save_issues_batch")));

        for issue in issues {
            let summary = issue.summary.as_deref().unwrap_or("");
            let created = issue.created.as_deref().unwrap_or("");
            let updated = issue.updated.as_deref().unwrap_or("");
            let rendered_fields = issue.rendered_fields.as_deref().unwrap_or("");
            let names = issue.names.as_deref().unwrap_or("");
            let schema = issue.schema.as_deref().unwrap_or("");
            let transitions = issue.transitions.as_deref().unwrap_or("");
            let edit_meta = issue.edit_meta.as_deref().unwrap_or("");
            let changelog = issue.changelog.as_deref().unwrap_or("");
            let versioned_representations =
                issue.versioned_representations.as_deref().unwrap_or("");

            // New fields
            let issue_type = issue.issue_type.as_deref().unwrap_or("");
            let issue_type_id = issue.issue_type_id.as_deref().unwrap_or("");
            let is_subtask = issue.is_subtask.unwrap_or(false);
            let hierarchy_level = issue.hierarchy_level.unwrap_or(0);
            let priority = issue.priority.as_deref().unwrap_or("");
            let priority_id = issue.priority_id.as_deref().unwrap_or("");
            let assignee = issue.assignee.as_deref().unwrap_or("");
            let reporter = issue.reporter.as_deref().unwrap_or("");
            let project_name = issue.project_name.as_deref().unwrap_or("");
            let project_key = issue.project_key.as_deref().unwrap_or("");
            
            // Serialize complex fields to JSON
            let labels = issue.labels.as_ref()
                .map(|l| serde_json::to_string(l).unwrap_or_default())
                .unwrap_or_default();
            let extracted_links = issue.extracted_links.as_ref()
                .map(|l| serde_json::to_string(l).unwrap_or_default())
                .unwrap_or_default();

            let watcher = json_opt_to_string(&issue.watcher);
            let attachment = json_opt_to_string(&issue.attachment);
            let sub_tasks = json_opt_to_string(&issue.sub_tasks);
            let description = json_opt_to_string(&issue.description);
            let project = json_opt_to_string(&issue.project);
            let comment = json_opt_to_string(&issue.comment);
            let issue_links = json_opt_to_string(&issue.issue_links);
            let work_log = json_opt_to_string(&issue.work_log);
            let time_tracking = json_opt_to_string(&issue.time_tracking);

            stmt.execute(duckdb::params![
                issue.id,
                issue.key,
                issue.self_link,
                summary,
                issue.status,
                issue_type,
                issue_type_id,
                is_subtask.to_string(),
                hierarchy_level.to_string(),
                priority,
                priority_id,
                assignee,
                reporter,
                labels,
                created,
                updated,
                project_name,
                project_key,
                extracted_links,
                rendered_fields,
                names,
                schema,
                transitions,
                edit_meta,
                changelog,
                versioned_representations,
                watcher,
                attachment,
                sub_tasks,
                description,
                project,
                comment,
                issue_links,
                work_log,
                time_tracking,
            ])
            .unwrap_or_else(|_| panic!("{} Insert issue", log_error("save_issues_batch")));
        }

        println!(
            "{}",
            format!("✅ Successfully inserted {} issues.", issues.len()).green()
        );
    });

    log_success("Batch commit complete.");
}

// Content storage table creation is handled in content_storage.rs module
