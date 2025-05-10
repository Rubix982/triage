use crate::auth::{authenticate, get_domain};
use crate::constants::PAGE_SIZE;
use crate::db::save_issues_to_duckdb;
use crate::routes::{get_issue_object, get_projects_api_route, search_issues_for_project};
use crate::types::{Issue, IssueFieldMetadata, Project};
use crate::utils::{extract_json_field_as_string, get_optional_field};
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub async fn fetch_projects() -> Vec<Project> {
    println!("📡 Fetching projects from Jira...");

    let domain = get_domain();
    let token = authenticate().await;
    let url = get_projects_api_route(domain);

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
    serde_json::from_str::<Vec<Project>>(&body).expect("❌ Failed to parse project response")
}

pub async fn sync_issues_for_projects(selected_ids: &Vec<String>) {
    println!(
        "🔄 Starting issue sync for {} project(s)...",
        selected_ids.len()
    );

    for project_id in selected_ids {
        println!("➡️  Syncing issues for project [{}]...", project_id);
        fetch_issues_for_project(&project_id).await;
    }

    println!("✅ All project issues synced.");
}

pub async fn fetch_issues_for_project(project_id: &str) {
    let token = authenticate().await;
    let domain = get_domain();
    let client = Client::new();
    let mut start_at = 0;

    loop {
        println!(
            "📥 Fetching issues (start_at = {}) for project [{}]...",
            start_at, project_id
        );

        let url = search_issues_for_project(&domain, project_id, start_at);

        let res = client
            .get(&url)
            .header("Authorization", format!("Basic {}", token))
            .header("Accept", "application/json")
            .send()
            .await
            .expect("❌ Failed to fetch issues");

        if !res.status().is_success() {
            panic!(
                "❌ API error while fetching issues for project [{}]: {}",
                project_id,
                res.text().await.unwrap_or_default()
            );
        }

        let body: Value = res
            .json()
            .await
            .expect("❌ Failed to parse issue search response");

        let issues = body["issues"]
            .as_array()
            .expect("❌ Missing 'issues' array in response");

        if issues.is_empty() {
            println!("🚫 No more issues found for project [{}].", project_id);
            break;
        }

        let parsed: Vec<Issue> = issues
            .iter()
            .map(|json| serde_json::from_value(json.clone()).unwrap())
            .collect();

        println!(
            "🔎 Retrieved {} issue(s) for project [{}], fetching metadata...",
            parsed.len(),
            project_id
        );

        get_issue_metaobjects(parsed).await;

        start_at += PAGE_SIZE;
        let total = body["total"].as_u64().unwrap_or(0);
        if start_at as u64 >= total {
            println!(
                "🏁 Completed syncing all {} issues for project [{}]",
                total, project_id
            );
            break;
        }
    }
}

async fn get_issue_metaobjects(issues: Vec<Issue>) {
    let token = authenticate().await;
    let domain = get_domain();
    let client = Client::new();

    let semaphore = Arc::new(Semaphore::new(30));
    let mut tasks = FuturesUnordered::new();

    for issue in issues {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let token = token.clone();
        let domain = domain.clone();
        let client = client.clone();

        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            let url = get_issue_object(&domain, &issue.id);

            let res = client
                .get(&url)
                .header("Authorization", format!("Basic {}", token))
                .header("Accept", "application/json")
                .send()
                .await
                .map_err(|e| format!("❌ Network error for issue [{}]: {}", issue.id, e))?;

            if !res.status().is_success() {
                return Err(format!(
                    "❌ API error for issue [{}]: {}",
                    issue.id,
                    res.text().await.unwrap_or_default()
                ));
            }

            let body: Value = res
                .json()
                .await
                .map_err(|e| format!("❌ JSON parse error for issue [{}]: {}", issue.id, e))?;

            let fields = body["fields"].as_object().ok_or(format!(
                "❌ Missing 'fields' object in issue metadata. Body: {}",
                body
            ))?;

            let metadata = IssueFieldMetadata {
                id: body["id"].to_string(),
                key: body["key"].to_string(),
                self_link: body["self"].to_string(),
                summary: issue.fields.summary.clone(),
                status: issue
                    .fields
                    .status
                    .as_ref()
                    .map(|s| s.name.as_str())
                    .unwrap_or("")
                    .to_string(),
                created: issue.fields.created.clone(),
                updated: issue.fields.updated.clone(),
                rendered_fields: Some(extract_json_field_as_string(&body, "renderedFields")),
                names: Some(extract_json_field_as_string(&body, "names")),
                schema: Some(extract_json_field_as_string(&body, "schema")),
                transitions: Some(extract_json_field_as_string(&body, "transitions")),
                edit_meta: Some(extract_json_field_as_string(&body, "editMeta")),
                changelog: Some(extract_json_field_as_string(&body, "changelog")),
                versioned_representations: Some(extract_json_field_as_string(
                    &body,
                    "versionedRepresentations",
                )),
                watcher: get_optional_field(fields, "watcher").unwrap_or_default(),
                attachment: get_optional_field(fields, "attachment").unwrap_or_default(),
                sub_tasks: get_optional_field(fields, "sub-tasks").unwrap_or_default(),
                description: get_optional_field(fields, "description").unwrap_or_default(),
                project: get_optional_field(fields, "project").unwrap_or_default(),
                comment: get_optional_field(fields, "comment").unwrap_or_default(),
                issue_links: get_optional_field(fields, "issuelinks").unwrap_or_default(),
                work_log: get_optional_field(fields, "worklog").unwrap_or_default(),
                time_tracking: get_optional_field(fields, "timetracking").unwrap_or_default(),
            };

            Ok(metadata)
        }));
    }

    let mut success_count = 0;
    let mut fail_count = 0;

    while let Some(result) = tasks.next().await {
        match result {
            Ok(Ok(metadata)) => {
                println!(
                    "📄 Saving issue [{}] {}",
                    metadata.key,
                    metadata.summary.as_deref().unwrap_or("")
                );
                save_issues_to_duckdb(&metadata).await;
                success_count += 1;
            }
            Ok(Err(e)) => {
                eprintln!("❌ Skipped issue due to error: {}", e);
                fail_count += 1;
            }
            Err(e) => {
                eprintln!(
                    "❌ Tokio join error while processing issue: {}",
                    e.to_string()
                );
                fail_count += 1;
            }
        }
    }

    println!(
        "✅ Completed metadata sync for {} issues ({} success, {} failed).",
        success_count + fail_count,
        success_count,
        fail_count
    );
}
