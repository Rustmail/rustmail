[discord-shield]: https://img.shields.io/discord/1407300551686885418?color=5865F2&logo=discord&logoColor=white
[discord-invite]: https://discord.gg/9Bzma6SwtW

<div align="center">
  <img src="https://github.com/Akinator31/rustmail/blob/main/docs/static/logo.svg?raw=true" width="250" alt="Rustmail Logo">
</div>

# Rustmail - Discord Modmail Bot (Rust)

[ ![discord-shield][] ][discord-invite]

---
## ⚠️ Warning ⚠️
This is my first major project in Rust; while I have solid experience in C and other languages, I'm learning Rust as I go — feedback and PRs are welcome.

Project documentation is currently being written.

---
> Status: **Public Alpha** – **NON PRODUCTION-READY** (unstable, API & schema subject to change, data loss risk). Use at your own risk.
>
> Goal: Provide a performant, extensible Rust modmail foundation with thread management, message editing, i18n, rich error system and future advanced features.

---
## Feature Summary (Currently Implemented)
- Open support ticket / staff⇄user thread (`!new_thread` or optional manual creation if enabled)
- **Single-server** or **dual-server (community + staff)** mode
- SQLite storage (threads, messages, staff alerts, blocked users)
- Incremental per-thread message numbering
- Staff replies → user DM + mirrored in thread channel
- Anonymous replies (`!anonreply` / `!ar`)
- Retroactive message editing (staff) with propagation & internal audit note (`!edit` / `!e`)
- Message deletion + renumbering (`!delete`)
- Controlled thread closing (`!close`) + forced orphan cleanup (`!force_close`)
- Scheduled closing with cancel and silent modes (see Thread Closing)
- Thread category moving with fuzzy matching (`!move` / `!mv`)
- Add/remove staff participants to a ticket (`!add_staff`, `!remove_staff`)
- Staff alert subscription with ping on next user activity (mentions + auto-clear on use)
- User/server membership awareness (e.g., user left server / not in community → system notice)
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

---
## Architecture
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
- Scheduled closing with delay and optional silent mode (no user DM)
- `!close cancel`: Cancel a scheduled closing (prevents pending closure)
- `!force_close`: Delete orphaned channel OR residual untracked inbox channel

### Message Editing
- `!edit <num> <new text>`: Validate rights (author/staff), fetch IDs, edit thread + DM messages, update DB
- System message (audit) indicates change with deep link to edited message

---
## Configuration (config.toml)
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
enable_features = true
logs_channel_id = 1404359738566133899
features_channel_id = 1406940454859309076

[bot.mode]
type = "dual"
community_guild_id = 1209667980506892590
staff_guild_id = 711880297245311856

[command]
prefix = "!"

[thread]
inbox_category_id = 1376460196847505960
embedded_message = true
user_message_color = "3d54ff"
staff_message_color = "ff3126"
system_message_color = "00ff00"
block_quote = true
time_to_close_thread = 5
create_ticket_by_create_channel = true

[notifications]
show_success_on_edit = false
show_partial_success_on_edit = true
show_failure_on_edit = true
show_success_on_reply = false
show_success_on_delete = false

[logs]
show_log_on_edit = true

[language]
default_language = "en"
auto_detect = true
fallback_language = "en"
supported_languages = ["en", "fr", "es", "de"]

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
## Commands (Prefix configurable – default `!`)
General format: `!command [arguments]`

| Command                          | Alias | Description                                                                                                     | Example                          |
|----------------------------------|-------|-----------------------------------------------------------------------------------------------------------------|----------------------------------|
| new_thread <user_id \| @mention> | nt    | Create a thread for a user                                                                                      | `!new_thread 123456789012345678` |
| reply <text>                     | r     | Reply (staff visible + DM)                                                                                      | `!reply Hello`                   |
| anonreply <text>                 | ar    | Anonymous reply                                                                                                 | `!ar Thanks for reporting`       |
| edit <num> <new>                 | e     | Edit message number N                                                                                           | `!edit 5 Correction`             |
| delete <num>                     | —     | Delete message N + renumber                                                                                     | `!delete 7`                      |
| add_staff <@user>                | —     | Add a staff member to the current ticket                                                                        | `!add_staff @Moderator`          |
| remove_staff <@user>             | —     | Remove a staff member from the current ticket                                                                   | `!remove_staff @Moderator`       |
| id [@user]                       | —     | Show the numeric ID of a user (defaults to author if omitted)                                                   | `!id @User`                      |
| move <category>                  | mv    | Move thread to category (fuzzy)                                                                                 | `!move Resolved`                 |
| alert [cancel]                   | —     | Set (or cancel) personal alert                                                                                  | `!alert` / `!alert cancel`       |
| recover                          | —     | Start async missing message recovery                                                                            | `!recover`                       |
| close                            | —     | Close current thread (DM user if still present)                                                                 | `!close`                         |
| close cancel                     | —     | Cancel a scheduled closing                                                                                      | `!close cancel`                  |
| force_close                      | —     | Force delete orphan / leftover channel                                                                          | `!force_close`                   |
| test_errors <type>               | —     | Emit test error (db, discord, command, validation, message, thread, permission, user, channel, number, success) | `!test_errors db`                |
| test_language <code>             | —     | Set user language + trigger test error                                                                          | `!test_language fr`              |
| test_all_errors                  | —     | Sequential demo of various errors                                                                               | `!test_all_errors`               |

Notes:
- `edit`, `delete`, and `reply` must be inside a valid thread channel.
- `move` uses Levenshtein matching (~50% distance threshold) on category names.
- `anonreply` hides staff identity from the user.
- Alerts: subscribed staff are mentioned when the user sends a new message; the subscription is marked as used afterward.
- System notices indicate if a user has left the server or isn’t in the community guild.

---
## Internationalization (i18n)
- Default + fallback language from config.
- Per-user preference via `!test_language <code>` (test command; a dedicated command could replace it later).
- Key namespaces: (reply.*, delete.*, new_thread.*, move.*, permission.*, success.*, close.*, alert.*, server.*, user.*, etc.).
- Missing translation → fallback language.

### Adding a Language
1. Create file in `src/i18n/language/<code>.rs`
2. Implement dictionary similar to `en.rs`
3. Add code to `supported_languages` + enum mapping

---
## Install & Run
### Prerequisites
- Rust (2024 edition toolchain) – see `rust-toolchain.toml`
- SQLite library (SQLx manages access)
- Create application + bot in [Discord Developer Portal]

### Steps
```bash
git clone https://github.com/Akinator31/rustmail.git
cd rustmail
cp config.example.toml config.toml   # Edit values
cargo run --release
```

### Critical Variables
- Keep `bot.token` secret (do not commit).
- Enable required privileged intents (MESSAGE CONTENT, GUILD MEMBERS, PRESENCES) in the developer portal.

---
## Security / Permissions (Current Partial State)
- Staff detection mostly implicit (e.g. staff guild presence; granular role enforcement to improve).
- Recommendation: restrict inbox category access via Discord roles.
- No exhaustive sanitization audit (user content stored verbatim).

Security roadmap:
- Configurable role matrix per command
- Rate limiting
- Dangerous attachment filtering

---
## Roadmap
Not yet determined. See GitHub Project.

---
## Message Conventions
- Staff → `staff_message_color`
- User → `user_message_color`
- System / success / errors → `system_message_color` (or derivative)
- `block_quote = true` applies quoted styling depending on builder implementation

---
## Maintenance
- Backups: copy `db/db.sqlite`
- New migration → restart binary (sqlx applies at startup via `init_database()`)
- Logs: currently stdout/stderr (improve later)

---
## License
MIT. See LICENSE file.

---
## Contributions
Alpha phase: open descriptive issues (bugs, UX). PRs accepted after discussion.

---
## ⚠️ Final Disclaimer
Project is in **alpha**. APIs, structures, schemas and behaviors may change without backward compatibility. Do not use in critical environments or with sensitive data. Make frequent backups.

---

Made with Rust 🦀 – contributions welcome.