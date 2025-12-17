# Database Reference

Rustmail uses SQLite for persistent storage. The database file `db.sqlite` is created automatically on first run.

---

## Overview

The database stores:
- Ticket threads and messages
- Staff alerts and reminders
- Scheduled closures
- Panel sessions and permissions
- API keys
- Snippets

---

## Tables

### threads

Stores ticket information.

| Column | Type | Description |
|--------|------|-------------|
| `id` | TEXT | Primary key (UUID) |
| `user_id` | INTEGER | Discord user ID |
| `user_name` | TEXT | Username at ticket creation |
| `channel_id` | TEXT | Discord channel ID |
| `created_at` | DATETIME | Ticket creation timestamp |
| `next_message_number` | INTEGER | Counter for message numbering |
| `status` | INTEGER | Ticket status (1=open, 0=closed) |
| `user_left` | BOOLEAN | Whether user left the server |
| `closed_at` | DATETIME | Closure timestamp (nullable) |
| `closed_by` | TEXT | Staff who closed (nullable) |
| `category_id` | TEXT | Current category ID (nullable) |
| `category_name` | TEXT | Current category name (nullable) |
| `required_permissions` | TEXT | Permission requirements (nullable) |

### thread_messages

Stores all messages in tickets.

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Primary key (auto-increment) |
| `thread_id` | TEXT | Foreign key to threads |
| `user_id` | INTEGER | Author's Discord ID |
| `user_name` | TEXT | Author's username |
| `is_anonymous` | BOOLEAN | Whether sent anonymously |
| `dm_message_id` | TEXT | Discord message ID in DM |
| `inbox_message_id` | TEXT | Discord message ID in ticket channel |
| `message_number` | INTEGER | Sequential message number |
| `created_at` | DATETIME | Message timestamp |
| `content` | TEXT | Message content |
| `thread_status` | INTEGER | Thread status when sent |

### blocked_users

Stores blocked users who cannot create tickets.

| Column | Type | Description |
|--------|------|-------------|
| `user_id` | TEXT | Primary key (Discord user ID) |
| `user_name` | TEXT | Username when blocked |
| `blocked_by` | TEXT | Staff who blocked |
| `blocked_at` | DATETIME | Block timestamp |
| `expires_at` | DATETIME | Block expiration |

### staff_alerts

Stores alert subscriptions for tickets.

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Primary key |
| `staff_user_id` | INTEGER | Staff Discord ID |
| `thread_user_id` | INTEGER | Ticket user Discord ID |
| `created_at` | DATETIME | Alert creation time |
| `used` | BOOLEAN | Whether alert was triggered |

### reminders

Stores scheduled reminders.

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Primary key |
| `thread_id` | TEXT | Foreign key to threads |
| `user_id` | BIGINT | Staff Discord ID |
| `channel_id` | BIGINT | Channel Discord ID |
| `guild_id` | BIGINT | Server Discord ID |
| `reminder_content` | TEXT | Reminder message |
| `trigger_time` | INTEGER | Unix timestamp to trigger |
| `created_at` | INTEGER | Creation Unix timestamp |
| `completed` | BOOLEAN | Whether reminder fired |

### scheduled_closures

Stores scheduled ticket closures.

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Primary key |
| `thread_id` | TEXT | Foreign key to threads |
| `scheduled_time` | INTEGER | Unix timestamp for closure |
| `silent` | BOOLEAN | Close without notification |
| `created_by` | TEXT | Staff who scheduled |

### snippets

Stores saved response templates.

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Primary key |
| `key` | TEXT | Unique snippet identifier |
| `content` | TEXT | Snippet text |
| `created_by` | TEXT | Creator Discord ID |
| `created_at` | DATETIME | Creation timestamp |
| `updated_at` | DATETIME | Last update timestamp |

### sessions_panel

Stores web panel sessions.

| Column | Type | Description |
|--------|------|-------------|
| `session_id` | TEXT | Primary key (session token) |
| `user_id` | TEXT | Discord user ID |
| `access_token` | TEXT | Discord OAuth2 access token |
| `refresh_token` | TEXT | Discord OAuth2 refresh token |
| `expires_at` | INTEGER | Session expiration Unix timestamp |
| `avatar_hash` | TEXT | User's avatar hash |

### api_keys

Stores API keys for external access.

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Primary key |
| `key_hash` | TEXT | Hashed API key (unique) |
| `name` | TEXT | Key description |
| `permissions` | TEXT | JSON array of permissions |
| `created_at` | INTEGER | Creation Unix timestamp |
| `expires_at` | INTEGER | Expiration timestamp (nullable) |
| `last_used_at` | INTEGER | Last usage timestamp (nullable) |
| `is_active` | INTEGER | Whether key is active |

### panel_permissions

Stores granted panel permissions.

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Primary key |
| `subject_type` | TEXT | "user" or "role" |
| `subject_id` | TEXT | Discord user/role ID |
| `permission` | TEXT | Permission name |
| `granted_by` | TEXT | Who granted it |
| `granted_at` | INTEGER | Grant Unix timestamp |

### features_messages

Stores feature request tracking.

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Primary key |
| `message_id` | TEXT | Discord message ID |
| `content` | TEXT | Feature description |

### thread_status

Stores thread status history.

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Primary key |
| `thread_id` | TEXT | Foreign key to threads |
| `status` | INTEGER | Status value |
| `changed_at` | DATETIME | Change timestamp |

---

## Indexes

Performance indexes on frequently queried columns:

- `threads_id_key` on `threads(id)`
- `thread_messages_id_key` on `thread_messages(id)`
- `idx_api_keys_hash` on `api_keys(key_hash)`
- `idx_api_keys_active` on `api_keys(is_active)`
- `idx_snippets_key` on `snippets(key)`
- `idx_panel_perms_subject` on `panel_permissions(subject_type, subject_id)`
- `idx_panel_perms_permission` on `panel_permissions(permission)`

---

## Migrations

Database schema is managed through SQLx migrations in the `migrations/` directory. Migrations run automatically on bot startup.

Migration files are named with timestamps:
```
migrations/
├── 20250815145017_create_tables.sql
├── 20250815161000_unique_open_and_metadata.sql
├── 20250816120000_message_number_unique_and_cleanup.sql
└── ...
```

---

## Backup

The database is a single file (`db.sqlite`). To backup:

```bash
# Stop the bot first for consistency
cp db.sqlite db.sqlite.backup
```

For production, consider scheduled backups:

```bash
# Example cron job (daily at 3 AM)
0 3 * * * cp /opt/rustmail/db.sqlite /backups/rustmail-$(date +\%Y\%m\%d).sqlite
```

---

## Direct Access

You can query the database directly with SQLite tools:

```bash
sqlite3 db.sqlite

# Example queries
sqlite3 db.sqlite "SELECT COUNT(*) FROM threads WHERE status = 1;"
sqlite3 db.sqlite "SELECT * FROM threads ORDER BY created_at DESC LIMIT 10;"
```

**Warning:** Avoid modifying data while the bot is running to prevent corruption.

---

## Data Retention

Rustmail does not automatically delete old data. For compliance or storage management, you may need to implement your own retention policies:

```sql
-- Example: Delete closed tickets older than 1 year
DELETE FROM thread_messages
WHERE thread_id IN (
  SELECT id FROM threads
  WHERE status = 0
  AND closed_at < datetime('now', '-1 year')
);

DELETE FROM threads
WHERE status = 0
AND closed_at < datetime('now', '-1 year');
```
