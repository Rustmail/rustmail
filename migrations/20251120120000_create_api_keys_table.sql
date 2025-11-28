-- Add migration script here
CREATE TABLE IF NOT EXISTS api_keys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key_hash TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    permissions TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    expires_at INTEGER,
    last_used_at INTEGER,
    is_active INTEGER NOT NULL DEFAULT 1
);

-- Index for faster lookups by hash (used on every API request)
CREATE INDEX IF NOT EXISTS idx_api_keys_hash ON api_keys(key_hash);

-- Index for active keys only
CREATE INDEX IF NOT EXISTS idx_api_keys_active ON api_keys(is_active);
