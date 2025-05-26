-- Your SQL goes here
CREATE TABLE moderation (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    moderation_type VARCHAR NOT NULL,     -- e.g., 'accept', 'reject', 'delete'
    kind VARCHAR NOT NULL,                -- 'truth' or 'dare'
    item_id INTEGER NOT NULL,
    moderator_id VARCHAR NOT NULL,
    reason TEXT,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
