SELECT c.*,
    si.full_text_search
FROM extracted_content c
    JOIN content_search_index si ON c.id = si.content_id
WHERE si.full_text_search LIKE ?
    AND c.content_status = 'Active'
ORDER BY c.last_updated_at DESC
LIMIT ?;