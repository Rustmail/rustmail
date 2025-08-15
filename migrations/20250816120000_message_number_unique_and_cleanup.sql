UPDATE thread_messages
SET message_number = NULL
WHERE message_number IS NOT NULL
  AND rowid NOT IN (
    SELECT MIN(rowid)
    FROM thread_messages
    WHERE message_number IS NOT NULL
    GROUP BY thread_id, message_number
  );

CREATE UNIQUE INDEX IF NOT EXISTS idx_thread_messages_unique_num
ON thread_messages(thread_id, message_number)
WHERE message_number IS NOT NULL;
