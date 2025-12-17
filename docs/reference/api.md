# REST API Reference

Rustmail exposes a REST API on port 3002 for external integrations and the web panel.

---

## Base URL

```
http://localhost:3002/api
```

Or with a reverse proxy:

```
https://panel.example.com/api
```

---

## Authentication

### Session-Based (Panel)

The web panel and most API endpoints (`/api/bot/*`, `/api/admin/*`, `/api/user/*`, etc.) use Discord OAuth2 with session
cookies. These endpoints are designed for the panel interface, not external integrations.

### API Key (External Integrations)

API keys are used exclusively for the **External API** (`/api/externals/*` endpoints). They allow third-party
applications to interact with Rustmail without going through the panel.

**Use cases for API keys:**

- Create tickets from an external website or application
- Integrate Rustmail with other support systems
- Automate ticket creation from forms, bots, or scripts

**Important:** API keys only grant access to `/api/externals/*` endpoints. They cannot be used to access panel endpoints
like `/api/bot/status` or `/api/admin/*`.

Include the key in the `X-API-Key` header:

```
X-API-Key: rustmail_your_api_key_here
```

**Required headers:**

| Header         | Value              | Description                |
|----------------|--------------------|----------------------------|
| `X-API-Key`    | `rustmail_...`     | Your API key               |
| `Content-Type` | `application/json` | Required for POST requests |

**Example:**

```bash
curl --request POST \
  --url 'https://panel.example.com/api/externals/tickets/create' \
  --header 'Content-Type: application/json' \
  --header 'X-API-Key: rustmail_350e97ec369e3b8afe133d1154d6eb8f2e779bd9' \
  --data '{"discord_id": "123456789012345678"}'
```

---

## Endpoints

### Authentication

#### GET /api/auth/login

Initiates Discord OAuth2 flow. Redirects to Discord authorization.

**Response:** 302 Redirect to Discord

#### GET /api/auth/callback

OAuth2 callback handler. Discord redirects here after authorization.

**Query Parameters:**

- `code` - Authorization code from Discord
- `state` - Redirect URL after authentication

**Response:** 302 Redirect to panel home

#### GET /api/auth/logout

Ends the current session.

**Response:** 302 Redirect to panel home

---

### Bot Control

#### GET /api/bot/status

Get current bot status.

**Response:**

```json
{
  "status": "running",
  "presence": "online"
}
```

| Field      | Type   | Description                                     |
|------------|--------|-------------------------------------------------|
| `status`   | string | Bot state: `"running"` or `"stopped"`           |
| `presence` | string | Current presence: `online`, `idle`, `dnd`, etc. |

#### POST /api/bot/start

Start the bot (if stopped).

**Response (success):**

```json
"Bot is starting"
```

**Response (already running):**

```json
"Bot is already running"
```

#### POST /api/bot/stop

Stop the bot.

**Response (success):**

```json
"Bot stopped"
```

**Response (not running):**

```json
"Bot is not running"
```

#### POST /api/bot/restart

Restart the bot. Stops the bot and starts it again.

#### POST /api/bot/presence

Update bot presence status.

**Request Body:**

```json
{
  "status": "online"
}
```

| Value         | Description                                   |
|---------------|-----------------------------------------------|
| `online`      | Online status, shows configured activity      |
| `idle`        | Idle/Away status                              |
| `dnd`         | Do Not Disturb status                         |
| `invisible`   | Invisible/Offline status                      |
| `maintenance` | Maintenance mode (DND + maintenance activity) |

**Response:**

```json
{
  "status": "online"
}
```

---

### Configuration

#### GET /api/bot/config

Retrieve current configuration. Sensitive fields (token, client_secret) are partially masked.

**Response:**

```json
{
  "bot": {
    "token": "MTIz...4567",
    "status": "DM for support",
    "welcome_message": "...",
    "close_message": "...",
    "enable_panel": true,
    "client_id": 123456789012345678,
    "client_secret": "abc1...xyz9",
    "redirect_url": "https://panel.example.com/api/auth/callback",
    "timezone": "Europe/Paris"
  },
  "command": {
    ...
  },
  "thread": {
    ...
  },
  "language": {
    ...
  },
  "error_handling": {
    ...
  },
  "notifications": {
    ...
  },
  "reminders": {
    ...
  },
  "logs": {
    ...
  }
}
```

#### PUT /api/bot/config

Update configuration. Send the full configuration object. Masked fields (containing `...`) will preserve their original
values.

**Request Body:** Full `ConfigResponse` object

**Response:**

```json
{
  "success": true,
  "message": "Configuration saved successfully. Restart the bot to apply changes."
}
```

---

### Tickets

#### GET /api/bot/tickets

List tickets with pagination and filtering.

**Query Parameters:**

| Parameter     | Type   | Default    | Description                                        |
|---------------|--------|------------|----------------------------------------------------|
| `id`          | string | -          | Get a specific ticket by ID                        |
| `page`        | int    | 1          | Page number                                        |
| `page_size`   | int    | 50         | Items per page (max 200)                           |
| `status`      | int    | 0          | Filter: `0` = open, `1` = closed                   |
| `category_id` | string | -          | Filter by category ID                              |
| `sort_by`     | string | created_at | Sort field: `created_at`, `closed_at`, `user_name` |
| `sort_order`  | string | DESC       | Sort order: `asc` or `desc`                        |

**Response (list):**

```json
{
  "threads": [
    {
      "id": "abc123",
      "user_id": 123456789012345678,
      "user_name": "Username",
      "channel_id": "987654321098765432",
      "created_at": 1705312200,
      "new_message_number": 5,
      "status": 0,
      "user_left": false,
      "closed_at": null,
      "closed_by": null,
      "category_id": "111222333444555666",
      "category_name": "Support",
      "required_permissions": null,
      "messages": [
        ...
      ]
    }
  ],
  "total": 150,
  "page": 1,
  "page_size": 50,
  "total_pages": 3
}
```

**Response (single ticket with `?id=abc123`):**

```json
{
  "id": "abc123",
  "user_id": 123456789012345678,
  "user_name": "Username",
  "channel_id": "987654321098765432",
  "created_at": 1705312200,
  "new_message_number": 5,
  "status": 0,
  "user_left": false,
  "closed_at": null,
  "closed_by": null,
  "category_id": null,
  "category_name": null,
  "required_permissions": null,
  "messages": [
    {
      "id": 1,
      "thread_id": "abc123",
      "user_id": 123456789012345678,
      "user_name": "Username",
      "is_anonymous": false,
      "dm_message_id": "111222333",
      "inbox_message_id": "444555666",
      "message_number": 1,
      "created_at": "2024-01-15 10:30:00",
      "content": "Hello, I need help",
      "is_internal": false
    }
  ]
}
```

---

### External Ticket Creation

#### POST /api/externals/tickets/create

Create a ticket from an external source. Useful for integrating Rustmail with external support systems, websites, or
automation tools.

**Headers:**

```
Content-Type: application/json
X-API-Key: rustmail_your_api_key_here
```

**Request Body:**

```json
{
  "discord_id": "123456789012345678",
  "staff_discord_id": "987654321098765432"
}
```

| Field              | Type   | Required | Description                                   |
|--------------------|--------|----------|-----------------------------------------------|
| `discord_id`       | string | Yes      | Discord user ID to create a ticket for        |
| `staff_discord_id` | string | No       | Staff member to ping in the ticket (optional) |

**Full Example:**

```bash
curl --request POST \
  --url 'https://panel.example.com/api/externals/tickets/create' \
  --header 'Content-Type: application/json' \
  --header 'X-API-Key: rustmail_350e97ec369e3b8afe133d1154d6eb8f2e779bd9' \
  --data '{
    "discord_id": "689149284871962727",
    "staff_discord_id": "123456789012345678"
}'
```

**Response:**

```json
{
  "success": true,
  "channel_id": "987654321098765432",
  "user_id": "689149284871962727",
  "username": "Username",
  "message": "Ticket created successfully"
}
```

**Error Responses:**

| Status | Error                                       |
|--------|---------------------------------------------|
| 400    | Invalid Discord ID format                   |
| 403    | User is not a member of the community guild |
| 404    | Discord user not found                      |
| 409    | User already has an active ticket           |

---

### API Keys

#### GET /api/apikeys

List all API keys.

**Response:**

```json
[
  {
    "id": 1,
    "name": "Integration Key",
    "permissions": [
      "CreateTicket"
    ],
    "created_at": 1705312200,
    "expires_at": null,
    "last_used_at": 1705398600,
    "is_active": true,
    "key_preview": "a1b2c3d4e5f6..."
  }
]
```

#### POST /api/apikeys

Create a new API key.

**Request Body:**

```json
{
  "name": "My Integration",
  "permissions": [
    "CreateTicket"
  ],
  "expires_at": null
}
```

| Permission     | Description                |
|----------------|----------------------------|
| `CreateTicket` | Can create tickets via API |

**Response:**

```json
{
  "api_key": "rustmail_350e97ec369e3b8afe133d1154d6eb8f2e779bd9214a6800509d72c91a13f3e5",
  "id": 2,
  "name": "My Integration",
  "permissions": [
    "CreateTicket"
  ],
  "created_at": 1705312200,
  "expires_at": null
}
```

The `api_key` field is only returned once at creation. Store it securely as it cannot be retrieved later.

#### POST /api/apikeys/{id}/revoke

Revoke an API key (deactivates it but keeps the record).

**Response:** `204 No Content`

#### DELETE /api/apikeys/{id}

Permanently delete an API key.

**Response:** `204 No Content`

---

### Administration

#### GET /api/admin/members

List server members (for permission management).

**Response:**

```json
[
  {
    "user_id": "123456789012345678",
    "username": "StaffMember",
    "discriminator": "0",
    "avatar": "abc123def456",
    "roles": [
      "111222333444555666",
      "777888999000111222"
    ]
  }
]
```

#### GET /api/admin/roles

List server roles.

**Response:**

```json
[
  {
    "role_id": "123456789012345678",
    "name": "Moderator",
    "color": 3447003,
    "position": 5
  }
]
```

Roles are sorted by position (highest first).

#### GET /api/admin/permissions

List granted panel permissions.

**Response:**

```json
[
  {
    "id": 1,
    "subject_type": "User",
    "subject_id": "123456789012345678",
    "permission": "ViewPanel",
    "granted_by": "987654321098765432",
    "granted_at": 1705312200
  }
]
```

**Permission values:**

- `ViewPanel` - Can access the panel
- `ManageBot` - Can start/stop/restart the bot
- `ManageConfig` - Can edit configuration
- `ManageTickets` - Can manage tickets
- `ManageApiKeys` - Can create/revoke API keys
- `ManagePermissions` - Can grant/revoke permissions

**Subject types:**

- `User` - Permission granted to a specific user
- `Role` - Permission granted to all members with a role

#### POST /api/admin/permissions

Grant a permission.

**Request Body:**

```json
{
  "subject_type": "User",
  "subject_id": "123456789012345678",
  "permission": "ViewPanel"
}
```

**Response:**

```json
{
  "success": true
}
```

#### DELETE /api/admin/permissions/{id}

Revoke a permission.

**Response:**

```json
{
  "success": true
}
```

---

### User

#### GET /api/user/avatar

Get current user's avatar URL.

**Response:**

```json
{
  "avatar_url": "https://cdn.discordapp.com/avatars/123456789012345678/abc123.png"
}
```

#### GET /api/user/permissions

Get current user's panel permissions.

**Response:**

```json
[
  "ViewPanel",
  "ManageTickets"
]
```

Returns an array of permission strings. Super admins and server admins receive all permissions.

---

### Panel

#### GET /api/panel/check

Verify panel access. This endpoint requires authentication.

**Response:**

```json
{
  "authorized": true
}
```

---

## Error Responses

All endpoints return errors in a consistent format:

```json
{
  "error": "Error message"
}
```

### HTTP Status Codes

| Code | Meaning                                       |
|------|-----------------------------------------------|
| 200  | Success                                       |
| 204  | Success (no content)                          |
| 400  | Bad request (invalid parameters)              |
| 401  | Unauthorized (missing/invalid authentication) |
| 403  | Forbidden (insufficient permissions)          |
| 404  | Not found                                     |
| 409  | Conflict (e.g., ticket already exists)        |
| 500  | Internal server error                         |

---

## Rate Limiting

The API does not currently implement rate limiting. For high-volume integrations, implement client-side throttling to
avoid overwhelming the bot.

---

## Webhooks

Rustmail does not currently support outgoing webhooks. Use polling with the tickets endpoint for integration needs.
