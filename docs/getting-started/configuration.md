# Configuration

This guide explains how to configure Rustmail using the `config.toml` file.

---

## Using the Configuration Generator

The easiest way to create your configuration is the online generator:

**[config.rustmail.rs](https://config.rustmail.rs)**

The generator walks you through each setting and produces a valid `config.toml` file. You can also build the
configurator locally from the [rustmail_configurator](https://github.com/Rustmail/rustmail_configurator) repository.

---

## Manual Configuration

If you prefer to create the configuration manually, copy `config.example.toml` and edit it:

```bash
cp config.example.toml config.toml
```

Below is an overview of each configuration section. For a complete reference of all options,
see [Configuration Reference](../reference/configuration.md).

---

## Essential Settings

### Bot Section

```toml
[bot]
token = "YOUR_BOT_TOKEN"
status = "DM for support"
welcome_message = "Your message has been received. Staff will respond shortly."
close_message = "This ticket has been closed. Thank you for contacting us."
```

| Field             | Description                                      |
|-------------------|--------------------------------------------------|
| `token`           | Your Discord bot token from the Developer Portal |
| `status`          | Text displayed as the bot's activity status      |
| `welcome_message` | Sent to users when they open a new ticket        |
| `close_message`   | Sent to users when their ticket is closed        |

### Server Mode

Rustmail supports two operating modes. Choose based on your server structure.

**Single-server mode** - Everything on one Discord server:

```toml
[bot.mode]
type = "single"
guild_id = 123456789012345678
```

**Dual-server mode** - Separate community and staff servers:

```toml
[bot.mode]
type = "dual"
community_guild_id = 123456789012345678
staff_guild_id = 987654321098765432
```

In dual-server mode:

- `community_guild_id` is where your users are
- `staff_guild_id` is where ticket channels are created

See [Server Modes](../guides/server-modes.md) for detailed guidance on choosing and configuring modes.

### Thread Settings

```toml
[thread]
inbox_category_id = 123456789012345678
embedded_message = true
user_message_color = "3d54ff"
staff_message_color = "ff3126"
```

The `inbox_category_id` is required. Create a category in your staff server (or your single server) and copy its ID. All
ticket channels will be created under this category.

---

## Web Panel Configuration

The web panel provides browser-based administration. Enabling it requires OAuth2 setup.

### OAuth2 Setup

1. In the [Discord Developer Portal](https://discord.com/developers/applications), select your application
2. Go to **OAuth2 > General**
3. Copy the **Client ID** and **Client Secret**
4. Add a redirect URL (see below)

```toml
[bot]
enable_panel = true
client_id = 123456789012345678
client_secret = "your_oauth2_client_secret"
redirect_url = "http://localhost:3002/api/auth/callback"
```

### Understanding redirect_url vs ip

These two fields serve different purposes and are often confused:

| Field          | Purpose                                            | Required                     |
|----------------|----------------------------------------------------|------------------------------|
| `redirect_url` | Public URL for OAuth2 authentication and log links | **Yes** (if panel enabled)   |
| `ip`           | Network interface binding address                  | No (defaults to auto-detect) |

### The redirect_url Field (Important)

The `redirect_url` is **your panel's public URL**. It is used for:

1. **OAuth2 authentication** - Discord redirects users here after login
2. **Log links** - Links to ticket logs sent in your logs channel

It must:

- Match **exactly** what you configured in the Discord Developer Portal
- End with `/api/auth/callback`
- Be accessible from the internet (or your network for local use)

**Local development:**

```toml
redirect_url = "http://localhost:3002/api/auth/callback"
```

**Production with domain (behind reverse proxy):**

```toml
redirect_url = "https://panel.example.com/api/auth/callback"
```

**LAN access (no domain):**

```toml
redirect_url = "http://192.168.1.100:3002/api/auth/callback"
```

### The ip Field (Optional)

```toml
[bot]
ip = "192.168.1.100"  # Optional
```

The `ip` field controls which **network interface** the panel server binds to. This is a technical setting for advanced
network configurations.

- If omitted, Rustmail auto-detects your local IP
- If the IP is invalid or unavailable, it falls back to `0.0.0.0` (all interfaces)

**When to set it manually:**

- Running in Docker with host networking
- When auto-detection returns an incorrect interface
- When you need to bind to a specific network interface

**For most users:** Leave `ip` unset and focus on configuring `redirect_url` correctly.

---

## Reverse Proxy Setup

For production deployments, you typically run Rustmail behind a reverse proxy (Nginx, Caddy, Traefik, NPM, etc.) with a
custom domain.

### Architecture

```
Internet → Reverse Proxy (443) → Rustmail (3002)
              ↓
         SSL/TLS termination
         Domain: panel.example.com
```

### Nginx Proxy Manager (NPM)

1. **Add Proxy Host:**
    - Domain: `panel.example.com`
    - Scheme: `http`
    - Forward Hostname/IP: Your server's internal IP or `localhost`
    - Forward Port: `3002`
    - Enable SSL with Let's Encrypt

2. **Configure Rustmail:**
   ```toml
   [bot]
   enable_panel = true
   redirect_url = "https://panel.example.com/api/auth/callback"
   ```

3. **Update Discord OAuth2:**
    - Add `https://panel.example.com/api/auth/callback` to your redirect URIs

### Nginx Configuration

```nginx
server {
    listen 443 ssl http2;
    server_name panel.example.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://127.0.0.1:3002;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Caddy Configuration

```caddyfile
panel.example.com {
    reverse_proxy localhost:3002
}
```

### Traefik Labels (Docker)

```yaml
labels:
  - "traefik.enable=true"
  - "traefik.http.routers.rustmail.rule=Host(`panel.example.com`)"
  - "traefik.http.routers.rustmail.tls.certresolver=letsencrypt"
  - "traefik.http.services.rustmail.loadbalancer.server.port=3002"
```

### Common Issues

**OAuth2 redirect mismatch:**
The redirect URL in `config.toml` must exactly match one of the URLs configured in Discord Developer Portal. Check for:

- Protocol mismatch (`http` vs `https`)
- Trailing slashes
- Port numbers

**Panel not accessible:**

- Verify the reverse proxy can reach port 3002
- Check firewall rules
- Ensure Rustmail is running and panel is enabled

---

## Panel Administrators

Define super administrators who have full panel access:

```toml
[bot]
panel_super_admin_users = [123456789012345678]
panel_super_admin_roles = [987654321098765432]
```

- `panel_super_admin_users` - List of Discord user IDs
- `panel_super_admin_roles` - List of Discord role IDs

Users matching either list have unrestricted panel access. Additional permissions can be granted through the panel
itself.

---

## Language Settings

```toml
[language]
default_language = "en"
fallback_language = "en"
supported_languages = ["en", "fr", "es", "de"]
```

Available language codes: `en`, `fr`, `es`, `de`, `it`, `pt`, `ru`, `zh`, `ja`, `ko`

---

## Complete Example

```toml
[bot]
token = "YOUR_BOT_TOKEN"
status = "DM for support"
welcome_message = "Your message has been received. Staff will respond shortly."
close_message = "This ticket has been closed. Thank you."
typing_proxy_from_user = true
typing_proxy_from_staff = true
enable_logs = true
enable_features = false
enable_panel = true
client_id = 123456789012345678
client_secret = "your_client_secret"
redirect_url = "https://panel.example.com/api/auth/callback"
timezone = "Europe/Paris"
logs_channel_id = 123456789012345678
panel_super_admin_users = [123456789012345678]
panel_super_admin_roles = []

[bot.mode]
type = "dual"
community_guild_id = 123456789012345678
staff_guild_id = 987654321098765432

[command]
prefix = "!"

[thread]
inbox_category_id = 123456789012345678
embedded_message = true
user_message_color = "5865f2"
staff_message_color = "57f287"
system_message_color = "faa81a"
block_quote = true
time_to_close_thread = 0
create_ticket_by_create_channel = false

[language]
default_language = "en"
fallback_language = "en"
supported_languages = ["en", "fr"]

[notifications]
show_success_on_edit = false
show_partial_success_on_edit = true
show_failure_on_edit = true
show_success_on_reply = false
show_success_on_delete = false

[logs]
show_log_on_edit = true
show_log_on_delete = true

[reminders]
embed_color = "ffb800"

[error_handling]
show_detailed_errors = false
log_errors = true
send_error_embeds = true
auto_delete_error_messages = true
error_message_ttl = 30
```

---

## Next Steps

After creating your configuration:

1. Verify the file is named `config.toml` and is in the same directory as the executable
2. Proceed to [First Steps](first-steps.md) to launch the bot
