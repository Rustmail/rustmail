-- CreateTable (idempotent)
CREATE TABLE IF NOT EXISTS "blocked_users" (
    "user_id" TEXT NOT NULL PRIMARY KEY,
    "user_name" TEXT NOT NULL,
    "blocked_by" TEXT NOT NULL,
    "blocked_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expires_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateTable (idempotent)
CREATE TABLE IF NOT EXISTS "threads" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "user_id" INTEGER NOT NULL,
    "user_name" TEXT NOT NULL,
    "channel_id" TEXT NOT NULL,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "next_message_number" INTEGER DEFAULT 1,
    "status" INTEGER NOT NULL DEFAULT 1,
    "user_left" BOOLEAN NOT NULL DEFAULT false
);

-- CreateTable (idempotent)
CREATE TABLE IF NOT EXISTS "thread_messages" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "thread_id" TEXT NOT NULL,
    "user_id" INTEGER NOT NULL,
    "user_name" TEXT NOT NULL,
    "is_anonymous" BOOLEAN NOT NULL,
    "dm_message_id" TEXT,
    "inbox_message_id" TEXT,
    "message_number" INTEGER,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "content" TEXT NOT NULL,
    "thread_status" INTEGER NOT NULL,
    CONSTRAINT "thread_messages_thread_id_fkey" FOREIGN KEY ("thread_id") REFERENCES "threads" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable (idempotent)
CREATE TABLE IF NOT EXISTS "staff_alerts" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "staff_user_id" INTEGER NOT NULL,
    "thread_user_id" INTEGER NOT NULL,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "used" BOOLEAN NOT NULL DEFAULT false
);

-- CreateIndex (idempotent)
CREATE UNIQUE INDEX IF NOT EXISTS "threads_id_key" ON "threads"("id");

-- CreateIndex (idempotent)
CREATE UNIQUE INDEX IF NOT EXISTS "thread_messages_id_key" ON "thread_messages"("id");
