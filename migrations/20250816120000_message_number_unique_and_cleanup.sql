-- Déduplication des numéros de messages dupliqués (on garde le plus ancien)
UPDATE thread_messages
SET message_number = NULL
WHERE message_number IS NOT NULL
  AND rowid NOT IN (
    SELECT MIN(rowid)
    FROM thread_messages
    WHERE message_number IS NOT NULL
    GROUP BY thread_id, message_number
  );

-- Index unique partiel pour garantir l’unicité (thread_id, message_number) quand message_number n’est pas NULL
CREATE UNIQUE INDEX IF NOT EXISTS idx_thread_messages_unique_num
ON thread_messages(thread_id, message_number)
WHERE message_number IS NOT NULL;
