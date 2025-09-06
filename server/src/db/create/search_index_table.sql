CREATE TABLE IF NOT EXISTS content_search_index (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL,
    content_type TEXT NOT NULL,
    title_tokens TEXT,
    -- JSON array
    body_tokens TEXT,
    -- JSON array  
    concept_tokens TEXT,
    -- JSON array
    author_tokens TEXT,
    -- JSON array
    full_text_search TEXT NOT NULL,
    embedding_vector TEXT,
    -- JSON array of floats
    indexed_at TEXT NOT NULL,
    FOREIGN KEY (content_id) REFERENCES extracted_content(id),
    -- Full-text search index
    INDEX idx_full_text (full_text_search),
    INDEX idx_content_type_search (content_type),
    INDEX idx_indexed_at (indexed_at)
);