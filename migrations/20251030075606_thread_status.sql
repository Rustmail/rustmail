-- Add migration script here
CREATE TABLE IF NOT EXISTS "thread_status" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "thread_id" TEXT NOT NULL UNIQUE,
    "channel_id" INTEGER NOT NULL UNIQUE,
    "owner_id" TEXT NOT NULL,
    "taken_by" TEXT DEFAULT NULL,
    "last_message_by" TEXT NOT NULL,
    "last_message_at" INTEGER NOT NULL,
    CONSTRAINT "thread_status_thread_id_fkey" FOREIGN KEY ("thread_id") REFERENCES "threads" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);