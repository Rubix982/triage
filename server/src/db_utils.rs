use crate::utils::log_error;
use colored::*;
use dirs::home_dir;
use duckdb::{Connection, Result as DuckResult, Transaction};
use once_cell::sync::Lazy;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

static IS_CONNECTION_MSG_LOGGED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

fn get_db_path() -> PathBuf {
    let mut dir = home_dir().expect("❌ Could not find home directory");
    dir.push(".triage");
    if !dir.exists() {
        fs::create_dir_all(&dir).expect("❌ Failed to create .triage dir");
    }
    dir.push("triage.duckdb");
    dir
}

pub fn get_connection() -> DuckResult<Connection> {
    let db_path = get_db_path();
    let mut is_connection_msg_logged = IS_CONNECTION_MSG_LOGGED.lock().unwrap();
    if !*is_connection_msg_logged {
        println!(
            "{} {}",
            "🔌 Connecting to DuckDB at:".bright_black(),
            db_path.display()
        );
        *is_connection_msg_logged = true;
    }
    Connection::open(db_path)
}

pub fn with_connection<F>(context: &'static str, f: F)
where
    F: FnOnce(Connection),
{
    f(get_connection().unwrap_or_else(|_| panic!("{} DB connection failed", log_error(context))));
}

pub fn with_transaction<F>(context: &'static str, f: F)
where
    F: FnOnce(&Transaction),
{
    let mut conn =
        get_connection().unwrap_or_else(|_| panic!("{} DB connection failed", log_error(context)));
    let tx = conn
        .transaction()
        .unwrap_or_else(|_| panic!("{} Failed to start transaction", log_error(context)));
    f(&tx);
    tx.commit()
        .unwrap_or_else(|_| panic!("{} Failed to commit transaction", log_error(context)));
}
