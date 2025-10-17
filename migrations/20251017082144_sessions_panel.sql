-- Add migration script here
CREATE TABLE IF NOT EXISTS sessions_panel (
    session_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    expires_at INTEGER NOT NULL,
    avatar_hash TEXT NOT NULL
);