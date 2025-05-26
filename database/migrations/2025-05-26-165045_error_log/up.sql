-- Your SQL goes here
CREATE TABLE error_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    error_message TEXT NOT NULL,
    error_code TEXT NOT NULL,
    stack_trace TEXT,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);