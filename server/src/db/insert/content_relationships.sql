INSERT
    OR REPLACE INTO content_relationships (
        id,
        source_content_id,
        target_content_id,
        relationship_type,
        strength,
        context,
        created_at
    )
VALUES (?, ?, ?, ?, ?, ?, ?);