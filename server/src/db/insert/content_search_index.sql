INSERT
    OR REPLACE INTO content_search_index (
        id,
        content_id,
        content_type,
        title_tokens,
        body_tokens,
        concept_tokens,
        author_tokens,
        full_text_search,
        embedding_vector,
        indexed_at
    )
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);