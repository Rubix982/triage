use crate::google_auth::GoogleAuthManager;
use crate::types::{PlatformType, ExtractedLink};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::Client;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleDocumentContent {
    pub document_id: String,
    pub title: String,
    pub body: String,
    pub created_time: Option<String>,
    pub modified_time: Option<String>,
    pub author: Option<String>,
    pub suggestions: Vec<GoogleSuggestion>,
    pub comments: Vec<GoogleComment>,
    pub revision_history: Vec<GoogleRevision>,
    pub sharing_info: GoogleSharingInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleSuggestion {
    pub id: String,
    pub author: String,
    pub create_time: String,
    pub suggestion_type: String,
    pub text: String,
    pub state: String, // ACCEPTED, REJECTED, PENDING
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleComment {
    pub id: String,
    pub author: String,
    pub create_time: String,
    pub content: String,
    pub resolved: bool,
    pub replies: Vec<GoogleCommentReply>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleCommentReply {
    pub id: String,
    pub author: String,
    pub create_time: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleRevision {
    pub id: String,
    pub modified_time: String,
    pub last_modifying_user: String,
    pub size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleSharingInfo {
    pub shared_with_me: bool,
    pub owned_by_me: bool,
    pub permissions: Vec<GooglePermission>,
    pub sharing_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GooglePermission {
    pub role: String, // reader, writer, commenter, owner
    pub user_type: String, // user, group, domain, anyone
    pub email: Option<String>,
    pub domain: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleSpreadsheetContent {
    pub spreadsheet_id: String,
    pub title: String,
    pub sheets: Vec<GoogleSheet>,
    pub created_time: Option<String>,
    pub modified_time: Option<String>,
    pub author: Option<String>,
    pub sharing_info: GoogleSharingInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleSheet {
    pub sheet_id: i32,
    pub title: String,
    pub data: Vec<Vec<String>>,
    pub note_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GooglePresentationContent {
    pub presentation_id: String,
    pub title: String,
    pub slides: Vec<GoogleSlide>,
    pub created_time: Option<String>,
    pub modified_time: Option<String>,
    pub author: Option<String>,
    pub sharing_info: GoogleSharingInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleSlide {
    pub slide_id: String,
    pub title: String,
    pub content: String,
    pub notes: String,
}

pub struct GoogleApiClient {
    auth_manager: GoogleAuthManager,
    client: Client,
}

impl GoogleApiClient {
    pub fn new(auth_manager: GoogleAuthManager) -> Self {
        Self {
            auth_manager,
            client: Client::new(),
        }
    }

    pub async fn extract_document_content(&mut self, document_id: &str) -> Result<GoogleDocumentContent, Box<dyn std::error::Error>> {
        let access_token = self.auth_manager.get_valid_access_token().await?;
        
        // Get document content
        let doc_url = format!("https://docs.googleapis.com/v1/documents/{}", document_id);
        let doc_response = self.client
            .get(&doc_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !doc_response.status().is_success() {
            return Err(format!("Failed to fetch document: {}", doc_response.status()).into());
        }

        let doc_data: Value = doc_response.json().await?;
        
        // Extract document metadata
        let title = doc_data["title"].as_str().unwrap_or("Untitled").to_string();
        
        // Extract body content
        let body = self.extract_body_text(&doc_data["body"])?;
        
        // Get document metadata from Drive API
        let drive_url = format!("https://www.googleapis.com/drive/v3/files/{}?fields=createdTime,modifiedTime,owners,sharingUser,permissions,webViewLink", document_id);
        let drive_response = self.client
            .get(&drive_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let drive_data: Value = if drive_response.status().is_success() {
            drive_response.json().await?
        } else {
            serde_json::json!({})
        };

        let created_time = drive_data["createdTime"].as_str().map(|s| s.to_string());
        let modified_time = drive_data["modifiedTime"].as_str().map(|s| s.to_string());
        let author = drive_data["owners"].as_array()
            .and_then(|owners| owners.first())
            .and_then(|owner| owner["displayName"].as_str())
            .map(|s| s.to_string());

        // Get suggestions (requires special endpoint)
        let suggestions = self.extract_suggestions(document_id, &access_token).await?;
        
        // Get comments
        let comments = self.extract_comments(document_id, &access_token).await?;
        
        // Get revision history
        let revision_history = self.extract_revisions(document_id, &access_token).await?;
        
        // Extract sharing information
        let sharing_info = self.extract_sharing_info(&drive_data);

        Ok(GoogleDocumentContent {
            document_id: document_id.to_string(),
            title,
            body,
            created_time,
            modified_time,
            author,
            suggestions,
            comments,
            revision_history,
            sharing_info,
        })
    }

    pub async fn extract_spreadsheet_content(&mut self, spreadsheet_id: &str) -> Result<GoogleSpreadsheetContent, Box<dyn std::error::Error>> {
        let access_token = self.auth_manager.get_valid_access_token().await?;
        
        let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{}", spreadsheet_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to fetch spreadsheet: {}", response.status()).into());
        }

        let data: Value = response.json().await?;
        
        let title = data["properties"]["title"].as_str().unwrap_or("Untitled").to_string();
        
        let mut sheets = Vec::new();
        if let Some(sheets_array) = data["sheets"].as_array() {
            for sheet_data in sheets_array {
                let sheet_id = sheet_data["properties"]["sheetId"].as_i64().unwrap_or(0) as i32;
                let sheet_title = sheet_data["properties"]["title"].as_str().unwrap_or("Untitled").to_string();
                
                // Get sheet data
                let sheet_values = self.extract_sheet_values(spreadsheet_id, &sheet_title, &access_token).await?;
                
                sheets.push(GoogleSheet {
                    sheet_id,
                    title: sheet_title,
                    data: sheet_values,
                    note_count: 0, // TODO: Extract note count if needed
                });
            }
        }

        // Get metadata from Drive API
        let drive_url = format!("https://www.googleapis.com/drive/v3/files/{}?fields=createdTime,modifiedTime,owners,permissions", spreadsheet_id);
        let drive_response = self.client
            .get(&drive_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let drive_data: Value = if drive_response.status().is_success() {
            drive_response.json().await?
        } else {
            serde_json::json!({})
        };

        let created_time = drive_data["createdTime"].as_str().map(|s| s.to_string());
        let modified_time = drive_data["modifiedTime"].as_str().map(|s| s.to_string());
        let author = drive_data["owners"].as_array()
            .and_then(|owners| owners.first())
            .and_then(|owner| owner["displayName"].as_str())
            .map(|s| s.to_string());

        let sharing_info = self.extract_sharing_info(&drive_data);

        Ok(GoogleSpreadsheetContent {
            spreadsheet_id: spreadsheet_id.to_string(),
            title,
            sheets,
            created_time,
            modified_time,
            author,
            sharing_info,
        })
    }

    pub async fn extract_presentation_content(&mut self, presentation_id: &str) -> Result<GooglePresentationContent, Box<dyn std::error::Error>> {
        let access_token = self.auth_manager.get_valid_access_token().await?;
        
        let url = format!("https://slides.googleapis.com/v1/presentations/{}", presentation_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to fetch presentation: {}", response.status()).into());
        }

        let data: Value = response.json().await?;
        
        let title = data["title"].as_str().unwrap_or("Untitled").to_string();
        
        let mut slides = Vec::new();
        if let Some(slides_array) = data["slides"].as_array() {
            for slide_data in slides_array {
                let slide_id = slide_data["objectId"].as_str().unwrap_or("").to_string();
                let (slide_title, content, notes) = self.extract_slide_content(slide_data);
                
                slides.push(GoogleSlide {
                    slide_id,
                    title: slide_title,
                    content,
                    notes,
                });
            }
        }

        // Get metadata from Drive API (similar to above)
        let drive_url = format!("https://www.googleapis.com/drive/v3/files/{}?fields=createdTime,modifiedTime,owners,permissions", presentation_id);
        let drive_response = self.client
            .get(&drive_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let drive_data: Value = if drive_response.status().is_success() {
            drive_response.json().await?
        } else {
            serde_json::json!({})
        };

        let created_time = drive_data["createdTime"].as_str().map(|s| s.to_string());
        let modified_time = drive_data["modifiedTime"].as_str().map(|s| s.to_string());
        let author = drive_data["owners"].as_array()
            .and_then(|owners| owners.first())
            .and_then(|owner| owner["displayName"].as_str())
            .map(|s| s.to_string());

        let sharing_info = self.extract_sharing_info(&drive_data);

        Ok(GooglePresentationContent {
            presentation_id: presentation_id.to_string(),
            title,
            slides,
            created_time,
            modified_time,
            author,
            sharing_info,
        })
    }

    // Helper methods
    fn extract_body_text(&self, body: &Value) -> Result<String, Box<dyn std::error::Error>> {
        let mut text = String::new();
        
        if let Some(content) = body["content"].as_array() {
            for element in content {
                if let Some(paragraph) = element["paragraph"].as_object() {
                    if let Some(elements) = paragraph["elements"].as_array() {
                        for elem in elements {
                            if let Some(text_run) = elem["textRun"].as_object() {
                                if let Some(content_text) = text_run["content"].as_str() {
                                    text.push_str(content_text);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(text)
    }

    async fn extract_suggestions(&self, document_id: &str, access_token: &str) -> Result<Vec<GoogleSuggestion>, Box<dyn std::error::Error>> {
        // Google Docs API doesn't directly expose suggestions in a simple way
        // This would require using the Drive Activity API or other specialized endpoints
        // For now, return empty vector - can be enhanced later
        Ok(Vec::new())
    }

    async fn extract_comments(&self, document_id: &str, access_token: &str) -> Result<Vec<GoogleComment>, Box<dyn std::error::Error>> {
        let url = format!("https://www.googleapis.com/drive/v3/files/{}/comments", document_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(Vec::new()); // Comments might not be accessible
        }

        let data: Value = response.json().await?;
        let mut comments = Vec::new();

        if let Some(comments_array) = data["comments"].as_array() {
            for comment_data in comments_array {
                let id = comment_data["id"].as_str().unwrap_or("").to_string();
                let author = comment_data["author"]["displayName"].as_str().unwrap_or("Unknown").to_string();
                let create_time = comment_data["createdTime"].as_str().unwrap_or("").to_string();
                let content = comment_data["content"].as_str().unwrap_or("").to_string();
                let resolved = comment_data["resolved"].as_bool().unwrap_or(false);
                
                let mut replies = Vec::new();
                if let Some(replies_array) = comment_data["replies"].as_array() {
                    for reply_data in replies_array {
                        replies.push(GoogleCommentReply {
                            id: reply_data["id"].as_str().unwrap_or("").to_string(),
                            author: reply_data["author"]["displayName"].as_str().unwrap_or("Unknown").to_string(),
                            create_time: reply_data["createdTime"].as_str().unwrap_or("").to_string(),
                            content: reply_data["content"].as_str().unwrap_or("").to_string(),
                        });
                    }
                }

                comments.push(GoogleComment {
                    id,
                    author,
                    create_time,
                    content,
                    resolved,
                    replies,
                });
            }
        }

        Ok(comments)
    }

    async fn extract_revisions(&self, document_id: &str, access_token: &str) -> Result<Vec<GoogleRevision>, Box<dyn std::error::Error>> {
        let url = format!("https://www.googleapis.com/drive/v3/files/{}/revisions", document_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let data: Value = response.json().await?;
        let mut revisions = Vec::new();

        if let Some(revisions_array) = data["revisions"].as_array() {
            for revision_data in revisions_array {
                revisions.push(GoogleRevision {
                    id: revision_data["id"].as_str().unwrap_or("").to_string(),
                    modified_time: revision_data["modifiedTime"].as_str().unwrap_or("").to_string(),
                    last_modifying_user: revision_data["lastModifyingUser"]["displayName"].as_str().unwrap_or("Unknown").to_string(),
                    size: revision_data["size"].as_str().and_then(|s| s.parse().ok()),
                });
            }
        }

        Ok(revisions)
    }

    fn extract_sharing_info(&self, drive_data: &Value) -> GoogleSharingInfo {
        let mut permissions = Vec::new();
        
        if let Some(permissions_array) = drive_data["permissions"].as_array() {
            for perm_data in permissions_array {
                permissions.push(GooglePermission {
                    role: perm_data["role"].as_str().unwrap_or("reader").to_string(),
                    user_type: perm_data["type"].as_str().unwrap_or("user").to_string(),
                    email: perm_data["emailAddress"].as_str().map(|s| s.to_string()),
                    domain: perm_data["domain"].as_str().map(|s| s.to_string()),
                });
            }
        }

        GoogleSharingInfo {
            shared_with_me: false, // TODO: Determine from context
            owned_by_me: false,    // TODO: Determine from context
            permissions,
            sharing_link: drive_data["webViewLink"].as_str().map(|s| s.to_string()),
        }
    }

    async fn extract_sheet_values(&self, spreadsheet_id: &str, sheet_name: &str, access_token: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let range = format!("'{}'", sheet_name);
        let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}", spreadsheet_id, urlencoding::encode(&range));
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let data: Value = response.json().await?;
        let mut sheet_data = Vec::new();

        if let Some(values) = data["values"].as_array() {
            for row in values {
                if let Some(row_array) = row.as_array() {
                    let row_data: Vec<String> = row_array
                        .iter()
                        .map(|cell| cell.as_str().unwrap_or("").to_string())
                        .collect();
                    sheet_data.push(row_data);
                }
            }
        }

        Ok(sheet_data)
    }

    fn extract_slide_content(&self, slide_data: &Value) -> (String, String, String) {
        let mut title = String::new();
        let mut content = String::new();
        let mut notes = String::new();

        // Extract text from slide elements
        if let Some(page_elements) = slide_data["pageElements"].as_array() {
            for element in page_elements {
                if let Some(shape) = element["shape"].as_object() {
                    if let Some(text) = shape["text"].as_object() {
                        if let Some(text_elements) = text["textElements"].as_array() {
                            for text_elem in text_elements {
                                if let Some(text_run) = text_elem["textRun"].as_object() {
                                    if let Some(content_text) = text_run["content"].as_str() {
                                        content.push_str(content_text);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Extract speaker notes
        if let Some(slide_properties) = slide_data["slideProperties"].as_object() {
            if let Some(notes_page) = slide_properties["notesPage"].as_object() {
                // Similar extraction for notes...
            }
        }

        // Try to identify title from first text element or slide properties
        if content.len() > 100 {
            let lines: Vec<&str> = content.lines().collect();
            if !lines.is_empty() {
                title = lines[0].trim().to_string();
                if title.len() > 100 {
                    title = format!("{}...", &title[..97]);
                }
            }
        }

        (title, content, notes)
    }
}

#[derive(Debug)]
pub enum GoogleContentType {
    Document(GoogleDocumentContent),
    Spreadsheet(GoogleSpreadsheetContent),
    Presentation(GooglePresentationContent),
}

impl GoogleApiClient {
    pub async fn extract_content_by_platform_type(&mut self, platform_type: &PlatformType) -> Result<GoogleContentType, Box<dyn std::error::Error>> {
        match platform_type {
            PlatformType::GoogleDocs { document_id } => {
                let content = self.extract_document_content(document_id).await?;
                Ok(GoogleContentType::Document(content))
            },
            PlatformType::GoogleSheets { spreadsheet_id } => {
                let content = self.extract_spreadsheet_content(spreadsheet_id).await?;
                Ok(GoogleContentType::Spreadsheet(content))
            },
            PlatformType::GoogleSlides { presentation_id } => {
                let content = self.extract_presentation_content(presentation_id).await?;
                Ok(GoogleContentType::Presentation(content))
            },
            _ => Err("Not a Google platform type".into()),
        }
    }
}