// DDL
pub const CREATE_PROJECT_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    key TEXT,
    name TEXT
);
"#;

pub const CREATE_ISSUES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS issues (
    id TEXT PRIMARY KEY,
    key TEXT,
    summary TEXT,
    description TEXT,
    status TEXT,
    created TEXT,
    updated TEXT
);
"#;

// DML
pub const INSERT_PROJECT: &str = r#"
INSERT INTO projects (id, key, name) VALUES (?, ?, ?);
"#;

pub const INSERT_ISSUES: &str = r#"
INSERT OR REPLACE INTO issues
(id, key, summary, description, status, created, updated)
VALUES (?, ?, ?, ?, ?, ?, ?)
"#;

pub const GET_PROJECT_IDS: &str = r#"
SELECT id FROM projects;
"#;
