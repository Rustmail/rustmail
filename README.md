<p align="center">
  <img src="https://github.com/Rustmail/rustmail/blob/main/docs/static/logo.svg?raw=true" width="200" alt="Rustmail">
</p>

<h1 align="center">Rustmail</h1>

<p align="center">
  A modern Discord modmail bot written in Rust
</p>

<p align="center">
  <a href="https://discord.gg/9Bzma6SwtW"><img src="https://img.shields.io/discord/1407300551686885418?color=5865F2&logo=discord&logoColor=white&label=Discord" alt="Discord"></a>
  <a href="https://github.com/Rustmail/rustmail/releases"><img src="https://img.shields.io/github/v/release/Rustmail/rustmail?label=Release" alt="Release"></a>
  <a href="https://github.com/Rustmail/rustmail/blob/main/LICENSE"><img src="https://img.shields.io/github/license/Rustmail/rustmail?label=License" alt="License"></a>
</p>

---

## Overview

Rustmail is a Discord modmail bot that enables staff teams to manage support tickets through private channels. Users
send direct messages to the bot, which creates dedicated channels for staff to respond and track conversations.

### Key Features

- **Dual-server or single-server mode** - Separate community and staff servers, or run everything on one server
- **Web administration panel** - Manage tickets, configuration, and permissions through a browser
- **Message editing and deletion** - Full control over ticket messages with change tracking
- **Scheduled closures and reminders** - Automate ticket management workflows
- **Multi-language support** - 10 languages available (EN, FR, ES, DE, IT, PT, RU, ZH, JA, KO)
- **REST API** - Integrate with external tools and automation

---

## Quick Start

### 1. Generate your configuration

Use the online configurator to create your `config.toml` file:

**[config.rustmail.rs](https://config.rustmail.rs)**

The configurator guides you through all required settings and generates a ready-to-use configuration file.

### 2. Download and run

Download the latest release for your platform from [Releases](https://github.com/Rustmail/rustmail/releases), then:

```bash
# Place config.toml in the same directory as the executable
./rustmail
```

The bot creates its SQLite database automatically on first run.

---

## Documentation

| Section                                                | Description                     |
|--------------------------------------------------------|---------------------------------|
| [Installation](docs/getting-started/installation.md)   | Download and setup instructions |
| [Configuration](docs/getting-started/configuration.md) | Detailed configuration guide    |
| [Commands](docs/guides/commands.md)                    | Complete command reference      |
| [Server Modes](docs/guides/server-modes.md)            | Single vs dual-server setup     |
| [Web Panel](docs/guides/panel.md)                      | Administration panel guide      |
| [API Reference](docs/reference/api.md)                 | REST API documentation          |
| [Docker Deployment](docs/deployment/docker.md)         | Container deployment            |
| [Architecture](docs/development/architecture.md)       | Technical overview              |

Full documentation is available in the [docs](docs/) directory or on the [website](https://docs.rustmail.rs).

---

## Support

For help and discussions, join the [Discord server](https://discord.gg/9Bzma6SwtW).

---

## License

This project is licensed under the [AGPLv3 License](LICENSE).

The `rustmail_panel` i18n module includes code derived from [i18n-rs](https://github.com/opensass/i18n-rs), licensed
under MIT.
