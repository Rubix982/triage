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
    issue_type TEXT,
    issue_type_id TEXT,
    is_subtask BOOLEAN,
    hierarchy_level INTEGER,
    priority TEXT,
    priority_id TEXT,
    assignee TEXT,
    reporter TEXT,
    labels TEXT, -- JSON array as string
    created TEXT,
    updated TEXT,
    project_name TEXT,
    project_key TEXT,
    extracted_links TEXT, -- JSON array as string
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
    time_tracking TEXT
);
"#;

// DML
pub const INSERT_PROJECT: &str = r#"
INSERT INTO projects (id, key, name) VALUES (?, ?, ?);
"#;

pub const INSERT_ISSUE_METADATA: &str = r#"
INSERT OR REPLACE INTO issues
(id, key, self_link, summary, status, 
issue_type, issue_type_id, is_subtask, hierarchy_level, priority,
priority_id, assignee, reporter, labels, created, 
updated, project_name, project_key, extracted_links, rendered_fields, 
names, schema, transitions, edit_meta, changelog, 
versioned_representations, watcher, attachment, sub_tasks, description, 
project, comment, issue_links, work_log, time_tracking)
VALUES
(?, ?, ?, ?, ?,
?, ?, ?, ?, ?,
?, ?, ?, ?, ?,
?, ?, ?, ?, ?,
?, ?, ?, ?, ?,
?, ?, ?, ?, ?,
?, ?, ?, ?, ?)
"#;

pub const GET_PROJECT_IDS: &str = r#"
SELECT id FROM projects;
"#;
