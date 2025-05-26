-- Your SQL goes here
CREATE TABLE user_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id VARCHAR NOT NULL,
    action VARCHAR NOT NULL,                -- e.g., 'suggest'
    details TEXT,                           -- Additional details about the action
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);