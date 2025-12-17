# First Steps

This guide walks you through launching Rustmail and verifying your setup.

---

## Starting the Bot

### Direct Execution

From your installation directory:

```bash
./rustmail
```

On Windows:

```cmd
rustmail.exe
```

### Expected Output

On successful startup, you will see:

```
[INFO] Database connection pool established
[INFO] Database connected!
[INFO] Starting rustmail...
[INFO] listening on 0.0.0.0:3002
[INFO] Configuration successfully validated!!
[INFO] Mode: Mono server (ID: 711880297272311856)
[INFO] Rustmail is online !
[INFO] All pending reminders have been scheduled.
[INFO] Updated 0 ticket statuses
```

If there are configuration errors, the bot will display specific messages indicating what needs to be fixed.

---

## Verifying the Setup

### 1. Check Bot Status

In Discord, verify the bot appears online in your server's member list. Its status should display the text you
configured in `bot.status`.

### 2. Test Slash Commands

In any channel where the bot has access:

1. Type `/help` and press Enter
2. The bot should respond with a list of available commands

If slash commands don't appear:

- Wait a few minutes (Discord caches command registrations)
- Verify the bot has the `applications.commands` scope
- Check that the bot has permissions in the channel

### 3. Test Ticket Creation

1. Send a direct message to the bot
2. The bot should:
    - Reply with your configured `welcome_message`
    - Create a new channel in your inbox category
3. Staff can now respond using `/reply` or `!reply` in the ticket channel

### 4. Access the Panel (if enabled)

Open your browser and navigate to:

- Local: `http://localhost:3002`
- With reverse proxy: `https://your-panel-domain.com`

Click **Login** to authenticate with Discord.

---

## Common Startup Issues

### Configuration Parse Error

```
Failed to parse config.toml: ...
```

Check your `config.toml` for:

- Missing required fields
- Incorrect TOML syntax (missing quotes, brackets)
- Invalid color hex codes (should be 6 characters without `#`)

### Invalid Server ID

```
Serveur principal introuvable: ...
```

Verify:

- The guild IDs in your configuration are correct
- The bot has been invited to all configured servers
- You're using the server ID, not a channel or user ID

### Logs/Features Channel Required

```
'logs_channel_id' field is required if 'enable_logs' is true
```

Either:

- Set `enable_logs = false` to disable logging
- Or provide a valid `logs_channel_id`

The same applies to `enable_features` and `features_channel_id`.

### OAuth2 Errors

If the panel login fails:

- Verify `client_id` and `client_secret` match your Discord application
- Ensure `redirect_url` exactly matches what's configured in Discord Developer Portal
- Check that your application has the OAuth2 redirect URI added

---

## Running as a Service

For production, run Rustmail as a background service.

### Systemd (Linux)

Create `/etc/systemd/system/rustmail.service`:

```ini
[Unit]
Description=Rustmail Discord Bot
After=network.target

[Service]
Type=simple
User=rustmail
WorkingDirectory=/opt/rustmail
ExecStart=/opt/rustmail/rustmail
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable rustmail
sudo systemctl start rustmail
```

View logs:

```bash
sudo journalctl -u rustmail -f
```

### Docker

See [Docker Deployment](../deployment/docker.md) for containerized operation.

---

## Next Steps

Your bot is now running. Learn more about:

- [Commands](../guides/commands.md) - All available commands
- [Tickets](../guides/tickets.md) - Managing support tickets
- [Web Panel](../guides/panel.md) - Using the administration interface
