-- Create features messages table to track posted feature messages
CREATE TABLE IF NOT EXISTS features_messages (
    feature_key TEXT PRIMARY KEY,
    channel_id TEXT NOT NULL,
    message_id TEXT NOT NULL,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

