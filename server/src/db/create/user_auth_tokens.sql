CREATE TABLE IF NOT EXISTS user_auth_tokens (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    platform TEXT NOT NULL,
    team_id TEXT,
    access_token_encrypted TEXT NOT NULL,
    refresh_token_encrypted TEXT,
    token_expires_at TEXT,
    scopes TEXT,
    -- JSON array
    created_at TEXT NOT NULL,
    last_used_at TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    INDEX idx_user_platform (user_id, platform),
    INDEX idx_team (team_id),
    INDEX idx_active (is_active)
);