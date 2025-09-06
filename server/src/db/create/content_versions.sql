CREATE TABLE IF NOT EXISTS content_versions (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL,
    version_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    body_text TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    author TEXT,
    modified_at TEXT NOT NULL,
    change_summary TEXT,
    diff_from_previous TEXT,
    -- JSON
    FOREIGN KEY (content_id) REFERENCES extracted_content(id),
    UNIQUE(content_id, version_number),
    INDEX idx_content_version (content_id, version_number),
    INDEX idx_modified_at (modified_at)
);