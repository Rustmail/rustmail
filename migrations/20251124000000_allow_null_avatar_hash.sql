-- Allow NULL in avatar_hash column for users without Discord avatars
-- SQLite doesn't support ALTER COLUMN, so we need to recreate the table
CREATE TABLE IF NOT EXISTS sessions_panel_new (
    session_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    expires_at INTEGER NOT NULL,
    avatar_hash TEXT
);

-- Copy existing data
INSERT INTO sessions_panel_new (session_id, user_id, access_token, refresh_token, expires_at, avatar_hash)
SELECT session_id, user_id, access_token, refresh_token, expires_at, avatar_hash
FROM sessions_panel;

-- Drop old table and rename new one
DROP TABLE sessions_panel;
ALTER TABLE sessions_panel_new RENAME TO sessions_panel;
