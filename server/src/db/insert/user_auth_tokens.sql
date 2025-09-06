INSERT
    OR REPLACE INTO user_auth_tokens (
        id,
        user_id,
        platform,
        team_id,
        access_token_encrypted,
        refresh_token_encrypted,
        token_expires_at,
        scopes,
        created_at,
        last_used_at,
        is_active
    )
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)