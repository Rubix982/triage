use crate::types::{ExtractedLink, PlatformType};
use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;

pub struct LinkDetector {
    url_regex: Regex,
    google_docs_regex: Regex,
    google_sheets_regex: Regex,
    google_slides_regex: Regex,
    slack_regex: Regex,
    confluence_regex: Regex,
    github_pr_regex: Regex,
    github_issue_regex: Regex,
    github_commit_regex: Regex,
}

impl LinkDetector {
    pub fn new() -> Self {
        Self {
            url_regex: Regex::new(r#"https?://[^\s<>"]{2,}"#).unwrap(),
            google_docs_regex: Regex::new(r"https://docs\.google\.com/document/d/([a-zA-Z0-9-_]+)").unwrap(),
            google_sheets_regex: Regex::new(r"https://docs\.google\.com/spreadsheets/d/([a-zA-Z0-9-_]+)").unwrap(),
            google_slides_regex: Regex::new(r"https://docs\.google\.com/presentation/d/([a-zA-Z0-9-_]+)").unwrap(),
            slack_regex: Regex::new(r"https://([^.]+)\.slack\.com/archives/([^/]+)/p(\d+)(?:\?thread_ts=(\d+\.\d+))?").unwrap(),
            confluence_regex: Regex::new(r"https://([^.]+)\.atlassian\.net/wiki/spaces/([^/]+)/pages/(\d+)").unwrap(),
            github_pr_regex: Regex::new(r"https://github\.com/([^/]+)/([^/]+)/pull/(\d+)").unwrap(),
            github_issue_regex: Regex::new(r"https://github\.com/([^/]+)/([^/]+)/issues/(\d+)").unwrap(),
            github_commit_regex: Regex::new(r"https://github\.com/([^/]+)/([^/]+)/commit/([a-f0-9]{7,40})").unwrap(),
        }
    }

    pub fn extract_links_from_issue(&self, issue_data: &Value, context: &str) -> Vec<ExtractedLink> {
        let mut links = Vec::new();
        let mut found_urls = HashSet::new();

        // Extract from description
        if let Some(description) = issue_data.get("description") {
            let desc_text = self.extract_text_from_field(description);
            self.find_links_in_text(&desc_text, &format!("{}.description", context), &mut links, &mut found_urls);
        }

        // Extract from comments
        if let Some(comments) = issue_data.get("comment").and_then(|c| c.get("comments")).and_then(|c| c.as_array()) {
            for (idx, comment) in comments.iter().enumerate() {
                if let Some(body) = comment.get("body") {
                    let comment_text = self.extract_text_from_field(body);
                    self.find_links_in_text(&comment_text, &format!("{}.comment.{}", context, idx), &mut links, &mut found_urls);
                }
            }
        }

        // Extract from summary
        if let Some(summary) = issue_data.get("summary").and_then(|s| s.as_str()) {
            self.find_links_in_text(summary, &format!("{}.summary", context), &mut links, &mut found_urls);
        }

        // Extract from custom fields (like acceptance criteria, etc.)
        if let Some(fields) = issue_data.as_object() {
            for (field_name, field_value) in fields {
                if field_name != "description" && field_name != "comment" && field_name != "summary" {
                    let field_text = self.extract_text_from_field(field_value);
                    if !field_text.is_empty() {
                        self.find_links_in_text(&field_text, &format!("{}.{}", context, field_name), &mut links, &mut found_urls);
                    }
                }
            }
        }

        links
    }

    fn extract_text_from_field(&self, field: &Value) -> String {
        match field {
            Value::String(s) => s.clone(),
            Value::Object(obj) => {
                // Handle ADF (Atlassian Document Format) or similar structured text
                if let Some(content) = obj.get("content") {
                    self.extract_text_from_adf(content)
                } else {
                    // Try to extract any string values from the object
                    serde_json::to_string(obj).unwrap_or_default()
                }
            },
            Value::Array(arr) => {
                arr.iter()
                    .map(|v| self.extract_text_from_field(v))
                    .collect::<Vec<String>>()
                    .join(" ")
            },
            _ => field.to_string(),
        }
    }

    fn extract_text_from_adf(&self, content: &Value) -> String {
        match content {
            Value::Array(arr) => {
                arr.iter()
                    .map(|item| self.extract_text_from_adf_node(item))
                    .collect::<Vec<String>>()
                    .join(" ")
            },
            _ => self.extract_text_from_adf_node(content),
        }
    }

    fn extract_text_from_adf_node(&self, node: &Value) -> String {
        let mut text = String::new();
        
        if let Some(node_type) = node.get("type").and_then(|t| t.as_str()) {
            match node_type {
                "text" => {
                    if let Some(text_content) = node.get("text").and_then(|t| t.as_str()) {
                        text.push_str(text_content);
                    }
                },
                "paragraph" | "heading" | "listItem" | "codeBlock" => {
                    if let Some(content) = node.get("content") {
                        text.push_str(&self.extract_text_from_adf(content));
                    }
                },
                _ => {
                    if let Some(content) = node.get("content") {
                        text.push_str(&self.extract_text_from_adf(content));
                    }
                }
            }
        }
        
        text
    }

    fn find_links_in_text(&self, text: &str, context: &str, links: &mut Vec<ExtractedLink>, found_urls: &mut HashSet<String>) {
        for url_match in self.url_regex.find_iter(text) {
            let url = url_match.as_str().to_string();
            
            // Avoid duplicates
            if found_urls.contains(&url) {
                continue;
            }
            found_urls.insert(url.clone());

            let platform_type = self.classify_url(&url);
            
            links.push(ExtractedLink {
                url: url.clone(),
                platform_type,
                link_context: context.to_string(),
                extraction_metadata: None,
            });
        }
    }

    fn classify_url(&self, url: &str) -> PlatformType {
        // Google Docs
        if let Some(captures) = self.google_docs_regex.captures(url) {
            return PlatformType::GoogleDocs {
                document_id: captures.get(1).unwrap().as_str().to_string(),
            };
        }

        // Google Sheets
        if let Some(captures) = self.google_sheets_regex.captures(url) {
            return PlatformType::GoogleSheets {
                spreadsheet_id: captures.get(1).unwrap().as_str().to_string(),
            };
        }

        // Google Slides
        if let Some(captures) = self.google_slides_regex.captures(url) {
            return PlatformType::GoogleSlides {
                presentation_id: captures.get(1).unwrap().as_str().to_string(),
            };
        }

        // Slack
        if let Some(captures) = self.slack_regex.captures(url) {
            let workspace = captures.get(1).unwrap().as_str().to_string();
            let channel = captures.get(2).unwrap().as_str().to_string();
            let message_ts = captures.get(3).unwrap().as_str().to_string();
            
            if let Some(thread_ts) = captures.get(4) {
                return PlatformType::SlackThread {
                    workspace,
                    channel,
                    thread_ts: thread_ts.as_str().to_string(),
                };
            } else {
                return PlatformType::SlackMessage {
                    workspace,
                    channel,
                    message_ts,
                };
            }
        }

        // Confluence
        if let Some(captures) = self.confluence_regex.captures(url) {
            return PlatformType::ConfluencePage {
                space: captures.get(2).unwrap().as_str().to_string(),
                page_id: captures.get(3).unwrap().as_str().to_string(),
            };
        }

        // GitHub PR
        if let Some(captures) = self.github_pr_regex.captures(url) {
            return PlatformType::GitHubPR {
                owner: captures.get(1).unwrap().as_str().to_string(),
                repo: captures.get(2).unwrap().as_str().to_string(),
                pr_number: captures.get(3).unwrap().as_str().parse().unwrap_or(0),
            };
        }

        // GitHub Issue
        if let Some(captures) = self.github_issue_regex.captures(url) {
            return PlatformType::GitHubIssue {
                owner: captures.get(1).unwrap().as_str().to_string(),
                repo: captures.get(2).unwrap().as_str().to_string(),
                issue_number: captures.get(3).unwrap().as_str().parse().unwrap_or(0),
            };
        }

        // GitHub Commit
        if let Some(captures) = self.github_commit_regex.captures(url) {
            return PlatformType::GitHubCommit {
                owner: captures.get(1).unwrap().as_str().to_string(),
                repo: captures.get(2).unwrap().as_str().to_string(),
                commit_hash: captures.get(3).unwrap().as_str().to_string(),
            };
        }

        // Extract domain for unknown links
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(domain) = parsed_url.host_str() {
                return PlatformType::Unknown {
                    domain: domain.to_string(),
                };
            }
        }

        PlatformType::Unknown {
            domain: "unknown".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_google_docs_extraction() {
        let detector = LinkDetector::new();
        let url = "https://docs.google.com/document/d/1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms";
        let platform_type = detector.classify_url(url);
        
        match platform_type {
            PlatformType::GoogleDocs { document_id } => {
                assert_eq!(document_id, "1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms");
            },
            _ => panic!("Expected GoogleDocs platform type"),
        }
    }

    #[test]
    fn test_slack_thread_extraction() {
        let detector = LinkDetector::new();
        let url = "https://myworkspace.slack.com/archives/C1234567890/p1234567890123456?thread_ts=1234567890.123456";
        let platform_type = detector.classify_url(url);
        
        match platform_type {
            PlatformType::SlackThread { workspace, channel, thread_ts } => {
                assert_eq!(workspace, "myworkspace");
                assert_eq!(channel, "C1234567890");
                assert_eq!(thread_ts, "1234567890.123456");
            },
            _ => panic!("Expected SlackThread platform type"),
        }
    }

    #[test]
    fn test_github_pr_extraction() {
        let detector = LinkDetector::new();
        let url = "https://github.com/owner/repo/pull/123";
        let platform_type = detector.classify_url(url);
        
        match platform_type {
            PlatformType::GitHubPR { owner, repo, pr_number } => {
                assert_eq!(owner, "owner");
                assert_eq!(repo, "repo");
                assert_eq!(pr_number, 123);
            },
            _ => panic!("Expected GitHubPR platform type"),
        }
    }

    #[test]
    fn test_issue_link_extraction() {
        let detector = LinkDetector::new();
        let issue_data = json!({
            "description": {
                "type": "doc",
                "content": [
                    {
                        "type": "paragraph",
                        "content": [
                            {
                                "type": "text",
                                "text": "See the design doc: https://docs.google.com/document/d/1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms"
                            }
                        ]
                    }
                ]
            },
            "summary": "Implement new feature"
        });

        let links = detector.extract_links_from_issue(&issue_data, "TEST-123");
        assert_eq!(links.len(), 1);
        assert!(matches!(links[0].platform_type, PlatformType::GoogleDocs { .. }));
        assert_eq!(links[0].link_context, "TEST-123.description");
    }
}