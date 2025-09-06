CREATE TABLE IF NOT EXISTS extracted_content (
    id TEXT PRIMARY KEY,
    content_type TEXT NOT NULL,
    source_url TEXT NOT NULL UNIQUE,
    source_platform TEXT NOT NULL,
    title TEXT NOT NULL,
    body_text TEXT NOT NULL,
    raw_content TEXT,
    -- JSON
    content_hash TEXT NOT NULL,
    author TEXT,
    created_at TEXT,
    modified_at TEXT,
    extracted_at TEXT NOT NULL,
    last_updated_at TEXT NOT NULL,
    content_status TEXT NOT NULL,
    access_permissions TEXT,
    -- JSON
    metadata TEXT,
    -- JSON ContentMetadata
    -- Indexes for search
    INDEX idx_content_type (content_type),
    INDEX idx_source_platform (source_platform),
    INDEX idx_content_hash (content_hash),
    INDEX idx_author (author),
    INDEX idx_created_at (created_at)
);