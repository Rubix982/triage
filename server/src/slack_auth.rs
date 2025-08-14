use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub user_scopes: Vec<String>, // For user tokens
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlackTokens {
    pub access_token: String, // Bot token
    pub user_token: Option<String>, // User token for private content
    pub team_id: String,
    pub team_name: String,
    pub bot_user_id: String,
    pub user_id: Option<String>,
    pub expires_at: Option<DateTime<Utc>>, // Most Slack tokens don't expire
    pub scope: String,
    pub user_scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackTokenResponse {
    pub ok: bool,
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub bot_user_id: Option<String>,
    pub app_id: String,
    pub team: SlackTeamInfo,
    pub enterprise: Option<SlackEnterpriseInfo>,
    pub authed_user: Option<SlackAuthedUser>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackTeamInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackEnterpriseInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackAuthedUser {
    pub id: String,
    pub scope: Option<String>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackAuthorizationRequest {
    pub client_id: String,
    pub scope: String,
    pub user_scope: Option<String>,
    pub redirect_uri: String,
    pub state: String,
    pub team: Option<String>,
}

pub struct SlackAuthManager {
    config: SlackOAuthConfig,
    client: Client,
    tokens: Option<SlackTokens>,
}

impl SlackAuthManager {
    pub fn new(config: SlackOAuthConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            tokens: None,
        }
    }

    pub fn get_authorization_url(&self, state: &str, team_id: Option<&str>) -> String {
        let scope = self.config.scopes.join(",");
        let user_scope = if !self.config.user_scopes.is_empty() {
            Some(self.config.user_scopes.join(","))
        } else {
            None
        };

        let mut params = vec![
            ("client_id", self.config.client_id.as_str()),
            ("scope", &scope),
            ("redirect_uri", &self.config.redirect_uri),
            ("state", state),
        ];

        if let Some(user_scope) = &user_scope {
            params.push(("user_scope", user_scope));
        }

        if let Some(team) = team_id {
            params.push(("team", team));
        }

        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<String>>()
            .join("&");

        format!("https://slack.com/oauth/v2/authorize?{}", query_string)
    }

    pub async fn exchange_code_for_tokens(&mut self, authorization_code: &str) -> Result<(), Box<dyn std::error::Error>> {
        let token_url = "https://slack.com/api/oauth.v2.access";
        
        let mut params = HashMap::new();
        params.insert("client_id", &self.config.client_id);
        params.insert("client_secret", &self.config.client_secret);
        let code = authorization_code.to_string();
        params.insert("code", &code);
        params.insert("redirect_uri", &self.config.redirect_uri);

        let response = self.client
            .post(token_url)
            .form(&params)
            .send()
            .await?;

        let token_response: SlackTokenResponse = response.json().await?;
        
        if !token_response.ok {
            return Err(format!("Slack OAuth error: {}", 
                token_response.error.unwrap_or("Unknown error".to_string())).into());
        }

        let user_token = token_response.authed_user
            .as_ref()
            .and_then(|user| user.access_token.clone());

        let user_scope = token_response.authed_user
            .as_ref()
            .and_then(|user| user.scope.clone());

        let user_id = token_response.authed_user
            .as_ref()
            .map(|user| user.id.clone());

        self.tokens = Some(SlackTokens {
            access_token: token_response.access_token,
            user_token,
            team_id: token_response.team.id,
            team_name: token_response.team.name,
            bot_user_id: token_response.bot_user_id.unwrap_or_default(),
            user_id,
            expires_at: None, // Slack tokens typically don't expire
            scope: token_response.scope,
            user_scope,
        });

        Ok(())
    }

    pub async fn test_auth(&self) -> Result<SlackAuthTest, Box<dyn std::error::Error>> {
        let tokens = self.tokens.as_ref().ok_or("No tokens available")?;
        
        let test_url = "https://slack.com/api/auth.test";
        let response = self.client
            .post(test_url)
            .header("Authorization", format!("Bearer {}", tokens.access_token))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?;

        let auth_test: SlackAuthTest = response.json().await?;
        
        if !auth_test.ok {
            return Err(format!("Auth test failed: {}", 
                auth_test.error.unwrap_or("Unknown error".to_string())).into());
        }

        Ok(auth_test)
    }

    pub fn get_bot_token(&self) -> Option<&str> {
        self.tokens.as_ref().map(|t| t.access_token.as_str())
    }

    pub fn get_user_token(&self) -> Option<&str> {
        self.tokens.as_ref().and_then(|t| t.user_token.as_deref())
    }

    pub fn is_authenticated(&self) -> bool {
        self.tokens.is_some()
    }

    pub fn get_tokens(&self) -> Option<&SlackTokens> {
        self.tokens.as_ref()
    }

    pub fn set_tokens(&mut self, tokens: SlackTokens) {
        self.tokens = Some(tokens);
    }

    pub fn get_team_info(&self) -> Option<(String, String)> {
        self.tokens.as_ref().map(|t| (t.team_id.clone(), t.team_name.clone()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackAuthTest {
    pub ok: bool,
    pub url: Option<String>,
    pub team: Option<String>,
    pub user: Option<String>,
    pub team_id: Option<String>,
    pub user_id: Option<String>,
    pub bot_id: Option<String>,
    pub is_enterprise_install: Option<bool>,
    pub error: Option<String>,
}

impl Default for SlackOAuthConfig {
    fn default() -> Self {
        Self {
            client_id: std::env::var("SLACK_CLIENT_ID").unwrap_or_default(),
            client_secret: std::env::var("SLACK_CLIENT_SECRET").unwrap_or_default(),
            redirect_uri: std::env::var("SLACK_REDIRECT_URI")
                .unwrap_or_else(|_| "http://localhost:3001/auth/slack/callback".to_string()),
            scopes: vec![
                "channels:read".to_string(),      // Read public channel info
                "groups:read".to_string(),        // Read private channel info (if bot is added)
                "im:read".to_string(),            // Read direct messages (if bot is added)
                "mpim:read".to_string(),          // Read group DMs (if bot is added)
                "channels:history".to_string(),   // Read message history in public channels
                "groups:history".to_string(),     // Read message history in private channels
                "im:history".to_string(),         // Read DM history
                "mpim:history".to_string(),       // Read group DM history
                "users:read".to_string(),         // Read user information
                "files:read".to_string(),         // Read file information
                "reactions:read".to_string(),     // Read message reactions
            ],
            user_scopes: vec![
                "channels:read".to_string(),      // User-level access to channels
                "groups:read".to_string(),        // User-level access to private channels
                "im:read".to_string(),            // User-level access to DMs
                "mpim:read".to_string(),          // User-level access to group DMs
                "channels:history".to_string(),   // User-level message history
                "groups:history".to_string(),     // User-level private message history
                "im:history".to_string(),         // User-level DM history
                "mpim:history".to_string(),       // User-level group DM history
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slack_authorization_url() {
        let config = SlackOAuthConfig {
            client_id: "test_client_id".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: "http://localhost:3001/callback".to_string(),
            scopes: vec![
                "channels:read".to_string(),
                "channels:history".to_string(),
            ],
            user_scopes: vec![
                "channels:read".to_string(),
            ],
        };
        
        let auth_manager = SlackAuthManager::new(config);
        let url = auth_manager.get_authorization_url("test_state", Some("T1234567"));
        
        assert!(url.contains("slack.com/oauth/v2/authorize"));
        assert!(url.contains("client_id=test_client_id"));
        assert!(url.contains("state=test_state"));
        assert!(url.contains("team=T1234567"));
        assert!(url.contains("scope=channels%3Aread%2Cchannels%3Ahistory"));
        assert!(url.contains("user_scope=channels%3Aread"));
    }

    #[test]
    fn test_default_config() {
        let config = SlackOAuthConfig::default();
        
        assert!(!config.scopes.is_empty());
        assert!(config.scopes.contains(&"channels:read".to_string()));
        assert!(config.scopes.contains(&"channels:history".to_string()));
        assert!(config.scopes.contains(&"users:read".to_string()));
        
        assert!(!config.user_scopes.is_empty());
        assert!(config.user_scopes.contains(&"channels:read".to_string()));
    }
}