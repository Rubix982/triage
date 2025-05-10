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
    self_link TEXT,
    summary TEXT,
    status TEXT,
    created TEXT,
    updated TEXT,
    rendered_fields TEXT,
    names TEXT,
    schema TEXT,
    transitions TEXT,
    edit_meta TEXT,
    changelog TEXT,
    versioned_representations TEXT,
    watcher TEXT,
    attachment TEXT,
    sub_tasks TEXT,
    description TEXT,
    project TEXT,
    comment TEXT,
    issue_links TEXT,
    work_log TEXT,
    time_tracking TEXT,
);
"#;

// DML
pub const INSERT_PROJECT: &str = r#"
INSERT INTO projects (id, key, name) VALUES (?, ?, ?);
"#;

pub const INSERT_ISSUE_METADATA: &str = r#"
INSERT OR REPLACE INTO issues
(id, key, self_link, summary, status, 
created, updated, rendered_fields, names, schema,
transitions, edit_meta, changelog, versioned_representations, watcher,
attachment, sub_tasks, description, project, comment,
issue_links, work_log, time_tracking)
VALUES
(?, ?, ?, ?, ?,
?, ?, ?, ?, ?,
?, ?, ?, ?, ?,
?, ?, ?, ?, ?,
?, ?, ?)
"#;

pub const GET_PROJECT_IDS: &str = r#"
SELECT id FROM projects;
"#;
