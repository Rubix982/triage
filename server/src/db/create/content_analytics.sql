CREATE TABLE IF NOT EXISTS content_analytics (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL,
    date DATE NOT NULL,
    view_count INTEGER DEFAULT 0,
    search_hits INTEGER DEFAULT 0,
    link_clicks INTEGER DEFAULT 0,
    note_creations INTEGER DEFAULT 0,
    knowledge_impact_score REAL DEFAULT 0.0,
    
    FOREIGN KEY (content_id) REFERENCES extracted_content(id),
    
    UNIQUE(content_id, date),
    INDEX idx_date (date),
    INDEX idx_impact_score (knowledge_impact_score)
);
