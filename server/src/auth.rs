use crate::types::Config;
use base64::engine::general_purpose;
use base64::Engine;
use dirs::home_dir;
use inquire::Text;
use std::fs::{self, create_dir_all};
use std::path::PathBuf;

fn get_config_path() -> PathBuf {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".triage");
    if !path.exists() {
        create_dir_all(&path).expect("Failed to create .triage directory");
    }
    path.push(".triage-config.json");
    path
}

pub async fn authenticate() -> String {
    let config_path = get_config_path();

    if !config_path.exists() {
        let domain = Text::new("Enter your JIRA domain (e.g., your-domain.atlassian.net):")
            .prompt()
            .expect("Failed to read domain");
        let email = Text::new("Enter your user email")
            .prompt()
            .expect("Failed to read the email");
        let mut token = Text::new("Enter your JIRA token:")
            .prompt()
            .expect("Failed to read token");

        token = general_purpose::STANDARD.encode(format!("{}:{}", email, token));
        let config = Config { domain, token };
        let json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&config_path, json).expect("Failed to write config file");
    }

    let data = fs::read_to_string(&config_path).expect("Could not read config file");
    let config: Config = serde_json::from_str(&data).expect("Invalid config format");
    config.token
}

pub fn get_domain() -> String {
    let config_path = get_config_path();
    let data = fs::read_to_string(&config_path).expect("Could not read config file");
    let config: Config = serde_json::from_str(&data).expect("Invalid config format");
    config.domain
}
