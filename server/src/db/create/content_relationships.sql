CREATE TABLE IF NOT EXISTS content_relationships (
    id TEXT PRIMARY KEY,
    source_content_id TEXT NOT NULL,
    target_content_id TEXT NOT NULL,
    relationship_type TEXT NOT NULL,
    strength REAL NOT NULL,
    context TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (source_content_id) REFERENCES extracted_content(id),
    FOREIGN KEY (target_content_id) REFERENCES extracted_content(id),
    INDEX idx_source_content (source_content_id),
    INDEX idx_target_content (target_content_id),
    INDEX idx_relationship_type (relationship_type)
);