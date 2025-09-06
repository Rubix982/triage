INSERT
    OR REPLACE INTO content_extraction_jobs (
        id,
        source_ticket_id,
        source_url,
        platform_type,
        user_id,
        team_id,
        priority,
        status,
        retry_count,
        created_at,
        started_at,
        completed_at,
        error_message,
        extracted_content_id
    )
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);