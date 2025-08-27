-- Create scheduled closures table
CREATE TABLE IF NOT EXISTS scheduled_closures (
    thread_id TEXT PRIMARY KEY,
    close_at INTEGER NOT NULL, -- unix epoch seconds
    silent BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (thread_id) REFERENCES threads(id) ON DELETE CASCADE ON UPDATE CASCADE
);

-- Index for ordering by time if needed
CREATE INDEX IF NOT EXISTS idx_scheduled_closures_close_at ON scheduled_closures(close_at);

