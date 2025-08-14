use crate::slack_auth::SlackAuthManager;
use crate::types::PlatformType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::Client;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackConversationContent {
    pub channel_id: String,
    pub channel_name: String,
    pub channel_type: SlackChannelType,
    pub thread_ts: Option<String>,
    pub messages: Vec<SlackMessage>,
    pub channel_info: SlackChannelInfo,
    pub participants: Vec<SlackUser>,
    pub extracted_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackMessage {
    pub ts: String,
    pub user: Option<String>,
    pub username: Option<String>,
    pub bot_id: Option<String>,
    pub text: String,
    pub blocks: Option<Vec<Value>>, // Rich text blocks
    pub attachments: Vec<SlackAttachment>,
    pub files: Vec<SlackFile>,
    pub reactions: Vec<SlackReaction>,
    pub replies: Vec<SlackThreadReply>,
    pub edited: Option<SlackMessageEdit>,
    pub message_type: String,
    pub subtype: Option<String>,
    pub parent_user_id: Option<String>, // For threaded messages
    pub thread_ts: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackThreadReply {
    pub user: String,
    pub ts: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackMessageEdit {
    pub user: String,
    pub ts: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackAttachment {
    pub id: Option<i64>,
    pub fallback: Option<String>,
    pub color: Option<String>,
    pub pretext: Option<String>,
    pub author_name: Option<String>,
    pub author_link: Option<String>,
    pub title: Option<String>,
    pub title_link: Option<String>,
    pub text: Option<String>,
    pub fields: Vec<SlackAttachmentField>,
    pub image_url: Option<String>,
    pub thumb_url: Option<String>,
    pub footer: Option<String>,
    pub footer_icon: Option<String>,
    pub ts: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackAttachmentField {
    pub title: String,
    pub value: String,
    pub short: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackFile {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
    pub mimetype: String,
    pub filetype: String,
    pub size: i64,
    pub url_private: String,
    pub url_private_download: String,
    pub permalink: String,
    pub permalink_public: Option<String>,
    pub user: String,
    pub timestamp: String,
    pub is_external: bool,
    pub external_type: Option<String>,
    pub is_public: bool,
    pub public_url_shared: bool,
    pub display_as_bot: bool,
    pub username: Option<String>,
    pub initial_comment: Option<SlackFileComment>,
    pub num_stars: Option<i32>,
    pub is_starred: Option<bool>,
    pub preview: Option<String>,
    pub preview_highlight: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackFileComment {
    pub id: String,
    pub created: i64,
    pub timestamp: i64,
    pub user: String,
    pub is_intro: bool,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackReaction {
    pub name: String,
    pub users: Vec<String>,
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackChannelInfo {
    pub id: String,
    pub name: String,
    pub created: i64,
    pub creator: String,
    pub is_archived: bool,
    pub is_general: bool,
    pub name_normalized: String,
    pub is_shared: bool,
    pub is_org_shared: bool,
    pub is_member: bool,
    pub is_private: bool,
    pub is_mpim: bool,
    pub topic: SlackChannelTopic,
    pub purpose: SlackChannelPurpose,
    pub num_members: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackChannelTopic {
    pub value: String,
    pub creator: String,
    pub last_set: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackChannelPurpose {
    pub value: String,
    pub creator: String,
    pub last_set: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackUser {
    pub id: String,
    pub team_id: String,
    pub name: String,
    pub deleted: bool,
    pub color: Option<String>,
    pub real_name: String,
    pub tz: Option<String>,
    pub tz_label: Option<String>,
    pub tz_offset: Option<i32>,
    pub profile: SlackUserProfile,
    pub is_admin: bool,
    pub is_owner: bool,
    pub is_primary_owner: bool,
    pub is_restricted: bool,
    pub is_ultra_restricted: bool,
    pub is_bot: bool,
    pub is_app_user: bool,
    pub updated: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackUserProfile {
    pub title: String,
    pub phone: String,
    pub skype: String,
    pub real_name: String,
    pub real_name_normalized: String,
    pub display_name: String,
    pub display_name_normalized: String,
    pub fields: Option<HashMap<String, Value>>,
    pub status_text: String,
    pub status_emoji: String,
    pub status_expiration: i64,
    pub avatar_hash: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub image_24: String,
    pub image_32: String,
    pub image_48: String,
    pub image_72: String,
    pub image_192: String,
    pub image_512: String,
    pub image_original: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SlackChannelType {
    Public,
    Private,
    DirectMessage,
    MultiPartyDirectMessage,
    Unknown,
}

pub struct SlackApiClient {
    auth_manager: SlackAuthManager,
    client: Client,
}

impl SlackApiClient {
    pub fn new(auth_manager: SlackAuthManager) -> Self {
        Self {
            auth_manager,
            client: Client::new(),
        }
    }

    pub async fn extract_conversation_content(&mut self, channel_id: &str, thread_ts: Option<&str>) -> Result<SlackConversationContent, Box<dyn std::error::Error>> {
        // Get channel information
        let channel_info = self.get_channel_info(channel_id).await?;
        let channel_type = self.determine_channel_type(&channel_info);
        
        // Get conversation history
        let messages = if let Some(thread_ts) = thread_ts {
            self.get_thread_messages(channel_id, thread_ts).await?
        } else {
            self.get_channel_messages(channel_id, None, None).await?
        };

        // Get unique user IDs from messages
        let mut user_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
        for message in &messages {
            if let Some(user) = &message.user {
                user_ids.insert(user.clone());
            }
            // Add users from reactions
            for reaction in &message.reactions {
                for user in &reaction.users {
                    user_ids.insert(user.clone());
                }
            }
        }

        // Get user information
        let user_ids_vec: Vec<String> = user_ids.into_iter().collect();
        let participants = self.get_users_info(&user_ids_vec).await?;

        Ok(SlackConversationContent {
            channel_id: channel_id.to_string(),
            channel_name: channel_info.name.clone(),
            channel_type,
            thread_ts: thread_ts.map(|s| s.to_string()),
            messages,
            channel_info,
            participants,
            extracted_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    pub async fn get_channel_info(&self, channel_id: &str) -> Result<SlackChannelInfo, Box<dyn std::error::Error>> {
        let bot_token = self.auth_manager.get_bot_token().ok_or("No bot token available")?;
        
        let url = "https://slack.com/api/conversations.info";
        let mut params = HashMap::new();
        params.insert("channel", channel_id);

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", bot_token))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        let data: Value = response.json().await?;
        
        if !data["ok"].as_bool().unwrap_or(false) {
            return Err(format!("Slack API error: {}", 
                data["error"].as_str().unwrap_or("Unknown error")).into());
        }

        let channel = &data["channel"];
        Ok(SlackChannelInfo {
            id: channel["id"].as_str().unwrap_or("").to_string(),
            name: channel["name"].as_str().unwrap_or("").to_string(),
            created: channel["created"].as_i64().unwrap_or(0),
            creator: channel["creator"].as_str().unwrap_or("").to_string(),
            is_archived: channel["is_archived"].as_bool().unwrap_or(false),
            is_general: channel["is_general"].as_bool().unwrap_or(false),
            name_normalized: channel["name_normalized"].as_str().unwrap_or("").to_string(),
            is_shared: channel["is_shared"].as_bool().unwrap_or(false),
            is_org_shared: channel["is_org_shared"].as_bool().unwrap_or(false),
            is_member: channel["is_member"].as_bool().unwrap_or(false),
            is_private: channel["is_private"].as_bool().unwrap_or(false),
            is_mpim: channel["is_mpim"].as_bool().unwrap_or(false),
            topic: SlackChannelTopic {
                value: channel["topic"]["value"].as_str().unwrap_or("").to_string(),
                creator: channel["topic"]["creator"].as_str().unwrap_or("").to_string(),
                last_set: channel["topic"]["last_set"].as_i64().unwrap_or(0),
            },
            purpose: SlackChannelPurpose {
                value: channel["purpose"]["value"].as_str().unwrap_or("").to_string(),
                creator: channel["purpose"]["creator"].as_str().unwrap_or("").to_string(),
                last_set: channel["purpose"]["last_set"].as_i64().unwrap_or(0),
            },
            num_members: channel["num_members"].as_i64().map(|n| n as i32),
        })
    }

    async fn get_channel_messages(&self, channel_id: &str, cursor: Option<&str>, limit: Option<i32>) -> Result<Vec<SlackMessage>, Box<dyn std::error::Error>> {
        let bot_token = self.auth_manager.get_bot_token().ok_or("No bot token available")?;
        
        let url = "https://slack.com/api/conversations.history";
        let mut params = HashMap::new();
        params.insert("channel", channel_id);
        let limit_str = limit.unwrap_or(100).to_string();
        params.insert("limit", &limit_str);

        if let Some(cursor) = cursor {
            params.insert("cursor", cursor);
        }

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", bot_token))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        let data: Value = response.json().await?;
        
        if !data["ok"].as_bool().unwrap_or(false) {
            return Err(format!("Slack API error: {}", 
                data["error"].as_str().unwrap_or("Unknown error")).into());
        }

        let mut messages = Vec::new();
        if let Some(messages_array) = data["messages"].as_array() {
            for message_data in messages_array {
                messages.push(self.parse_message(message_data)?);
            }
        }

        Ok(messages)
    }

    pub async fn get_thread_messages(&self, channel_id: &str, thread_ts: &str) -> Result<Vec<SlackMessage>, Box<dyn std::error::Error>> {
        let bot_token = self.auth_manager.get_bot_token().ok_or("No bot token available")?;
        
        let url = "https://slack.com/api/conversations.replies";
        let mut params = HashMap::new();
        params.insert("channel", channel_id);
        params.insert("ts", thread_ts);

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", bot_token))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        let data: Value = response.json().await?;
        
        if !data["ok"].as_bool().unwrap_or(false) {
            return Err(format!("Slack API error: {}", 
                data["error"].as_str().unwrap_or("Unknown error")).into());
        }

        let mut messages = Vec::new();
        if let Some(messages_array) = data["messages"].as_array() {
            for message_data in messages_array {
                messages.push(self.parse_message(message_data)?);
            }
        }

        Ok(messages)
    }

    async fn get_users_info(&self, user_ids: &[String]) -> Result<Vec<SlackUser>, Box<dyn std::error::Error>> {
        let bot_token = self.auth_manager.get_bot_token().ok_or("No bot token available")?;
        let mut users = Vec::new();

        // Slack users.info API only accepts one user at a time
        for user_id in user_ids {
            let url = "https://slack.com/api/users.info";
            let mut params = HashMap::new();
            params.insert("user", user_id.as_str());

            let response = self.client
                .post(url)
                .header("Authorization", format!("Bearer {}", bot_token))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .form(&params)
                .send()
                .await?;

            let data: Value = response.json().await?;
            
            if data["ok"].as_bool().unwrap_or(false) {
                if let Some(user_data) = data.get("user") {
                    users.push(self.parse_user(user_data)?);
                }
            }
            // Continue even if one user fails
        }

        Ok(users)
    }

    fn parse_message(&self, message_data: &Value) -> Result<SlackMessage, Box<dyn std::error::Error>> {
        let ts = message_data["ts"].as_str().unwrap_or("").to_string();
        let user = message_data["user"].as_str().map(|s| s.to_string());
        let username = message_data["username"].as_str().map(|s| s.to_string());
        let bot_id = message_data["bot_id"].as_str().map(|s| s.to_string());
        let text = message_data["text"].as_str().unwrap_or("").to_string();
        let message_type = message_data["type"].as_str().unwrap_or("message").to_string();
        let subtype = message_data["subtype"].as_str().map(|s| s.to_string());
        let thread_ts = message_data["thread_ts"].as_str().map(|s| s.to_string());
        let parent_user_id = message_data["parent_user_id"].as_str().map(|s| s.to_string());

        // Parse attachments
        let mut attachments = Vec::new();
        if let Some(attachments_array) = message_data["attachments"].as_array() {
            for attachment_data in attachments_array {
                attachments.push(self.parse_attachment(attachment_data)?);
            }
        }

        // Parse files
        let mut files = Vec::new();
        if let Some(files_array) = message_data["files"].as_array() {
            for file_data in files_array {
                files.push(self.parse_file(file_data)?);
            }
        }

        // Parse reactions
        let mut reactions = Vec::new();
        if let Some(reactions_array) = message_data["reactions"].as_array() {
            for reaction_data in reactions_array {
                let name = reaction_data["name"].as_str().unwrap_or("").to_string();
                let count = reaction_data["count"].as_i64().unwrap_or(0) as i32;
                let users: Vec<String> = reaction_data["users"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|u| u.as_str())
                    .map(|s| s.to_string())
                    .collect();

                reactions.push(SlackReaction { name, users, count });
            }
        }

        // Parse thread replies info
        let mut replies = Vec::new();
        if let Some(replies_array) = message_data["replies"].as_array() {
            for reply_data in replies_array {
                let user = reply_data["user"].as_str().unwrap_or("").to_string();
                let ts = reply_data["ts"].as_str().unwrap_or("").to_string();
                replies.push(SlackThreadReply { user, ts });
            }
        }

        // Parse edit info
        let edited = if let Some(edited_data) = message_data.get("edited") {
            Some(SlackMessageEdit {
                user: edited_data["user"].as_str().unwrap_or("").to_string(),
                ts: edited_data["ts"].as_str().unwrap_or("").to_string(),
            })
        } else {
            None
        };

        Ok(SlackMessage {
            ts,
            user,
            username,
            bot_id,
            text,
            blocks: message_data.get("blocks")
                .and_then(|b| b.as_array())
                .map(|arr| arr.clone()),
            attachments,
            files,
            reactions,
            replies,
            edited,
            message_type,
            subtype,
            parent_user_id,
            thread_ts,
        })
    }

    fn parse_attachment(&self, attachment_data: &Value) -> Result<SlackAttachment, Box<dyn std::error::Error>> {
        let mut fields = Vec::new();
        if let Some(fields_array) = attachment_data["fields"].as_array() {
            for field_data in fields_array {
                fields.push(SlackAttachmentField {
                    title: field_data["title"].as_str().unwrap_or("").to_string(),
                    value: field_data["value"].as_str().unwrap_or("").to_string(),
                    short: field_data["short"].as_bool(),
                });
            }
        }

        Ok(SlackAttachment {
            id: attachment_data["id"].as_i64(),
            fallback: attachment_data["fallback"].as_str().map(|s| s.to_string()),
            color: attachment_data["color"].as_str().map(|s| s.to_string()),
            pretext: attachment_data["pretext"].as_str().map(|s| s.to_string()),
            author_name: attachment_data["author_name"].as_str().map(|s| s.to_string()),
            author_link: attachment_data["author_link"].as_str().map(|s| s.to_string()),
            title: attachment_data["title"].as_str().map(|s| s.to_string()),
            title_link: attachment_data["title_link"].as_str().map(|s| s.to_string()),
            text: attachment_data["text"].as_str().map(|s| s.to_string()),
            fields,
            image_url: attachment_data["image_url"].as_str().map(|s| s.to_string()),
            thumb_url: attachment_data["thumb_url"].as_str().map(|s| s.to_string()),
            footer: attachment_data["footer"].as_str().map(|s| s.to_string()),
            footer_icon: attachment_data["footer_icon"].as_str().map(|s| s.to_string()),
            ts: attachment_data["ts"].as_str().map(|s| s.to_string()),
        })
    }

    fn parse_file(&self, file_data: &Value) -> Result<SlackFile, Box<dyn std::error::Error>> {
        let initial_comment = if let Some(comment_data) = file_data.get("initial_comment") {
            Some(SlackFileComment {
                id: comment_data["id"].as_str().unwrap_or("").to_string(),
                created: comment_data["created"].as_i64().unwrap_or(0),
                timestamp: comment_data["timestamp"].as_i64().unwrap_or(0),
                user: comment_data["user"].as_str().unwrap_or("").to_string(),
                is_intro: comment_data["is_intro"].as_bool().unwrap_or(false),
                comment: comment_data["comment"].as_str().unwrap_or("").to_string(),
            })
        } else {
            None
        };

        Ok(SlackFile {
            id: file_data["id"].as_str().unwrap_or("").to_string(),
            name: file_data["name"].as_str().unwrap_or("").to_string(),
            title: file_data["title"].as_str().map(|s| s.to_string()),
            mimetype: file_data["mimetype"].as_str().unwrap_or("").to_string(),
            filetype: file_data["filetype"].as_str().unwrap_or("").to_string(),
            size: file_data["size"].as_i64().unwrap_or(0),
            url_private: file_data["url_private"].as_str().unwrap_or("").to_string(),
            url_private_download: file_data["url_private_download"].as_str().unwrap_or("").to_string(),
            permalink: file_data["permalink"].as_str().unwrap_or("").to_string(),
            permalink_public: file_data["permalink_public"].as_str().map(|s| s.to_string()),
            user: file_data["user"].as_str().unwrap_or("").to_string(),
            timestamp: file_data["timestamp"].as_str().unwrap_or("").to_string(),
            is_external: file_data["is_external"].as_bool().unwrap_or(false),
            external_type: file_data["external_type"].as_str().map(|s| s.to_string()),
            is_public: file_data["is_public"].as_bool().unwrap_or(false),
            public_url_shared: file_data["public_url_shared"].as_bool().unwrap_or(false),
            display_as_bot: file_data["display_as_bot"].as_bool().unwrap_or(false),
            username: file_data["username"].as_str().map(|s| s.to_string()),
            initial_comment,
            num_stars: file_data["num_stars"].as_i64().map(|n| n as i32),
            is_starred: file_data["is_starred"].as_bool(),
            preview: file_data["preview"].as_str().map(|s| s.to_string()),
            preview_highlight: file_data["preview_highlight"].as_str().map(|s| s.to_string()),
        })
    }

    fn parse_user(&self, user_data: &Value) -> Result<SlackUser, Box<dyn std::error::Error>> {
        let profile = &user_data["profile"];
        let profile_fields = profile.get("fields").and_then(|f| f.as_object()).map(|obj| {
            obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
        });

        Ok(SlackUser {
            id: user_data["id"].as_str().unwrap_or("").to_string(),
            team_id: user_data["team_id"].as_str().unwrap_or("").to_string(),
            name: user_data["name"].as_str().unwrap_or("").to_string(),
            deleted: user_data["deleted"].as_bool().unwrap_or(false),
            color: user_data["color"].as_str().map(|s| s.to_string()),
            real_name: user_data["real_name"].as_str().unwrap_or("").to_string(),
            tz: user_data["tz"].as_str().map(|s| s.to_string()),
            tz_label: user_data["tz_label"].as_str().map(|s| s.to_string()),
            tz_offset: user_data["tz_offset"].as_i64().map(|n| n as i32),
            profile: SlackUserProfile {
                title: profile["title"].as_str().unwrap_or("").to_string(),
                phone: profile["phone"].as_str().unwrap_or("").to_string(),
                skype: profile["skype"].as_str().unwrap_or("").to_string(),
                real_name: profile["real_name"].as_str().unwrap_or("").to_string(),
                real_name_normalized: profile["real_name_normalized"].as_str().unwrap_or("").to_string(),
                display_name: profile["display_name"].as_str().unwrap_or("").to_string(),
                display_name_normalized: profile["display_name_normalized"].as_str().unwrap_or("").to_string(),
                fields: profile_fields,
                status_text: profile["status_text"].as_str().unwrap_or("").to_string(),
                status_emoji: profile["status_emoji"].as_str().unwrap_or("").to_string(),
                status_expiration: profile["status_expiration"].as_i64().unwrap_or(0),
                avatar_hash: profile["avatar_hash"].as_str().unwrap_or("").to_string(),
                email: profile["email"].as_str().map(|s| s.to_string()),
                first_name: profile["first_name"].as_str().map(|s| s.to_string()),
                last_name: profile["last_name"].as_str().map(|s| s.to_string()),
                image_24: profile["image_24"].as_str().unwrap_or("").to_string(),
                image_32: profile["image_32"].as_str().unwrap_or("").to_string(),
                image_48: profile["image_48"].as_str().unwrap_or("").to_string(),
                image_72: profile["image_72"].as_str().unwrap_or("").to_string(),
                image_192: profile["image_192"].as_str().unwrap_or("").to_string(),
                image_512: profile["image_512"].as_str().unwrap_or("").to_string(),
                image_original: profile["image_original"].as_str().map(|s| s.to_string()),
            },
            is_admin: user_data["is_admin"].as_bool().unwrap_or(false),
            is_owner: user_data["is_owner"].as_bool().unwrap_or(false),
            is_primary_owner: user_data["is_primary_owner"].as_bool().unwrap_or(false),
            is_restricted: user_data["is_restricted"].as_bool().unwrap_or(false),
            is_ultra_restricted: user_data["is_ultra_restricted"].as_bool().unwrap_or(false),
            is_bot: user_data["is_bot"].as_bool().unwrap_or(false),
            is_app_user: user_data["is_app_user"].as_bool().unwrap_or(false),
            updated: user_data["updated"].as_i64().unwrap_or(0),
        })
    }

    fn determine_channel_type(&self, channel_info: &SlackChannelInfo) -> SlackChannelType {
        if channel_info.is_mpim {
            SlackChannelType::MultiPartyDirectMessage
        } else if channel_info.is_private {
            if channel_info.name.starts_with("D") {
                SlackChannelType::DirectMessage
            } else {
                SlackChannelType::Private
            }
        } else {
            SlackChannelType::Public
        }
    }

    pub async fn extract_content_by_platform_type(&mut self, platform_type: &PlatformType) -> Result<SlackConversationContent, Box<dyn std::error::Error>> {
        match platform_type {
            PlatformType::SlackThread { workspace: _, channel, thread_ts } => {
                self.extract_conversation_content(channel, Some(thread_ts)).await
            },
            PlatformType::SlackMessage { workspace: _, channel, message_ts } => {
                // For a single message, we'll get the thread if it exists
                self.extract_conversation_content(channel, Some(message_ts)).await
            },
            _ => Err("Not a Slack platform type".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slack_auth::{SlackOAuthConfig, SlackTokens};

    fn create_test_auth_manager() -> SlackAuthManager {
        let config = SlackOAuthConfig::default();
        let mut auth_manager = SlackAuthManager::new(config);
        
        // Set mock tokens
        auth_manager.set_tokens(SlackTokens {
            access_token: "xoxb-test-token".to_string(),
            user_token: Some("xoxp-user-token".to_string()),
            team_id: "T1234567".to_string(),
            team_name: "Test Team".to_string(),
            bot_user_id: "U1234567".to_string(),
            user_id: Some("U7654321".to_string()),
            expires_at: None,
            scope: "channels:read,channels:history".to_string(),
            user_scope: Some("channels:read".to_string()),
        });
        
        auth_manager
    }

    #[test]
    fn test_slack_api_client_creation() {
        let auth_manager = create_test_auth_manager();
        let client = SlackApiClient::new(auth_manager);
        
        // Test that client is created successfully
        assert!(client.auth_manager.is_authenticated());
    }

    #[test]
    fn test_channel_type_determination() {
        let auth_manager = create_test_auth_manager();
        let client = SlackApiClient::new(auth_manager);

        // Test public channel
        let public_channel = SlackChannelInfo {
            id: "C1234567".to_string(),
            name: "general".to_string(),
            created: 0,
            creator: "".to_string(),
            is_archived: false,
            is_general: true,
            name_normalized: "general".to_string(),
            is_shared: false,
            is_org_shared: false,
            is_member: true,
            is_private: false,
            is_mpim: false,
            topic: SlackChannelTopic { value: "".to_string(), creator: "".to_string(), last_set: 0 },
            purpose: SlackChannelPurpose { value: "".to_string(), creator: "".to_string(), last_set: 0 },
            num_members: Some(10),
        };

        assert_eq!(client.determine_channel_type(&public_channel), SlackChannelType::Public);

        // Test private channel
        let mut private_channel = public_channel.clone();
        private_channel.is_private = true;
        assert_eq!(client.determine_channel_type(&private_channel), SlackChannelType::Private);

        // Test MPIM
        let mut mpim_channel = public_channel.clone();
        mpim_channel.is_mpim = true;
        assert_eq!(client.determine_channel_type(&mpim_channel), SlackChannelType::MultiPartyDirectMessage);
    }
}