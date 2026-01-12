-- Add target_roles column to reminders table for role-targeted reminders
-- Stores role_ids separated by commas (e.g., "123456,789012")
-- NULL means personal reminder (current behavior)

ALTER TABLE reminders ADD COLUMN target_roles TEXT DEFAULT NULL;
