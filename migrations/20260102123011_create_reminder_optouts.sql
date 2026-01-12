-- Create reminder_optouts table for storing users who opted out of role-based reminders

CREATE TABLE IF NOT EXISTS reminder_optouts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE(guild_id, user_id, role_id)
);

CREATE INDEX idx_reminder_optouts_guild_role ON reminder_optouts(guild_id, role_id);
