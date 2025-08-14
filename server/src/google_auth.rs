use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleAuthorizationRequest {
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: String,
    pub response_type: String,
    pub access_type: String,
    pub prompt: String,
}

pub struct GoogleAuthManager {
    config: GoogleOAuthConfig,
    client: Client,
    tokens: Option<GoogleTokens>,
}

impl GoogleAuthManager {
    pub fn new(config: GoogleOAuthConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            tokens: None,
        }
    }

    pub fn get_authorization_url(&self, state: &str) -> String {
        let scopes = self.config.scopes.join(" ");
        let params = GoogleAuthorizationRequest {
            client_id: self.config.client_id.clone(),
            redirect_uri: self.config.redirect_uri.clone(),
            scope: scopes,
            response_type: "code".to_string(),
            access_type: "offline".to_string(),
            prompt: "consent".to_string(),
        };

        let string_state = &std::string::String::from(state);
        let query_params = vec![
            ("client_id", &params.client_id),
            ("redirect_uri", &params.redirect_uri),
            ("scope", &params.scope),
            ("response_type", &params.response_type),
            ("access_type", &params.access_type),
            ("prompt", &params.prompt),
            ("state", string_state),
        ];

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<String>>()
            .join("&");

        format!("https://accounts.google.com/o/oauth2/v2/auth?{}", query_string)
    }

    pub async fn exchange_code_for_tokens(&mut self, authorization_code: &str) -> Result<(), Box<dyn std::error::Error>> {
        let token_url = "https://oauth2.googleapis.com/token";
        
        let mut params = HashMap::new();
        params.insert("client_id", &self.config.client_id);
        params.insert("client_secret", &self.config.client_secret);
        params.insert("redirect_uri", &self.config.redirect_uri);
        let grant_type = "authorization_code".to_string();
        params.insert("grant_type", &grant_type);
        let code = authorization_code.to_string();
        params.insert("code", &code);

        let response = self.client
            .post(token_url)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Token exchange failed: {}", error_text).into());
        }

        let token_response: GoogleTokenResponse = response.json().await?;
        
        let expires_at = Utc::now() + Duration::seconds(token_response.expires_in);
        
        self.tokens = Some(GoogleTokens {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_at,
            token_type: token_response.token_type,
            scope: token_response.scope.unwrap_or_default(),
        });

        Ok(())
    }

    pub async fn refresh_tokens(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let tokens = self.tokens.as_ref().ok_or("No tokens available")?;
        let refresh_token = tokens.refresh_token.as_ref().ok_or("No refresh token available")?;

        let token_url = "https://oauth2.googleapis.com/token";
        
        let mut params = HashMap::new();
        params.insert("client_id", &self.config.client_id);
        params.insert("client_secret", &self.config.client_secret);
        let refresh_token_str = refresh_token.to_string();
        params.insert("refresh_token", &refresh_token_str);
        let grant_type = "refresh_token".to_string();
        params.insert("grant_type", &grant_type);

        let response = self.client
            .post(token_url)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Token refresh failed: {}", error_text).into());
        }

        let token_response: GoogleTokenResponse = response.json().await?;
        
        let expires_at = Utc::now() + Duration::seconds(token_response.expires_in);
        
        self.tokens = Some(GoogleTokens {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token.or_else(|| tokens.refresh_token.clone()),
            expires_at,
            token_type: token_response.token_type,
            scope: token_response.scope.unwrap_or_else(|| tokens.scope.clone()),
        });

        Ok(())
    }

    pub async fn get_valid_access_token(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(tokens) = &self.tokens {
            // Check if token is still valid (with 5 minute buffer)
            if tokens.expires_at > Utc::now() + Duration::minutes(5) {
                return Ok(tokens.access_token.clone());
            }
        }

        // Token is expired or about to expire, refresh it
        self.refresh_tokens().await?;
        
        Ok(self.tokens.as_ref().unwrap().access_token.clone())
    }

    pub fn is_authenticated(&self) -> bool {
        self.tokens.is_some()
    }

    pub fn get_tokens(&self) -> Option<&GoogleTokens> {
        self.tokens.as_ref()
    }

    pub fn set_tokens(&mut self, tokens: GoogleTokens) {
        self.tokens = Some(tokens);
    }
}

impl Default for GoogleOAuthConfig {
    fn default() -> Self {
        Self {
            client_id: std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default(),
            redirect_uri: std::env::var("GOOGLE_REDIRECT_URI").unwrap_or_else(|_| "http://localhost:3001/auth/google/callback".to_string()),
            scopes: vec![
                "https://www.googleapis.com/auth/documents.readonly".to_string(),
                "https://www.googleapis.com/auth/drive.readonly".to_string(),
                "https://www.googleapis.com/auth/spreadsheets.readonly".to_string(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization_url_generation() {
        let config = GoogleOAuthConfig {
            client_id: "test_client_id".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: "http://localhost:3001/callback".to_string(),
            scopes: vec![
                "https://www.googleapis.com/auth/documents.readonly".to_string(),
                "https://www.googleapis.com/auth/drive.readonly".to_string(),
            ],
        };
        
        let auth_manager = GoogleAuthManager::new(config);
        let url = auth_manager.get_authorization_url("test_state");
        
        assert!(url.contains("accounts.google.com/o/oauth2/v2/auth"));
        assert!(url.contains("client_id=test_client_id"));
        assert!(url.contains("state=test_state"));
        assert!(url.contains("access_type=offline"));
        assert!(url.contains("prompt=consent"));
    }
}