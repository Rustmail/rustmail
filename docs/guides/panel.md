# Web Panel

The Rustmail web panel provides browser-based administration for managing tickets, configuration, and permissions.

---

## Enabling the Panel

The panel is optional and requires OAuth2 configuration. See [Configuration](../getting-started/configuration.md#web-panel-configuration) for setup instructions.

```toml
[bot]
enable_panel = true
client_id = 123456789012345678
client_secret = "your_oauth2_client_secret"
redirect_url = "https://panel.example.com/api/auth/callback"
```

---

## Accessing the Panel

### Default URL

The panel runs on port 3002:

- Local: `http://localhost:3002`
- Network: `http://<server-ip>:3002`

### With Reverse Proxy

When using a reverse proxy with a custom domain:

`https://panel.example.com`

See [Configuration](../getting-started/configuration.md#reverse-proxy-setup) for proxy setup.

---

## Authentication

### Login Process

1. Click **Login** on the panel homepage
2. You are redirected to Discord's OAuth2 authorization
3. Authorize the application
4. Discord redirects back to the panel
5. A session is created

Sessions persist across browser restarts. Click **Logout** to end your session.

### Access Requirements

Panel access requires one of:

- Being listed in `panel_super_admin_users`
- Having a role listed in `panel_super_admin_roles`
- Having been granted permissions through the panel

---

## Panel Sections

### Dashboard

The home view displays:

- Bot status (online/offline)
- Active ticket count
- Quick statistics

### Tickets

View and manage active tickets:

- List of all open tickets
- User information
- Ticket creation time
- Quick actions

### Configuration

Modify bot settings without editing `config.toml`:

- Change bot status/presence
- Update welcome and close messages
- Toggle features

Changes take effect immediately without restart.

### API Keys

Manage API keys for external integrations:

- Create new API keys
- Set permissions per key
- Revoke or delete keys
- View last usage time

### Administration

For super administrators:

- Manage panel permissions
- Grant access to users and roles
- View audit information

---

## Permission System

### Super Administrators

Defined in `config.toml`:

```toml
[bot]
panel_super_admin_users = [123456789012345678]
panel_super_admin_roles = [987654321098765432]
```

Super administrators have:
- Full panel access
- Ability to grant permissions to others
- Access to all tickets and settings

### Granted Permissions

Super administrators can grant specific permissions to users or roles through the Administration section.

Available permissions:
- View tickets
- Manage tickets
- View configuration
- Edit configuration
- Manage API keys

---

## API Keys

API keys allow external applications to create tickets in Rustmail without going through the panel or Discord.

### What Are API Keys For?

API keys provide access to the **External API** (`/api/externals/*` endpoints). Common use cases:

- **Website integration** - Let users create support tickets from your website
- **Cross-platform support** - Connect Rustmail to other support tools
- **Automation** - Create tickets from scripts, forms, or other bots

API keys do not grant access to panel features (bot control, configuration, etc.). Those require logging in through the panel.

### Creating a Key

1. Navigate to **API Keys** in the panel
2. Click **Create New Key**
3. Enter a descriptive name (e.g., "Website Contact Form")
4. Optionally set an expiration date
5. Copy the generated key immediately (it won't be shown again)

### Using API Keys

Include the key in the `X-API-Key` header when calling external endpoints:

```
X-API-Key: rustmail_your_api_key_here
```

**Example: Create a ticket from an external source**
```bash
curl --request POST \
  --url 'https://panel.example.com/api/externals/tickets/create' \
  --header 'Content-Type: application/json' \
  --header 'X-API-Key: rustmail_350e97ec369e3b8afe133d1154d6eb8f...' \
  --data '{"discord_id": "123456789012345678"}'
```

See [API Reference](../reference/api.md) for all available external endpoints.

### Revoking Keys

- **Revoke**: Immediately invalidates the key but keeps it in records for audit purposes
- **Delete**: Permanently removes the key from the system

Revoke keys when they may have been compromised. Delete keys that are no longer needed.

---

## Security Considerations

### Session Security

- Sessions are stored server-side
- Session tokens are cryptographically random
- Sessions expire after the configured duration

### Network Security

For production deployments:

1. **Use HTTPS** - Run behind a reverse proxy with TLS
2. **Restrict access** - Use firewall rules to limit who can reach the panel
3. **Strong secrets** - Use a secure OAuth2 client secret

### Access Control

- Regularly audit panel permissions
- Remove access for departed staff members
- Use role-based permissions when possible

---

## Troubleshooting

### Cannot Login

**OAuth2 redirect mismatch:**
- Verify `redirect_url` exactly matches Discord Developer Portal
- Check for protocol (`http` vs `https`) and trailing slash differences

**Client ID/Secret incorrect:**
- Regenerate the client secret in Discord Developer Portal
- Update `config.toml` and restart

### Session Expires Immediately

- Check system clock synchronization
- Verify the database is writable
- Check for cookie blocking in browser

### Panel Not Loading

- Ensure `enable_panel = true`
- Check that port 3002 is not blocked
- Verify the bot process is running
- Check reverse proxy configuration if applicable
