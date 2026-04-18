CREATE TABLE IF NOT EXISTS "tracked_members" (
    "guild_id" TEXT NOT NULL,
    "user_id" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "global_name" TEXT DEFAULT NULL,
    "nickname" TEXT DEFAULT NULL,
    "avatar_url" TEXT DEFAULT NULL,
    "roles" TEXT NOT NULL DEFAULT '[]',
    "joined_at" INTEGER DEFAULT NULL,
    "first_seen_at" INTEGER NOT NULL,
    "last_seen_at" INTEGER NOT NULL,
    PRIMARY KEY ("guild_id", "user_id")
);

CREATE TABLE IF NOT EXISTS "banned_users" (
    "guild_id" TEXT NOT NULL,
    "user_id" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "global_name" TEXT DEFAULT NULL,
    "nickname" TEXT DEFAULT NULL,
    "avatar_url" TEXT DEFAULT NULL,
    "roles" TEXT NOT NULL DEFAULT '[]',
    "joined_at" INTEGER DEFAULT NULL,
    "banned_at" INTEGER NOT NULL,
    "banned_by" TEXT DEFAULT NULL,
    "ban_reason" TEXT DEFAULT NULL,
    "roles_unknown" INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY ("guild_id", "user_id")
);

CREATE INDEX IF NOT EXISTS "idx_banned_users_username" ON "banned_users" ("username");
CREATE INDEX IF NOT EXISTS "idx_tracked_members_username" ON "tracked_members" ("username");
