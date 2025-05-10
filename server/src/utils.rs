use colored::*;
use serde_json::Value;

pub fn get_optional_field<T: for<'de> serde::Deserialize<'de>>(
    map: &serde_json::Map<String, Value>,
    key: &str,
) -> Option<T> {
    map.get(key)
        .filter(|v| !v.is_null())
        .and_then(|v| serde_json::from_value(v.clone()).ok())
}

pub fn extract_json_field_as_string(body: &Value, key: &str) -> String {
    body.get(key)
        .map(|v| serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()))
        .unwrap_or_else(|| "{}".to_string())
}

pub fn json_opt_to_string(value: &Option<serde_json::Value>) -> String {
    value.as_ref().map(|v| v.to_string()).unwrap_or_default()
}

pub fn log_step(icon: &str, msg: &str) {
    println!("{} {}", icon.bright_yellow(), msg.yellow());
}

pub fn log_success(msg: &str) {
    println!("{} {}", "✅".green(), msg.green());
}

pub fn log_error(context: &str) -> String {
    format!("❌ [{}]", context)
}
