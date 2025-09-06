SELECT c.*,
    cr.relationship_type,
    cr.strength
FROM extracted_content c
    JOIN content_relationships cr ON c.id = cr.target_content_id
WHERE cr.source_content_id = ?
ORDER BY cr.strength DESC
LIMIT ?;