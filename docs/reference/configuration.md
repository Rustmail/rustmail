# Configuration Reference

Complete reference for all `config.toml` options.

---

## Bot Section

```toml
[bot]
```

### Core Settings

| Option            | Type   | Required | Default | Description                                 |
|-------------------|--------|----------|---------|---------------------------------------------|
| `token`           | string | Yes      | -       | Discord bot token from Developer Portal     |
| `status`          | string | Yes      | -       | Bot's activity status message               |
| `welcome_message` | string | Yes      | -       | Message sent to users when opening a ticket |
| `close_message`   | string | Yes      | -       | Message sent to users when ticket is closed |

### Typing Indicators

| Option                    | Type | Required | Default | Description                                     |
|---------------------------|------|----------|---------|-------------------------------------------------|
| `typing_proxy_from_user`  | bool | Yes      | -       | Show typing indicator in ticket when user types |
| `typing_proxy_from_staff` | bool | Yes      | -       | Show typing indicator in DM when staff types    |

### Feature Toggles

| Option            | Type | Required | Default | Description                     |
|-------------------|------|----------|---------|---------------------------------|
| `enable_logs`     | bool | Yes      | -       | Enable logging to a channel     |
| `enable_features` | bool | Yes      | -       | Enable feature request tracking |
| `enable_panel`    | bool | Yes      | -       | Enable web administration panel |

### Channel Configuration

| Option                | Type | Required    | Default | Description                                                        |
|-----------------------|------|-------------|---------|--------------------------------------------------------------------|
| `logs_channel_id`     | u64  | Conditional | -       | Channel for bot logs. Required if `enable_logs = true`             |
| `features_channel_id` | u64  | Conditional | -       | Channel for feature requests. Required if `enable_features = true` |

### OAuth2 (Panel)

Required when `enable_panel = true`.

| Option          | Type   | Required    | Default | Description                                     |
|-----------------|--------|-------------|---------|-------------------------------------------------|
| `client_id`     | u64    | Conditional | -       | Discord application client ID                   |
| `client_secret` | string | Conditional | -       | Discord application client secret               |
| `redirect_url`  | string | Conditional | -       | OAuth2 callback URL (must match Discord config) |

### Network

| Option | Type   | Required | Default       | Description                      |
|--------|--------|----------|---------------|----------------------------------|
| `ip`   | string | No       | Auto-detected | IP address for panel URL display |

**Note:** The `ip` field only affects what address is displayed for panel access. The server always binds to `0.0.0.0:3002`. Set this manually when:
- Auto-detection returns wrong interface
- Running in Docker with custom networking
- You want to show a specific LAN address

### Panel Administrators

| Option                    | Type  | Required | Default | Description                     |
|---------------------------|-------|----------|---------|---------------------------------|
| `panel_super_admin_users` | [u64] | No       | `[]`    | User IDs with full panel access |
| `panel_super_admin_roles` | [u64] | No       | `[]`    | Role IDs with full panel access |

### Timezone

| Option     | Type   | Required | Default | Description                           |
|------------|--------|----------|---------|---------------------------------------|
| `timezone` | string | No       | `UTC`   | Timezone for timestamps (IANA format) |

**Examples:** `Europe/Paris`, `America/New_York`, `Asia/Tokyo`

---

## Server Mode Section

```toml
[bot.mode]
```

### Single Server Mode

```toml
[bot.mode]
type = "single"
guild_id = 123456789012345678
```

| Option     | Type   | Required | Description            |
|------------|--------|----------|------------------------|
| `type`     | string | Yes      | Must be `"single"`     |
| `guild_id` | u64    | Yes      | Your Discord server ID |

### Dual Server Mode

```toml
[bot.mode]
type = "dual"
community_guild_id = 123456789012345678
staff_guild_id = 987654321098765432
```

| Option               | Type   | Required | Description                      |
|----------------------|--------|----------|----------------------------------|
| `type`               | string | Yes      | Must be `"dual"`                 |
| `community_guild_id` | u64    | Yes      | Server where users are           |
| `staff_guild_id`     | u64    | Yes      | Server where tickets are created |

---

## Command Section

```toml
[command]
```

| Option   | Type   | Required | Default | Description              |
|----------|--------|----------|---------|--------------------------|
| `prefix` | string | Yes      | `"!"`   | Prefix for text commands |

---

## Thread Section

```toml
[thread]
```

### Required Settings

| Option              | Type | Required | Default | Description                                |
|---------------------|------|----------|---------|--------------------------------------------|
| `inbox_category_id` | u64  | Yes      | -       | Category where ticket channels are created |

### Message Display

| Option                 | Type   | Required | Default    | Description                               |
|------------------------|--------|----------|------------|-------------------------------------------|
| `embedded_message`     | bool   | Yes      | -          | Display messages as Discord embeds        |
| `user_message_color`   | string | Yes      | `"5865f2"` | Hex color for user messages (without #)   |
| `staff_message_color`  | string | Yes      | `"57f287"` | Hex color for staff messages (without #)  |
| `system_message_color` | string | Yes      | `"faa81a"` | Hex color for system messages (without #) |
| `block_quote`          | bool   | Yes      | -          | Use block quotes for message content      |

### Ticket Behavior

| Option                            | Type | Required | Default | Description                                     |
|-----------------------------------|------|----------|---------|-------------------------------------------------|
| `time_to_close_thread`            | u64  | Yes      | `0`     | Default minutes until auto-close (0 = disabled) |
| `create_ticket_by_create_channel` | bool | Yes      | -       | Allow ticket creation by making a channel       |
| `close_on_leave`                  | bool | No       | `false` | Auto-close when user leaves server              |
| `auto_archive_duration`           | u16  | No       | `10080` | Thread auto-archive time in minutes             |

---

## Language Section

```toml
[language]
```

| Option                | Type     | Required | Default        | Description                       |
|-----------------------|----------|----------|----------------|-----------------------------------|
| `default_language`    | string   | Yes      | `"en"`         | Default language code             |
| `fallback_language`   | string   | Yes      | `"en"`         | Fallback when translation missing |
| `supported_languages` | [string] | Yes      | `["en", "fr"]` | Available languages               |

### Available Language Codes

| Code | Language   |
|------|------------|
| `en` | English    |
| `fr` | French     |
| `es` | Spanish    |
| `de` | German     |
| `it` | Italian    |
| `pt` | Portuguese |
| `ru` | Russian    |
| `zh` | Chinese    |
| `ja` | Japanese   |
| `ko` | Korean     |

---

## Notifications Section

```toml
[notifications]
```

Control feedback messages shown to staff.

| Option                         | Type | Required | Default | Description                    |
|--------------------------------|------|----------|---------|--------------------------------|
| `show_success_on_edit`         | bool | Yes      | `true`  | Confirm successful edits       |
| `show_partial_success_on_edit` | bool | Yes      | `true`  | Notify on partial edit success |
| `show_failure_on_edit`         | bool | Yes      | `true`  | Notify on edit failure         |
| `show_success_on_reply`        | bool | Yes      | `true`  | Confirm sent replies           |
| `show_success_on_delete`       | bool | Yes      | `true`  | Confirm deletions              |
| `show_success`                 | bool | No       | `true`  | General success notifications  |
| `show_error`                   | bool | No       | `true`  | General error notifications    |

---

## Logs Section

```toml
[logs]
```

Control what actions are logged.

| Option               | Type | Required | Default | Description           |
|----------------------|------|----------|---------|-----------------------|
| `show_log_on_edit`   | bool | Yes      | `true`  | Log message edits     |
| `show_log_on_delete` | bool | Yes      | `true`  | Log message deletions |

---

## Reminders Section

```toml
[reminders]
```

| Option        | Type   | Required | Default    | Description                               |
|---------------|--------|----------|------------|-------------------------------------------|
| `embed_color` | string | Yes      | `"ffcc00"` | Hex color for reminder embeds (without #) |

---

## Error Handling Section

```toml
[error_handling]
```

| Option                       | Type | Required | Default | Description                      |
|------------------------------|------|----------|---------|----------------------------------|
| `show_detailed_errors`       | bool | Yes      | `true`  | Show technical details in errors |
| `log_errors`                 | bool | Yes      | `true`  | Log errors to console            |
| `send_error_embeds`          | bool | Yes      | `true`  | Send errors as embeds            |
| `auto_delete_error_messages` | bool | Yes      | `false` | Auto-delete error messages       |
| `error_message_ttl`          | u64  | No       | -       | Seconds before error deletion    |
| `display_errors`             | bool | No       | `true`  | Show errors to users             |

---

## Complete Example

```toml
[bot]
token = "YOUR_BOT_TOKEN"
status = "DM for support"
welcome_message = "Your message has been received. Staff will respond shortly."
close_message = "This ticket has been closed. Thank you for contacting us."
typing_proxy_from_user = true
typing_proxy_from_staff = true
enable_logs = true
enable_features = false
enable_panel = true
client_id = 123456789012345678
client_secret = "your_client_secret_here"
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
close_on_leave = false
auto_archive_duration = 10080

[language]
default_language = "en"
fallback_language = "en"
supported_languages = ["en", "fr", "es", "de"]

[notifications]
show_success_on_edit = false
show_partial_success_on_edit = true
show_failure_on_edit = true
show_success_on_reply = false
show_success_on_delete = false
show_success = true
show_error = true

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
display_errors = true
```

---

## Environment Variables

Rustmail does not currently support environment variable substitution in `config.toml`. For sensitive values in containerized environments, consider mounting the config file as a secret.
