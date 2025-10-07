-- Create reminders table

CREATE TABLE IF NOT EXISTS reminders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    thread_id TEXT NOT NULL,
    user_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    reminder_content TEXT NOT NULL,
    trigger_time INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    completed BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (thread_id) REFERENCES threads(id) ON DELETE CASCADE ON UPDATE CASCADE
);