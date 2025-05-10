#[cfg(test)]
mod tests {

    use crate::auth::{authenticate, get_domain};
    use crate::routes::get_issue_object;
    use chrono::Local;
    use reqwest::Client;
    use serde_json::Value;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    #[tokio::test]
    async fn test_print_issue_json() {
        let issue_id: String = "ESCL-2058".to_string();
        let test_name = "test_print_issue_json";

        let token = authenticate().await;
        let domain = get_domain();
        let client = Client::new();
        let url = get_issue_object(&domain, &issue_id);

        let res = client
            .get(&url)
            .header("Authorization", format!("Basic {}", token))
            .header("Accept", "application/json")
            .send()
            .await
            .expect("❌ Request failed");

        let body: Value = res.json().await.expect("❌ Failed to parse response JSON");

        // Format the output
        let pretty_json = serde_json::to_string_pretty(&body).unwrap();

        // Get timestamp
        let timestamp = Local::now().format("%Y%m%d_%H%M%S"); // e.g. 20240510_234512

        // Compose file path
        let mut path = PathBuf::from("/Users/saif.islam/code/triage/server/misc/tests/");
        let filename = format!("{}_{}_{}.json", test_name, issue_id, timestamp);
        path.push(filename);

        // Create dir if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("❌ Could not create directory");
        }

        // Write file
        let mut file = fs::File::create(&path).expect("❌ Could not create output file");
        file.write_all(pretty_json.as_bytes())
            .expect("❌ Could not write JSON to file");

        println!("✅ JSON for [{}] written to: {}", issue_id, path.display());
    }
}
