use crate::auth::{authenticate, get_domain};
use crate::constants::{BATCH_SIZE, PAGE_SIZE};
use crate::db::save_issues_batch_to_duckdb;
use crate::link_detector::LinkDetector;
use crate::routes::{get_issue_object, get_projects_api_route, search_issues_for_project_all_types};
use crate::types::{Issue, IssueFieldMetadata, Project};
use crate::utils::{extract_json_field_as_string, get_optional_field};
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::sync::Arc;
use tokio::{
    sync::{mpsc, Semaphore},
    time::Duration,
};

struct SyncContext {
    token: String,
    domain: String,
    client: Client,
}

pub async fn fetch_projects() -> Vec<Project> {
    println!("📡 Fetching projects from Jira...");

    let domain = get_domain();
    let token = authenticate().await;
    let url = get_projects_api_route(domain.clone());

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

    let (tx, mut rx) = mpsc::channel::<IssueFieldMetadata>(BATCH_SIZE * 10);

    // Spawn one global DB writer task
    let writer = tokio::spawn(async move {
        let mut buffer = Vec::with_capacity(BATCH_SIZE);
        while let Some(meta) = rx.recv().await {
            buffer.push(meta);
            if buffer.len() >= BATCH_SIZE {
                save_issues_batch_to_duckdb(&buffer).await;
                buffer.clear();
            }
        }
        if !buffer.is_empty() {
            save_issues_batch_to_duckdb(&buffer).await;
        }
    });

    let mut tasks = FuturesUnordered::new();

    for project_id in selected_ids {
        println!("➡️  Syncing issues for project [{}]...", project_id);
        let project_id = project_id.clone();
        let tx = tx.clone();
        tasks.push(tokio::spawn(async move {
            fetch_issues_for_project(&project_id, tx).await;
        }));
    }

    while let Some(_) = tasks.next().await {}

    drop(tx); // Close the global sender
    writer.await.unwrap();

    println!("✅ All project issues synced.");
}

pub async fn fetch_issues_for_project(project_id: &str, tx: mpsc::Sender<IssueFieldMetadata>) {
    let ctx = Arc::new(SyncContext {
        token: authenticate().await,
        domain: get_domain(),
        client: Client::new(),
    });

    let mut start_at = 0;

    loop {
        println!(
            "📥 Fetching issues (start_at = {}) for project [{}]...",
            start_at, project_id
        );

        let url = search_issues_for_project_all_types(&ctx.domain, project_id, start_at);

        let res = ctx
            .client
            .get(&url)
            .header("Authorization", format!("Basic {}", ctx.token))
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

        get_issue_metaobjects(ctx.clone(), parsed, tx.clone()).await;

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

async fn get_issue_metaobjects(
    ctx: Arc<SyncContext>,
    issues: Vec<Issue>,
    tx: mpsc::Sender<IssueFieldMetadata>,
) {
    let semaphore = Arc::new(Semaphore::new(std::cmp::min(100, num_cpus::get() * 10)));
    let mut tasks = FuturesUnordered::new();
    let mut success_count = 0;
    let mut fail_count = 0;

    for issue in issues {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let ctx = ctx.clone();
        let tx = tx.clone();

        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            match fetch_metadata_with_retry(&ctx, issue).await {
                Ok(metadata) => {
                    let _ = tx.send(metadata).await;
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }));
    }

    drop(tx); // Close sender after all tasks dispatched

    while let Some(result) = tasks.next().await {
        match result {
            Ok(Ok(())) => success_count += 1,
            Ok(Err(e)) => {
                eprintln!("❌ Skipped issue due to error: {}", e);
                fail_count += 1;
            }
            Err(e) => {
                eprintln!("❌ Tokio join error: {}", e);
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

async fn fetch_metadata_with_retry(
    ctx: &SyncContext,
    issue: Issue,
) -> Result<IssueFieldMetadata, String> {
    const MAX_RETRIES: u32 = 5;
    const BASE_DELAY_SECS: u64 = 2;

    let mut attempts = 0;

    loop {
        match fetch_metadata_with_headers(ctx, &issue).await {
            Ok(meta) => return Ok(meta),
            Err((status, _body, headers))
                if status == StatusCode::TOO_MANY_REQUESTS && attempts < MAX_RETRIES =>
            {
                attempts += 1;

                let retry_after_secs = headers
                    .get("Retry-After")
                    .and_then(|val| val.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok());

                let jitter = rand::random::<u64>() % 3; // add up to 2 seconds jitter
                let backoff =
                    retry_after_secs.unwrap_or(BASE_DELAY_SECS << (attempts - 1)) + jitter;

                eprintln!(
                    "⏳ Rate-limited on issue [{}], retrying in {}s (attempt {}/{})...",
                    issue.id, backoff, attempts, MAX_RETRIES
                );

                tokio::time::sleep(Duration::from_secs(backoff)).await;
            }
            Err((status, body, _)) => {
                return Err(format!(
                    "❌ API error for issue [{}]: {} ({})",
                    issue.id, status, body
                ));
            }
        }
    }
}

async fn fetch_metadata_with_headers(
    ctx: &SyncContext,
    issue: &Issue,
) -> Result<IssueFieldMetadata, (StatusCode, String, reqwest::header::HeaderMap)> {
    let url = format!("{}?expand=renderedFields,names,schema,editmeta,changelog,versionedRepresentations", 
                     get_issue_object(&ctx.domain, &issue.id));

    let res = ctx
        .client
        .get(&url)
        .header("Authorization", format!("Basic {}", ctx.token))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
                reqwest::header::HeaderMap::new(),
            )
        })?;

    let status = res.status();
    let headers = res.headers().clone();
    let body = res.text().await.unwrap_or_default();

    if status != StatusCode::OK {
        return Err((status, body, headers));
    }

    let body_json: Value = serde_json::from_str(&body)
        .map_err(|e| (status, format!("JSON parse error: {}", e), headers.clone()))?;

    let fields = body_json["fields"].as_object().ok_or((
        status,
        format!("Missing 'fields' in response: {}", body),
        headers.clone(),
    ))?;

    // Initialize link detector
    let link_detector = LinkDetector::new();
    
    // Extract comprehensive issue information
    let issue_type = fields.get("issuetype")
        .and_then(|it| it.get("name"))
        .and_then(|n| n.as_str())
        .map(|s| s.to_string());
        
    let issue_type_id = fields.get("issuetype")
        .and_then(|it| it.get("id"))
        .and_then(|n| n.as_str())
        .map(|s| s.to_string());
        
    let is_subtask = fields.get("issuetype")
        .and_then(|it| it.get("subtask"))
        .and_then(|st| st.as_bool());
        
    let hierarchy_level = fields.get("issuetype")
        .and_then(|it| it.get("hierarchyLevel"))
        .and_then(|hl| hl.as_i64())
        .map(|l| l as i32);
        
    let priority = fields.get("priority")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .map(|s| s.to_string());
        
    let priority_id = fields.get("priority")
        .and_then(|p| p.get("id"))
        .and_then(|n| n.as_str())
        .map(|s| s.to_string());
        
    let assignee = fields.get("assignee")
        .and_then(|a| a.get("displayName"))
        .and_then(|n| n.as_str())
        .map(|s| s.to_string());
        
    let reporter = fields.get("reporter")
        .and_then(|r| r.get("displayName"))
        .and_then(|n| n.as_str())
        .map(|s| s.to_string());
        
    let labels = fields.get("labels")
        .and_then(|l| l.as_array())
        .map(|arr| arr.iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect::<Vec<String>>());
            
    let project_name = fields.get("project")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .map(|s| s.to_string());
        
    let project_key = fields.get("project")
        .and_then(|p| p.get("key"))
        .and_then(|n| n.as_str())
        .map(|s| s.to_string());

    // Extract links from issue content
    let issue_key = body_json["key"].as_str().unwrap_or("UNKNOWN");
    let extracted_links = link_detector.extract_links_from_issue(&body_json["fields"], issue_key);

    Ok(IssueFieldMetadata {
        id: body_json["id"].as_str().unwrap_or("").trim_matches('"').to_string(),
        key: body_json["key"].as_str().unwrap_or("").trim_matches('"').to_string(),
        self_link: body_json["self"].as_str().unwrap_or("").trim_matches('"').to_string(),
        summary: issue.fields.summary.clone(),
        status: issue
            .fields
            .status
            .as_ref()
            .map(|s| s.name.as_str())
            .unwrap_or("")
            .to_string(),
        issue_type,
        issue_type_id,
        is_subtask,
        hierarchy_level,
        priority,
        priority_id,
        assignee,
        reporter,
        labels,
        created: issue.fields.created.clone(),
        updated: issue.fields.updated.clone(),
        project_name,
        project_key,
        extracted_links: if extracted_links.is_empty() { None } else { Some(extracted_links) },
        rendered_fields: Some(extract_json_field_as_string(&body_json, "renderedFields")),
        names: Some(extract_json_field_as_string(&body_json, "names")),
        schema: Some(extract_json_field_as_string(&body_json, "schema")),
        transitions: Some(extract_json_field_as_string(&body_json, "transitions")),
        edit_meta: Some(extract_json_field_as_string(&body_json, "editMeta")),
        changelog: Some(extract_json_field_as_string(&body_json, "changelog")),
        versioned_representations: Some(extract_json_field_as_string(
            &body_json,
            "versionedRepresentations",
        )),
        watcher: get_optional_field(fields, "watcher"),
        attachment: get_optional_field(fields, "attachment"),
        sub_tasks: get_optional_field(fields, "sub-tasks"),
        description: get_optional_field(fields, "description"),
        project: get_optional_field(fields, "project"),
        comment: get_optional_field(fields, "comment"),
        issue_links: get_optional_field(fields, "issuelinks"),
        work_log: get_optional_field(fields, "worklog"),
        time_tracking: get_optional_field(fields, "timetracking"),
    })
}
