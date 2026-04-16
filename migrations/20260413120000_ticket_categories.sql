-- Ticket categories feature: user-selectable categories at ticket creation

CREATE TABLE IF NOT EXISTS ticket_categories (
    id                  TEXT PRIMARY KEY,
    name                TEXT NOT NULL,
    description         TEXT,
    emoji               TEXT,
    discord_category_id TEXT NOT NULL,
    position            INTEGER NOT NULL DEFAULT 0,
    enabled             INTEGER NOT NULL DEFAULT 1,
    created_at          INTEGER NOT NULL,
    updated_at          INTEGER NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_ticket_categories_name_unique
    ON ticket_categories(name COLLATE NOCASE);

CREATE INDEX IF NOT EXISTS idx_ticket_categories_enabled
    ON ticket_categories(enabled, position);

CREATE TABLE IF NOT EXISTS ticket_category_settings (
    id                  INTEGER PRIMARY KEY CHECK (id = 1),
    enabled             INTEGER NOT NULL DEFAULT 0,
    selection_timeout_s INTEGER NOT NULL DEFAULT 300
);

INSERT OR IGNORE INTO ticket_category_settings (id, enabled, selection_timeout_s)
VALUES (1, 0, 300);

CREATE TABLE IF NOT EXISTS pending_category_selections (
    user_id        INTEGER PRIMARY KEY,
    prompt_msg_id  TEXT NOT NULL,
    dm_channel_id  TEXT NOT NULL,
    started_at     INTEGER NOT NULL,
    expires_at     INTEGER NOT NULL,
    queued_msg_ids TEXT NOT NULL DEFAULT '[]'
);

CREATE INDEX IF NOT EXISTS idx_pending_category_selections_expires_at
    ON pending_category_selections(expires_at);

ALTER TABLE threads ADD COLUMN ticket_category_id TEXT;
