# Server Modes

Rustmail supports two operating modes to fit different community structures.

---

## Overview

| Mode   | Servers | Use Case                         |
|--------|---------|----------------------------------|
| Single | 1       | Small communities, simple setup  |
| Dual   | 2       | Large communities, staff privacy |

---

## Single-Server Mode

Everything operates within one Discord server.

### Configuration

```toml
[bot.mode]
type = "single"
guild_id = 123456789012345678
```

### Structure

```
Your Server
├── #general (community channels)
├── #announcements
├── Tickets (category)
│   ├── #ticket-user1
│   └── #ticket-user2
└── #staff-chat
```

### Advantages

- Simple setup
- No need for multiple server invitations
- Users can see ticket activity (if desired)

### Considerations

- Staff channels visible in same server
- Ticket category permissions must be carefully managed
- Less separation between community and support operations

### Permission Setup

1. Create a category for tickets (e.g., "Tickets")
2. Set category permissions:
   - `@everyone`: Deny View Channel
   - Staff roles: Allow View Channel
3. Use this category ID as `inbox_category_id`

---

## Dual-Server Mode

Separates your community server from your staff operations server.

### Configuration

```toml
[bot.mode]
type = "dual"
community_guild_id = 123456789012345678
staff_guild_id = 987654321098765432
```

### Structure

**Community Server:**
```
Community Server
├── #general
├── #announcements
└── #support-info
```

**Staff Server:**
```
Staff Server
├── Tickets (category)
│   ├── #ticket-user1
│   └── #ticket-user2
├── #staff-discussion
└── #logs
```

### Advantages

- Complete separation of community and staff spaces
- Staff discussions remain private
- Cleaner community server
- Better for larger operations with many staff members

### Considerations

- Requires managing two servers
- Bot must be invited to both servers
- Staff need access to the staff server

### Setup Steps

1. Create or designate your staff server
2. Invite the bot to both servers
3. In the staff server, create a category for tickets
4. Configure:
   ```toml
   [bot.mode]
   type = "dual"
   community_guild_id = <your_community_server_id>
   staff_guild_id = <your_staff_server_id>

   [thread]
   inbox_category_id = <category_id_in_staff_server>
   ```

---

## Choosing a Mode

### Choose Single-Server if:

- Your community is small (under 1,000 members)
- You have a small staff team
- You want simple setup and management
- Staff visibility in the community server is acceptable

### Choose Dual-Server if:

- Your community is large
- You have a dedicated staff team
- You want complete separation of concerns
- Staff privacy is important
- You handle sensitive support topics

---

## Migration Between Modes

### Single to Dual

1. Create a new staff server
2. Invite the bot to the staff server
3. Create a tickets category in the staff server
4. Update `config.toml`:
   ```toml
   [bot.mode]
   type = "dual"
   community_guild_id = <existing_server_id>
   staff_guild_id = <new_staff_server_id>

   [thread]
   inbox_category_id = <new_category_id>
   ```
5. Restart the bot

Existing tickets in the old location will remain but become inactive. New tickets will be created in the staff server.

### Dual to Single

1. Update `config.toml`:
   ```toml
   [bot.mode]
   type = "single"
   guild_id = <your_server_id>

   [thread]
   inbox_category_id = <category_id_in_that_server>
   ```
2. Restart the bot

---

## Getting Server and Category IDs

1. Enable Developer Mode in Discord:
   - User Settings > App Settings > Advanced > Developer Mode
2. Right-click on a server icon > Copy Server ID
3. Right-click on a category > Copy Category ID
