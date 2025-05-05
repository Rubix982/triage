use crate::auth::{authenticate, get_domain};
use crate::constants::PAGE_SIZE;
use crate::db::save_issues_to_duckdb;
use crate::types::{Issue, Project};
use reqwest::Client;
use serde_json::Value;

pub async fn fetch_projects() -> Vec<Project> {
    println!("📡 Fetching projects from Jira...");

    let domain = get_domain();
    let token = authenticate().await;
    let url = format!("https://{}/rest/api/3/project", domain);

    let client = Client::new();
    let res = client
        .get(&url)
        .header("Authorization", format!("Basic {}", token))
        .header("Accept", "application/json")
        .send()
        .await
        .expect("❌ Failed to fetch projects");

    let status = res.status();
    let body = res.text().await.expect("❌ Failed to read response body");

    if !status.is_success() {
        panic!("❌ Jira API error: {} - {}", status, body);
    }

    println!("✅ Projects fetched successfully.");
    serde_json::from_str::<Vec<Project>>(&body).expect("❌ Failed to parse response")
}

pub async fn sync_issues_for_projects(selected_ids: &Vec<String>) {
    println!("🔄 Starting issue sync for {} project(s)...", selected_ids.len());

    for project_id in selected_ids {
        println!("➡️ Syncing issues for project: {}", project_id);
        fetch_issues_for_project(&project_id).await;
    }

    println!("✅ Issue sync complete.");
}

pub async fn fetch_issues_for_project(project_id: &str) {
    let token = authenticate().await;
    let domain = get_domain();
    let client = Client::new();

    let mut start_at = 0;
    let mut issue_count = 0;

    loop {
        println!("📥 Fetching issues from {} for project {}...", start_at, project_id);

        let url = format!(
            "https://{}/rest/api/3/search?jql=project={}&startAt={}&maxResults={}",
            domain, project_id, start_at, PAGE_SIZE
        );

        let res = client
            .get(&url)
            .header("Authorization", format!("Basic {}", token))
            .header("Accept", "application/json")
            .send()
            .await
            .expect("❌ Failed to fetch issues");

        if !res.status().is_success() {
            panic!("❌ Failed to fetch issues: {}", res.text().await.unwrap_or_default());
        }

        let body: Value = res
            .json()
            .await
            .expect("❌ Failed to parse issue search response");

        let issues = body["issues"].as_array().expect("❌ Missing issues array");

        if issues.is_empty() {
            println!("🚫 No more issues found for project {}", project_id);
            break;
        }

        let parsed: Vec<Issue> = issues
            .iter()
            .map(|json| serde_json::from_value(json.clone()).unwrap())
            .collect();

        issue_count += parsed.len();
        save_issues_to_duckdb(&parsed).await;

        println!("📦 Saved {} issues so far for project {}", issue_count, project_id);

        start_at += PAGE_SIZE;
        let total = body["total"].as_u64().unwrap_or(0);
        if start_at as u64 >= total {
            println!("🏁 Finished syncing all {} issues for project {}", total, project_id);
            break;
        }
    }
}
