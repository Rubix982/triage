INSERT
    OR REPLACE INTO extracted_content (
        id,
        content_type,
        source_url,
        source_platform,
        title,
        body_text,
        raw_content,
        content_hash,
        author,
        created_at,
        modified_at,
        extracted_at,
        last_updated_at,
        content_status,
        access_permissions,
        metadata
    )
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);