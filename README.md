[discord-shield]: https://img.shields.io/discord/1407300551686885418?color=5865F2&logo=discord&logoColor=white
[discord-invite]: https://discord.gg/9Bzma6SwtW

<div align="center">
  <img src="https://github.com/Akinator31/rustmail/blob/main/docs/static/logo.svg?raw=true" width="250" alt="Rustmail Logo">
</div>

# Rustmail - Discord Modmail Bot (Rust)

---
## ⚠️ Warning ⚠️
This is my first major project in Rust; while I have solid experience in C and other languages, I'm learning Rust as I go — feedback and PRs are welcome.

---
> Status: **Private Alpha** – **NON PRODUCTION-READY** (unstable, API & schema subject to change, data loss risk). Use at your own risk.
>
> Goal: Provide a performant, extensible Rust modmail foundation with thread management, message editing, i18n, rich error system and future advanced features.

[![Discord][discord-shield]][discord-invite]

---
## ✨ Feature Summary (Currently Implemented)
- Open support ticket / staff⇄user thread (`!new_thread` or optional manual creation if enabled)
- **Single-server** or **dual-server (community + staff)** mode
- SQLite storage (threads, messages, staff alerts, blocked users)
- Incremental per-thread message numbering
- Staff replies → user DM + mirrored in thread channel
- Anonymous replies (`!anonreply` / `!ar`)
- Retroactive message editing (staff) with propagation & internal audit note (`!edit` / `!e`)
- Message deletion + renumbering (`!delete`)
- Controlled thread closing (`!close`) + forced orphan cleanup (`!force_close`)
- Thread category moving with fuzzy matching (`!move` / `!mv`)
- Personal staff alert subscription on future user activity (`!alert` / `!alert cancel`)
- Asynchronous recovery of missing messages (`!recover`)
- Typing proxy (user ↔ staff) configurable
- Configurable system messages (welcome / close)
- Configurable embed colors (user / staff / system) in hex
- Fine‑grained success/failure notification toggles
- Internationalization (multi-language + per-user preferences) – English & French shipped
- Structured error system (categorization, codes, TTL, auto-delete, translation)
- Error & language test commands (`!test_errors`, `!test_language`, `!test_all_errors`)
- In‑memory per-thread locks (basic race mitigation)
- Attachment download & relay
- Dynamic configuration validation (logs, features, Discord guild access)

## ⚠️ Limitations / Not Implemented / Known Issues
| Area                 | Limitation / Risk                                                                            |
|----------------------|----------------------------------------------------------------------------------------------|
| Production readiness | Not ready: no full security audit; no centralized fine-grained Discord role enforcement yet. |
| Permissions          | Basic staff membership inference; granular role logic incomplete.                            |
| Attachments          | No explicit size/type limits; no local cache; sequential downloads (slower).                 |
| Concurrency          | Potential races on message number allocation under high load (simple locks only).            |
| Network robustness   | No advanced retry/backoff for Discord or HTTP fetches.                                       |
| Logging              | Minimal println! usage; no structured logging levels.                                        |
| Security             | No content filtering / abuse detection.                                                      |
| DB migrations        | Partially idempotent; future changes may break backward compatibility.                       |
| Indexing             | Limited indexes (no composite on user_id/thread_id for heavier queries).                     |
| Discord sharding     | Not supported (single process / shard).                                                      |
| Horizontal scaling   | Not supported (SQLite local file, local locks).                                              |
| Monitoring           | No health checks or metrics.                                                                 |
| Tests                | Very few automated tests (only some parsing).                                                |
| Auto close           | `time_to_close_thread` unused (no scheduled job).                                            |
| User blocking        | `blocked_users` table present; no exposed commands yet.                                      |
| Features channel     | Features (e.g. poll) experimental / incomplete.                                              |

---
## 🧠 High-Level Architecture
- `main.rs`: Initialization (DB, config, Serenity client, handlers, guild validation)
- `config.rs`: Load + structural validation + dependency injection (pool, error handler)
- `handlers/`: Discord event listeners (messages, reactions, members, interactions, moderation, ready, typing proxy)
- `commands/`: Prefix command logic (manual parsing)
- `db/` & `db/operations/`: SQLx abstraction (SQLite) – threads, messages, features, alerts
- `modules/`: Functional logic (message recovery, thread helpers)
- `utils/`: Utility helpers (conversion, message builder, content extraction, time, locks)
- `i18n/`: Dictionary system + multi-language resolution + fallback
- `errors/`: Strongly typed errors, dictionary mapping → localized embeds
- `features/`: Optional / experimental modules (e.g. poll)

### Staff → User Reply Flow
1. Staff types `!reply` inside a thread channel
2. Parse content & attachments
3. Allocate message number (approx. atomic) → increment `next_message_number`
4. Send staff embed to thread + DM user
5. Persist in DB (`thread_messages`) with Discord IDs
6. Optional success notification

### Thread Closing
- `!close`: Final DM (if user still guild member), mark closed & delete channel
- `!force_close`: Delete orphaned channel OR residual untracked inbox channel

### Message Editing
- `!edit <num> <new text>`: Validate rights (author/staff), fetch IDs, edit thread + DM messages, update DB
- System message (audit) indicates change with deep link to edited message

---
## 🗃️ Database Schema (SQLite)
Primary tables:
- `threads(id TEXT PK, user_id INTEGER, user_name TEXT, channel_id TEXT, created_at, next_message_number, status, user_left)`
- `thread_messages(id INTEGER PK AUTOINC, thread_id FK, user_id, user_name, is_anonymous, dm_message_id, inbox_message_id, message_number, content, thread_status)`
- `staff_alerts(id, staff_user_id, thread_user_id, used BOOL)` – future activity alerts
- `blocked_users(user_id PK, user_name, blocked_by, blocked_at, expires_at)` (not yet exposed by commands)

Unique indexes on `threads.id`, `thread_messages.id`.

---
## ⚙️ Configuration (config.toml)
Minimal example (adjust real IDs):
```toml
[bot]
token = "YOUR_TOKEN"
status = "DM FOR SUPPORT"
welcome_message = "We received your message! A staff member will reply soon."
close_message = "Thanks for contacting support! Your ticket is now closed."
typing_proxy_from_user = true
typing_proxy_from_staff = true
enable_logs = true
logs_channel_id = 123456789012345678
enable_features = true
features_channel_id = 123456789012345678

[bot.mode]
# Single server
# type = "single"
# guild_id = 111111111111111111
# OR dual server
type = "dual"
community_guild_id = 222222222222222222
staff_guild_id = 333333333333333333

[command]
prefix = "!"

[thread]
inbox_category_id = 444444444444444444
embedded_message = true
user_message_color = "3d54ff"
staff_message_color = "ff3126"
system_message_color = "00fb3f"
block_quote = true
time_to_close_thread = 0           # (placeholder)
create_ticket_by_create_channel = false

[notifications]
show_success_on_edit = true
show_partial_success_on_edit = true
show_failure_on_edit = true
show_success_on_reply = true
show_success_on_delete = false

[language]
default_language = "en"
fallback_language = "en"
supported_languages = ["en", "fr"]
error_message_ttl = 30

[error_handling]
show_detailed_errors = false
log_errors = true
send_error_embeds = true
auto_delete_error_messages = true
error_message_ttl = 30
```

### Internal Validation
- Ensures `enable_logs` ↔ `logs_channel_id`
- Ensures `enable_features` ↔ `features_channel_id`
- Validates guild IDs (bot access) before startup
- Parses hex colors (panic if invalid)

---
## 🧩 Commands (Prefix configurable – default `!`)
General format: `!command [arguments]`

| Command              | Alias     | Description                                                                                                     | Example                    |
|----------------------|-----------|-----------------------------------------------------------------------------------------------------------------|----------------------------|
| new_thread <user_id  | @mention> | nt                                                                                                              | Create a thread for a user | `!new_thread 123456789012345678` |
| reply <text>         | r         | Reply (staff visible + DM)                                                                                      | `!reply Hello`             |
| anonreply <text>     | ar        | Anonymous reply                                                                                                 | `!ar Thanks for reporting` |
| edit <num> <new>     | e         | Edit message number N                                                                                           | `!edit 5 Correction`       |
| delete <num>         | —         | Delete message N + renumber                                                                                     | `!delete 7`                |
| close                | —         | Close current thread (DM user if still present)                                                                 | `!close`                   |
| force_close          | —         | Force delete orphan / leftover channel                                                                          | `!force_close`             |
| move <category>      | mv        | Move thread to category (fuzzy)                                                                                 | `!move Resolved`           |
| alert [cancel]       | —         | Set (or cancel) personal alert                                                                                  | `!alert` / `!alert cancel` |
| recover              | —         | Start async missing message recovery                                                                            | `!recover`                 |
| test_errors <type>   | —         | Emit test error (db, discord, command, validation, message, thread, permission, user, channel, number, success) | `!test_errors db`          |
| test_language <code> | —         | Set user language + trigger test error                                                                          | `!test_language fr`        |
| test_all_errors      | —         | Sequential demo of various errors                                                                               | `!test_all_errors`         |

Notes:
- `edit`, `delete`, and `reply` must be inside a valid thread channel.
- `move` uses Levenshtein matching (~50% distance threshold) on category names.
- `anonreply` hides staff identity from the user.
- `alert` writes `staff_alerts` entry (future trigger logic to be wired to user events).

---
## 🌍 Internationalization (i18n)
- Default + fallback language from config.
- Per-user preference via `!test_language <code>` (test command; a dedicated command could replace it later).
- Key namespaces: (reply.*, delete.*, new_thread.*, move.*, permission.*, success.*, etc.).
- Missing translation → fallback language.

### Adding a Language
1. Create file in `src/i18n/language/<code>.rs`
2. Implement dictionary similar to `en.rs`
3. Add code to `supported_languages` + enum mapping

---
## ❗ Error Handling
Categories: Database, Discord, Command, Validation, Message, Thread, Permission, User, Channel.
Mechanisms:
- Colored embeds (success / failure) + TTL (auto delete if configured)
- Dynamic translation via `ErrorHandler`
- Test commands for QA
- Differentiates silent vs informative errors

Future best practices: structured logging centralization; standardized codes.

---
## 🛠️ Install & Run
### Prerequisites
- Rust (2024 edition toolchain) – see `rust-toolchain.toml`
- SQLite library (SQLx manages access)
- Create application + bot in [Discord Developer Portal]

### Steps
```bash
git clone <repo>
cd modmail_rs
cp config.example.toml config.toml   # Edit values
cargo run --release
```

### Critical Variables
- Keep `bot.token` secret (do not commit).
- Enable required privileged intents (MESSAGE CONTENT, GUILD MEMBERS, PRESENCES) in the developer portal.

---
## 🧪 Testing / Quality
Current: limited unit tests (`edit_command.rs`).
Extension ideas:
- Integration tests (thread creation, reply, edit, delete)
- In-memory DB fixture / temporary SQLite file

Quick run:
```bash
cargo test
```

---
## 🔐 Security / Permissions (Current Partial State)
- Staff detection mostly implicit (e.g. staff guild presence; granular role enforcement to improve).
- Recommendation: restrict inbox category access via Discord roles.
- No exhaustive sanitization audit (user content stored verbatim).

Security roadmap:
- Configurable role matrix per command
- Rate limiting
- Dangerous attachment filtering

---
## 🚀 Roadmap
Not yet determined. See GitHub Project.

---
## 🧾 Message Conventions
- Staff → `staff_message_color`
- User → `user_message_color`
- System / success / errors → `system_message_color` (or derivative)
- `block_quote = true` applies quoted styling depending on builder implementation

---
## 🔄 Thread Lifecycle
Implicit states (`status` int): open, closed (exact mapping TBD).
Fields:
- `next_message_number` starting at 1
- `user_left` updated when user leaves (used by force_close / close logic)

---
## 🧪 Debug Commands
- `!test_errors`: error catalog
- `!test_language`: force language + test
- `!test_all_errors`: timed sequential demonstration

---
## ❓ Preliminary FAQ
Q: Why SQLite?
A: Fast prototyping. Upgrade to Postgres later for concurrency / scaling.

Q: Why do some commands seem silent?
A: Notification toggles may suppress confirmation messages.

Q: What if a DM fails to send?
A: The bot posts a system message "send_failed_dm" in the thread.

---
## 🧹 Maintenance & Housekeeping
- Backups: copy `db/db.sqlite`
- New migration → restart binary (sqlx applies at startup via `init_database()`)
- Logs: currently stdout/stderr (improve later)

---
## ⚖️ License
MIT. See LICENSE file.

---
## 🤝 Contributions
Alpha phase: open descriptive issues (bugs, UX). PRs accepted after discussion.

---
## ⚠️ Final Disclaimer
Project is in **alpha**. APIs, structures, schemas and behaviors may change without backward compatibility. Do not use in critical environments or with sensitive data. Make frequent backups.

---

Made with Rust 🦀 – contributions welcome.
