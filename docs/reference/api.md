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

The web panel and most API endpoints (`/api/bot/*`, `/api/admin/*`, `/api/user/*`, etc.) use Discord OAuth2 with session cookies. These endpoints are designed for the panel interface, not external integrations.

### API Key (External Integrations)

API keys are used exclusively for the **External API** (`/api/externals/*` endpoints). They allow third-party applications to interact with Rustmail without going through the panel.

**Use cases for API keys:**
- Create tickets from an external website or application
- Integrate Rustmail with other support systems
- Automate ticket creation from forms, bots, or scripts

**Important:** API keys only grant access to `/api/externals/*` endpoints. They cannot be used to access panel endpoints like `/api/bot/status` or `/api/admin/*`.

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
- `state` - CSRF protection token

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
  "online": true,
  "uptime": 3600,
  "presence": "DM for support",
  "guild_count": 2,
  "ticket_count": 15
}
```

#### POST /api/bot/start

Start the bot (if stopped).

**Response:**
```json
{
  "success": true,
  "message": "Bot started"
}
```

#### POST /api/bot/stop

Stop the bot.

**Response:**
```json
{
  "success": true,
  "message": "Bot stopped"
}
```

#### POST /api/bot/restart

Restart the bot.

**Response:**
```json
{
  "success": true,
  "message": "Bot restarted"
}
```

#### POST /api/bot/presence

Update bot presence/status.

**Request Body:**
```json
{
  "status": "New status message"
}
```

**Response:**
```json
{
  "success": true
}
```

---

### Configuration

#### GET /api/bot/config

Retrieve current configuration.

**Response:**
```json
{
  "bot": {
    "status": "DM for support",
    "welcome_message": "...",
    "close_message": "...",
    "enable_logs": true,
    "enable_features": false,
    "enable_panel": true
  },
  "thread": {
    "embedded_message": true,
    "user_message_color": "5865f2"
  }
}
```

Note: Sensitive fields (token, secrets) are not returned.

#### PUT /api/bot/config

Update configuration.

**Request Body:**
```json
{
  "bot": {
    "status": "Updated status"
  }
}
```

**Response:**
```json
{
  "success": true
}
```

---

### Tickets

#### GET /api/bot/tickets

List tickets.

**Query Parameters:**
- `status` - Filter by status (`open`, `closed`, `all`)
- `limit` - Maximum results (default: 50)
- `offset` - Pagination offset

**Response:**
```json
{
  "tickets": [
    {
      "id": "abc123",
      "user_id": "123456789012345678",
      "user_name": "Username",
      "channel_id": "987654321098765432",
      "created_at": "2024-01-15T10:30:00Z",
      "status": 1,
      "message_count": 12
    }
  ],
  "total": 150
}
```

---

### External Ticket Creation

#### POST /api/externals/tickets/create

Create a ticket from an external source. Useful for integrating Rustmail with external support systems, websites, or automation tools.

**Headers:**
```
Content-Type: application/json
X-API-Key: rustmail_your_api_key_here
```

**Request Body:**
```json
{
  "discord_id": "123456789012345678"
}
```

| Field        | Type   | Required | Description                            |
|--------------|--------|----------|----------------------------------------|
| `discord_id` | string | Yes      | Discord user ID to create a ticket for |

**Full Example:**
```bash
curl --request POST \
  --url 'https://panel.example.com/api/externals/tickets/create' \
  --header 'Content-Type: application/json' \
  --header 'X-API-Key: rustmail_350e97ec369e3b8afe133d1154d6eb8f2e779bd9' \
  --data '{
    "discord_id": "689149284871962727"
}'
```

**Response:**
```json
{
  "success": true,
  "ticket_id": "abc123",
  "channel_id": "987654321098765432"
}
```

**Error Response (user not found):**
```json
{
  "error": "User not found in community server"
}
```

---

### API Keys

#### GET /api/apikeys

List all API keys.

**Response:**
```json
{
  "keys": [
    {
      "id": 1,
      "name": "Integration Key",
      "permissions": ["read:tickets"],
      "created_at": 1705312200,
      "expires_at": null,
      "last_used_at": 1705398600,
      "is_active": true
    }
  ]
}
```

#### POST /api/apikeys

Create a new API key.

**Request Body:**
```json
{
  "name": "My Integration",
  "permissions": ["read:tickets", "write:tickets"],
  "expires_at": null
}
```

**Response:**
```json
{
  "id": 2,
  "key": "rustmail_350e97ec369e3b8afe133d1154d6eb8f2e779bd9214a6800509d72c91a13f3e5",
  "name": "My Integration"
}
```

The `key` field is only returned once at creation. Store it securely as it cannot be retrieved later.

#### POST /api/apikeys/{id}/revoke

Revoke an API key.

**Response:**
```json
{
  "success": true
}
```

#### DELETE /api/apikeys/{id}

Permanently delete an API key.

**Response:**
```json
{
  "success": true
}
```

---

### Administration

#### GET /api/admin/members

List server members (for permission management).

**Response:**
```json
{
  "members": [
    {
      "id": "123456789012345678",
      "username": "StaffMember",
      "avatar": "abc123"
    }
  ]
}
```

#### GET /api/admin/roles

List server roles.

**Response:**
```json
{
  "roles": [
    {
      "id": "123456789012345678",
      "name": "Moderator",
      "color": 3447003
    }
  ]
}
```

#### GET /api/admin/permissions

List granted panel permissions.

**Response:**
```json
{
  "permissions": [
    {
      "id": 1,
      "subject_type": "user",
      "subject_id": "123456789012345678",
      "permission": "view_tickets",
      "granted_by": "987654321098765432",
      "granted_at": 1705312200
    }
  ]
}
```

#### POST /api/admin/permissions

Grant a permission.

**Request Body:**
```json
{
  "subject_type": "user",
  "subject_id": "123456789012345678",
  "permission": "view_tickets"
}
```

**Response:**
```json
{
  "success": true,
  "id": 2
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
  "avatar_url": "https://cdn.discordapp.com/avatars/..."
}
```

#### GET /api/user/permissions

Get current user's permissions.

**Response:**
```json
{
  "is_super_admin": false,
  "permissions": ["view_tickets", "manage_tickets"]
}
```

---

### Panel

#### GET /api/panel/check

Verify panel access.

**Response:**
```json
{
  "authenticated": true,
  "user_id": "123456789012345678"
}
```

---

## Error Responses

All endpoints return errors in a consistent format:

```json
{
  "error": "Error message",
  "code": "ERROR_CODE"
}
```

### HTTP Status Codes

| Code | Meaning                                       |
|------|-----------------------------------------------|
| 200  | Success                                       |
| 400  | Bad request (invalid parameters)              |
| 401  | Unauthorized (missing/invalid authentication) |
| 403  | Forbidden (insufficient permissions)          |
| 404  | Not found                                     |
| 500  | Internal server error                         |

---

## Rate Limiting

The API does not currently implement rate limiting. For high-volume integrations, implement client-side throttling to avoid overwhelming the bot.

---

## Webhooks

Rustmail does not currently support outgoing webhooks. Use polling with the tickets endpoint for integration needs.
