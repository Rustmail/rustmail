# Architecture Overview

This document describes the technical architecture of Rustmail.

---

## Project Structure

Rustmail is a Rust workspace with three crates:

```
rustmail/
├── rustmail/           # Main bot application
├── rustmail_panel/     # Web panel (Yew/WASM)
├── rustmail_types/     # Shared type definitions
├── migrations/         # SQLite migrations
├── docs/               # Documentation
├── Cargo.toml          # Workspace manifest
└── Dockerfile
```

---

## Crates

### rustmail (Main Bot)

The core Discord bot application.

**Key dependencies:**
- `serenity` - Discord API client
- `sqlx` - Async SQLite database
- `axum` - HTTP server for panel/API
- `tokio` - Async runtime

**Structure:**
```
rustmail/src/
├── main.rs              # Entry point
├── config.rs            # Configuration loading
├── api/                 # REST API
│   ├── handler/         # Request handlers
│   └── routes/          # Route definitions
├── commands/            # Discord commands
├── database/            # Database operations
├── handlers/            # Discord event handlers
├── i18n/                # Internationalization
├── modules/             # Background tasks
├── prelude/             # Common imports
└── utils/               # Utility functions
```

### rustmail_panel (Web UI)

Single-page application built with Yew framework, compiled to WebAssembly.

**Key dependencies:**
- `yew` - Rust/WASM framework
- `yew-router` - Client-side routing
- `wasm-bindgen` - JavaScript interop

**Structure:**
```
rustmail_panel/src/
├── main.rs              # App entry point
├── app.rs               # Root component
├── components/          # UI components
├── pages/               # Page components
├── i18n/                # Translations
└── utils/               # Client utilities
```

### rustmail_types (Shared Types)

Type definitions shared between crates.

```
rustmail_types/src/
├── lib.rs
├── api/                 # API types
│   └── panel_permissions.rs
└── config/              # Configuration types
    ├── bot.rs
    ├── commands.rs
    ├── error_handling.rs
    ├── languages.rs
    ├── logs.rs
    ├── notifications.rs
    ├── reminders.rs
    └── threads.rs
```

---

## Runtime Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Rustmail Process                        │
│                                                             │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │   Discord    │    │    Axum      │    │   SQLite     │  │
│  │   Gateway    │    │   Server     │    │   Database   │  │
│  │   (Serenity) │    │   (API)      │    │   (SQLx)     │  │
│  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘  │
│         │                   │                   │          │
│         └─────────┬─────────┴─────────┬─────────┘          │
│                   │                   │                    │
│            ┌──────┴───────┐    ┌──────┴───────┐           │
│            │   Shared     │    │  Background  │           │
│            │   State      │    │  Tasks       │           │
│            └──────────────┘    └──────────────┘           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
         │                          │
         ▼                          ▼
┌─────────────────┐        ┌─────────────────┐
│  Discord API    │        │  Web Browser    │
│  (Bot)          │        │  (Panel)        │
└─────────────────┘        └─────────────────┘
```

---

## Discord Integration

### Gateway Events

Rustmail reacts to various Discord gateway events. Here are the main ones:

| Event                | Handler                 | Purpose                   |
|----------------------|-------------------------|---------------------------|
| `ready`              | `ReadyHandler`          | Initialize bot state      |
| `message_create`     | `GuildMessagesHandler`  | Process DMs and commands  |
| `interaction_create` | `InteractionHandler`    | Handle slash commands     |
| `typing_start`       | `TypingProxyHandler`    | Forward typing indicators |

All event handlers are located in `rustmail/src/handlers/`.

### Commands System

Commands are defined in `rustmail/src/commands/`:

```
commands/
├── mod.rs               # CommandRegistry and register_all
├── command.rs           # Command trait
├── invocation.rs        # Invocation: who ran a command, where, and how to reply
├── slash_options.rs     # SlashOptions: owned slash options, subcommand path, context menu target
├── ticket.rs            # Shared helpers: the ticket of the invoking channel, its status
├── close/               # One folder per command: mod.rs (definition + run), args.rs (parsers + tests)
└── ...
```

A command implements the `Command` trait once for both surfaces (slash and text):

- `parse_text` turns what follows the command name into a typed `Args`, and `parse_slash` does the same from `SlashOptions`. Both are pure and unit-tested.
- `run(args, &Invocation, &Config)` holds the behaviour. It answers with `invocation.reply(...)` or `invocation.reply_translated(...)`, and reports problems by returning an error.
- `name`, `aliases`, `surfaces` (both, slash only, text only), `ephemeral`, `community`, `slash_definitions` and `community_definitions` describe it.
- Subcommands are read by `SlashOptions::subcommand()`, e.g. `["roles", "add"]` for `/category roles add`.

The dispatchers (`handlers/guild_interaction_handler.rs`, `handlers/guild_messages_handler.rs`) own everything around `run`: the staff check, maintenance mode, deferring slash commands (ephemerally outside the staff guild), and answering errors on either surface. Commands never defer.

Every command is registered in `register_all` (`commands/mod.rs`). What a command needs from the running bot (shutdown receiver, logs pagination store, maintenance flag) comes from `CommandServices` and is stored on the command struct. `CommandRegistry` panics at startup if two commands share a name or alias.

Errors returned by a command are shown as an error embed in the invoker's language; `{prefix}` in an error message is replaced by the configured command prefix.

---

## API Architecture

The HTTP server uses Axum with these route groups:

```
/api
├── /auth           # OAuth2 flow
│   ├── /login
│   ├── /callback
│   └── /logout
├── /bot            # Bot control
│   ├── /status
│   ├── /start
│   ├── /stop
│   ├── /restart
│   ├── /config
│   └── /tickets
├── /apikeys        # API key management
├── /admin          # Administration
├── /user           # User info
├── /panel          # Panel data
└── /externals      # External integrations
    └── /tickets/create
```

### Middleware

- Session authentication (cookie-based)
- API key authentication (header-based)
- Permission checking

---

## Database Layer

### Connection Pool

SQLx manages a connection pool to the SQLite database:

```rust
let pool = SqlitePool::connect("sqlite:db.sqlite").await?;
```

### Migrations

Schema changes use SQLx migrations:

```
migrations/
├── 20250815145017_create_tables.sql
├── 20250815161000_unique_open_and_metadata.sql
└── ...
```

Migrations run automatically at startup.

### Query Pattern

Database operations in `rustmail/src/database/`:

```rust
pub async fn get_thread_by_id(pool: &SqlitePool, id: &str) -> Result<Thread> {
    sqlx::query_as!(Thread, "SELECT * FROM threads WHERE id = ?", id)
        .fetch_one(pool)
        .await
}
```

---

## Internationalization

### Bot (rustmail)

Internal i18n system in `rustmail/src/i18n/`:

```rust
pub enum Language {
    English,
    French,
    Spanish,
    // ...
}

impl Language {
    pub fn get_message(&self, key: &str) -> &str {
        // Translation lookup
    }
}
```

### Panel (rustmail_panel)

JSON-based translations loaded at runtime:

```
rustmail_panel/src/i18n/
├── mod.rs
└── translations/
    ├── en.json
    └── fr.json
```

---

## Background Tasks

Long-running tasks managed by Tokio:

| Task               | Module                              | Purpose                  |
|--------------------|-------------------------------------|--------------------------|
| Reminders          | `modules/reminders.rs`              | Check and fire reminders |
| Scheduled closures | `modules/scheduled_closures.rs`     | Auto-close tickets       |
| Thread status      | `modules/threads_status_updates.rs` | Update thread metadata   |
| Features polling   | `modules/features_polling.rs`       | Track feature requests   |

---

## State Management

### Shared State

Global state shared across handlers:

```rust
pub struct Config {
    pub bot: BotConfig,
    pub thread: ThreadConfig,
    // ...
    pub db_pool: Option<SqlitePool>,
    pub thread_locks: Arc<Mutex<HashMap<u64, Arc<Mutex<()>>>>>,
}
```

### Thread Locking

Prevents race conditions on ticket operations:

```rust
let lock = config.thread_locks
    .lock()
    .entry(thread_id)
    .or_insert_with(|| Arc::new(Mutex::new(())))
    .clone();

let _guard = lock.lock().await;
// Thread-safe operations
```

---

## Build Pipeline

### Development

```bash
# Bot only
cargo build -p rustmail

# Panel (requires trunk)
cd rustmail_panel
trunk build

# All
cargo build --workspace
```

### Release

```bash
# Optimized build
cargo build --release -p rustmail

# Panel with optimization
trunk build --release
```

### CI/CD

GitHub Actions workflow:
1. Build and test all crates
2. Build panel WASM
3. Create release binaries
4. Build and push Docker image

---

## Extension Points

### Adding Commands

1. Create `rustmail/src/commands/<name>/` with `mod.rs` (a struct implementing `Command`) and `args.rs` (the `Args` type, `parse_text`/`parse_slash` and their tests). See `commands/close/`.
2. Register it with `registry.register(...)` in `register_all` (`commands/mod.rs`) and re-export it from `commands/mod.rs`.
3. Add the i18n keys to every file in `i18n/language/`, and document the command in `docs/guides/commands.md`.

### Adding API Endpoints

1. Create handler in `rustmail/src/api/handler/`
2. Add route in `rustmail/src/api/routes/`
3. Apply middleware as needed

### Adding Translations

1. Add language to `Language` enum
2. Implement translations
3. Add to `supported_languages` config
