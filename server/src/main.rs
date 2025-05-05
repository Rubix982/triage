use crate::auth::authenticate;
use crate::db::{
    create_issues_table, create_project_table, get_stored_project_ids, save_projects_to_duckdb,
};
use crate::jira::{fetch_projects, sync_issues_for_projects};
use crate::types::Project;
use clap::{Parser, Subcommand};
use colored::*;
use inquire::MultiSelect;

mod auth;
mod constants;
mod db;
mod jira;
mod queries;
mod types;

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
}

#[tokio::main]
async fn main() {
    initialize_duck_db().await;

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
        None => {
            println!(
                "{}",
                "Run with --help to see available commands.".bright_black()
            );
        }
    }
}

async fn initialize_duck_db() {
    create_project_table().await;
    create_issues_table().await;
}
