SELECT c.*
FROM extracted_content c
    JOIN content_extraction_jobs cej ON c.id = cej.extracted_content_id
WHERE cej.source_ticket_id = ?
    AND c.content_status = 'Active'
ORDER BY c.extracted_at DESC;