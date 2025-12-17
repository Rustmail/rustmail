# Installation

This guide covers downloading and setting up Rustmail on your system.

---

## Prerequisites

### Discord Bot Application

Before installing Rustmail, create a Discord application:

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications)
2. Click **New Application** and give it a name
3. Navigate to **Bot** in the sidebar
4. Click **Add Bot**
5. Under **Privileged Gateway Intents**, enable:
    - **Presence Intent**
    - **Server Members Intent**
    - **Message Content Intent**
6. Copy the bot token (you will need it for configuration)

### Bot Invitation

Invite the bot to your server(s) with the required permissions:

1. Go to **OAuth2 > URL Generator**
2. Select scopes: `bot`, `applications.commands`
3. Select permissions:
    - Manage Channels
    - Read Messages/View Channels
    - Send Messages
    - Manage Messages
    - Embed Links
    - Attach Files
    - Read Message History
    - Add Reactions
    - Use Slash Commands
4. Copy the generated URL and open it in your browser
5. Select your server and authorize

For dual-server mode, invite the bot to both servers.

---

## Download

### Pre-built Binaries

Download the latest release from [GitHub Releases](https://github.com/Rustmail/rustmail/releases).

Available platforms:

- Linux (x86_64)
- Windows (x86_64)
- macOS (x86_64, ARM64)

Extract the archive to your desired installation directory.

### Docker

Pull the official image:

```bash
docker pull ghcr.io/rustmail/rustmail:latest
```

See [Docker Deployment](../deployment/docker.md) for complete container setup.

### Build from Source

See [Building](../development/building.md) for compilation instructions.

---

## Directory Structure

After extraction, your installation directory should contain:

```
rustmail/
├── rustmail          # Main executable (rustmail.exe on Windows)
└── config.toml       # Configuration file (create this)
```

On first run, the bot creates:

```
rustmail/
├── rustmail
├── config.toml
└── db
    └── db.sqlite # SQLite database (auto-created)
```

---

## Next Steps

Proceed to [Configuration](configuration.md) to set up your `config.toml` file.
