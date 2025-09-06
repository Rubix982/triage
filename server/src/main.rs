use crate::auth::authenticate;
use crate::db::{
    create_issues_table, create_project_table, get_stored_project_ids, save_projects_to_duckdb,
};
use crate::jira::{fetch_projects, sync_issues_for_projects};
use crate::types::Project;
use clap::{Parser, Subcommand};
use colored::*;
use inquire::MultiSelect;

mod advanced_analytics;
mod analytics;
mod auth;
mod constants;
mod content_extractor;
mod content_storage;
mod db;
mod db_utils;
mod google_auth;
mod google_client;
mod graph;
mod jira;
mod knowledge_engine;
mod link_detector;
mod queries;
mod routes;
mod semantic_search;
mod server;
mod slack_auth;
mod slack_client;
mod smart_graph;
mod sync_status;
mod types;
mod unified_search;
mod user_notes;
mod utils;

// Enhanced people and relationship tracking
mod enhanced_google_extractor;
mod enhanced_jira_extractor;
mod enhanced_slack_extractor;
mod people_graph;
mod people_integration;
mod people_routes;

// Tests
mod routes_test;

#[derive(Parser)]
#[command(name = "triage")]
#[command(version = "1.0.0")]
#[command(about = "CLI utility for Jira project metadata extraction")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with Jira
    Login,

    /// List and store Jira projects
    Projects {
        #[arg(long)]
        force: bool,
    },

    /// Start the web server for graph visualization
    Serve,
}

#[tokio::main]
async fn main() {
    create_project_table().await;
    create_issues_table().await;
    user_notes::initialize_notes_tables().await;
    content_storage::create_content_storage_tables().await;
    people_graph::initialize_people_tables().await;

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Login) => {
            authenticate().await;
            println!("{}", "✔ Authentication saved.".green());
        }
        Some(Commands::Projects { force }) => {
            let projects = fetch_projects().await;

            if !force {
                let existing_ids: Vec<String> = get_stored_project_ids().await;
                let new_projects: Vec<Project> = projects
                    .clone()
                    .into_iter()
                    .filter(|p| !existing_ids.contains(&p.id))
                    .collect();

                println!(
                    "{}",
                    format!("🔄 Found {} new projects.", new_projects.len()).blue()
                );
                save_projects_to_duckdb(&new_projects).await;
            } else {
                println!("{}", "⚠ Force syncing all projects...".yellow());
                save_projects_to_duckdb(&projects).await;
            }

            let options: Vec<String> = projects
                .iter()
                .map(|p| format!("{} ({})", p.name, p.id))
                .collect();

            let selected = MultiSelect::new("Select projects to sync:", options)
                .prompt()
                .unwrap_or_default();

            // Get back the IDs from selected strings
            let selected_ids: Vec<String> = selected
                .iter()
                .filter_map(|label| {
                    label
                        .split('(')
                        .last()
                        .and_then(|s| s.strip_suffix(')'))
                        .map(String::from)
                })
                .collect();

            sync_issues_for_projects(&selected_ids).await;
        }
        Some(Commands::Serve) => {
            if let Err(e) = server::start_server().await {
                eprintln!("❌ Server error: {}", e);
            }
        }
        None => {
            println!(
                "{}",
                "Run with --help to see available commands.".bright_black()
            );
        }
    }
}
