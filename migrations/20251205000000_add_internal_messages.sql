-- Add is_internal field to thread_messages table to support internal staff notes
ALTER TABLE thread_messages ADD COLUMN is_internal BOOLEAN NOT NULL DEFAULT false;
