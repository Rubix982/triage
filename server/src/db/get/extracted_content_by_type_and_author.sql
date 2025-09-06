SELECT *
FROM extracted_content
WHERE content_type = ?
    AND author LIKE ?
    AND content_status = 'Active'
ORDER BY last_updated_at DESC
LIMIT ?;