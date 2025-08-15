-- Create system metadata table if missing
CREATE TABLE IF NOT EXISTS system_metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Initialize last_recovery_timestamp if absent
INSERT OR IGNORE INTO system_metadata (key, value)
VALUES ('last_recovery_timestamp', datetime('now'));

-- Close duplicate open threads per user (keep oldest)
UPDATE threads
SET status = 0
WHERE status = 1
  AND rowid NOT IN (
    SELECT MIN(rowid) FROM threads WHERE status = 1 GROUP BY user_id
);

-- Ensure a single open thread per user via partial unique index
CREATE UNIQUE INDEX IF NOT EXISTS idx_threads_unique_open_per_user
ON threads(user_id)
WHERE status = 1;
