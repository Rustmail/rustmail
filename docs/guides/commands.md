# Commands Reference

Rustmail provides both slash commands and text commands. Text commands use a configurable prefix (default: `!`).

---

## Command Types

| Type  | Example        | Description                                       |
|-------|----------------|---------------------------------------------------|
| Slash | `/reply hello` | Discord's native slash commands with autocomplete |
| Text  | `!reply hello` | Traditional prefix-based commands                 |

Both types offer identical functionality. Slash commands provide better discoverability and parameter hints, while text commands may be faster for experienced users.

---

## General Commands

### help

Display available commands.

| Slash   | Text    |
|---------|---------|
| `/help` | `!help` |

### ping

Check bot responsiveness.

| Slash   | Text    |
|---------|---------|
| `/ping` | `!ping` |

---

## Ticket Management

### new_thread

Create a new ticket for a user. Use when you need to initiate contact.

| Slash                | Text                    |
|----------------------|-------------------------|
| `/new_thread <user>` | `!new_thread <user_id>` |

**Parameters:**
- `user` - The Discord user to create a ticket for

### close

Close the current ticket.

| Slash                             | Text                                          |
|-----------------------------------|-----------------------------------------------|
| `/close [time] [silent] [cancel]` | `!close [time] [-s\|--silent] [-c\|--cancel]` |

**Parameters:**
- `time` - Schedule closure after duration (e.g., `1h`, `30m`, `2d`)
- `silent` - Close without notifying the user
- `cancel` - Cancel a scheduled closure

**Examples:**
```
/close                    # Close immediately
/close time:2h            # Close in 2 hours
/close silent:true        # Close without notification
/close cancel:true        # Cancel scheduled closure

!close                    # Close immediately
!close 2h                 # Close in 2 hours
!close -s                 # Close silently
!close cancel             # Cancel scheduled closure
```

### force_close

Close an orphaned ticket when the user has left the server.

| Slash          | Text           |
|----------------|----------------|
| `/force_close` | `!force_close` |

### move_thread

Move a ticket to a different category.

| Slash                     | Text                           |
|---------------------------|--------------------------------|
| `/move_thread <category>` | `!move_thread <category_name>` |

**Parameters:**
- `category` - Target category name

### recover

Manually trigger message recovery. Rustmail automatically recovers messages sent while the bot was offline, but you can force a recovery check with this command.

| Slash      | Text       |
|------------|------------|
| `/recover` | `!recover` |

---

## Messaging

### reply

Send a message to the ticket user.

| Slash                                       | Text               |
|---------------------------------------------|--------------------|
| `/reply <content> [attachment] [anonymous]` | `!reply <content>` |

**Parameters:**
- `content` - Message text
- `attachment` - Optional file attachment
- `anonymous` - Send without revealing staff identity

For text commands, attach files using Discord's upload feature. For anonymous replies, use `!anonreply`.

### anonreply

Send an anonymous reply (text command only).

| Text                   |
|------------------------|
| `!anonreply <content>` |

Equivalent to `/reply anonymous:true`.

### edit

Edit a previous message you sent.

| Slash                              | Text                               |
|------------------------------------|------------------------------------|
| `/edit <message_id> <new_content>` | `!edit <message_id> <new_content>` |

**Parameters:**
- `message_id` - Rustmail message ID (shown in message footer)
- `new_content` - Updated message text

### delete

Delete a message you sent.

| Slash                  | Text                   |
|------------------------|------------------------|
| `/delete <message_id>` | `!delete <message_id>` |

**Parameters:**
- `message_id` - Rustmail message ID (shown in message footer)

---

## Staff Management

### add_staff

Grant a staff member access to the current ticket.

| Slash               | Text                   |
|---------------------|------------------------|
| `/add_staff <user>` | `!add_staff <user_id>` |

**Parameters:**
- `user` - Staff member to add

### remove_staff

Remove a staff member's access to the current ticket.

| Slash                  | Text                      |
|------------------------|---------------------------|
| `/remove_staff <user>` | `!remove_staff <user_id>` |

**Parameters:**
- `user` - Staff member to remove

### take

Assign yourself to the ticket (claim ownership).

| Slash   | Text    |
|---------|---------|
| `/take` | `!take` |

### release

Release your assignment from the ticket.

| Slash      | Text       |
|------------|------------|
| `/release` | `!release` |

---

## Reminders

### remind

Set a reminder for the current ticket. Reminders can target yourself or specific roles.

| Slash                                    | Text                                        |
|------------------------------------------|---------------------------------------------|
| `/remind <time> <content> [roles]`       | `!rem <time> [@roles] [content]`            |

**Parameters:**
- `time` - When to trigger in HH:MM format (e.g., `14:30`, `09:00`)
- `content` - Reminder message
- `roles` - Optional: roles to ping when the reminder triggers

**Examples:**
```
/remind time:14:30 content:Follow up with user
/remind time:09:00 content:Team meeting roles:@dev,@mod

!rem 14:30 Follow up with user
!rem 14:30 @dev @mod Team meeting
!rem 09:00 @support Check ticket status
```

**Notes:**
- If the specified time has already passed today, the reminder will be scheduled for tomorrow
- When targeting roles, only members who are subscribed to that role's reminders will be pinged
- You can use Discord role mentions or `@rolename` syntax in text commands

### remove_reminder

Cancel a scheduled reminder.

| Slash                            | Text                             |
|----------------------------------|----------------------------------|
| `/remove_reminder <reminder_id>` | `!remove_reminder <reminder_id>` |

Aliases: `!unremind`, `!urem`

**Parameters:**
- `reminder_id` - ID of the reminder to cancel

### reminder_subscription

Manage your role-based reminder subscriptions. By default, you receive pings for all roles you have. Use this to opt out of specific role reminders.

| Slash                                           | Text                            |
|-------------------------------------------------|---------------------------------|
| `/reminder_subscription <action> <role>`        | `!rem subscribe <role>`         |
|                                                 | `!rem unsubscribe <role>`       |

**Parameters:**
- `action` - Either `subscribe` or `unsubscribe`
- `role` - The role to manage subscription for

**Requirements:**
- You must have the role to modify your subscription for it

**Examples:**
```
/reminder_subscription action:unsubscribe role:@dev
/reminder_subscription action:subscribe role:@support

!rem unsubscribe dev
!rem subscribe @support
```

---

## Alerts

### alert

Get notified when the ticket receives a new message.

| Slash             | Text              |
|-------------------|-------------------|
| `/alert [cancel]` | `!alert [cancel]` |

**Parameters:**
- `cancel` - Remove an existing alert

---

## Information

### id

Display the ticket user's Discord ID.

| Slash | Text  |
|-------|-------|
| `/id` | `!id` |

### logs

View ticket history and logs.

| Slash   | Text    |
|---------|---------|
| `/logs` | `!logs` |

### status

View or change the bot's operational status.

| Slash                  | Text                   |
|------------------------|------------------------|
| `/status [new_status]` | `!status [new_status]` |

---

## Snippets

Snippets are saved responses that can be quickly inserted.

### snippet

Use a saved snippet.

| Slash            | Text             |
|------------------|------------------|
| `/snippet <key>` | `!snippet <key>` |

Snippet management is done through the web panel or database directly.

---

## Time Format Reference

For commands accepting time durations:

| Unit | Example | Description |
|------|---------|-------------|
| `s`  | `30s`   | Seconds     |
| `m`  | `15m`   | Minutes     |
| `h`  | `2h`    | Hours       |
| `d`  | `1d`    | Days        |

Combinations are supported: `1d12h` = 1 day and 12 hours.

---

## Command Permissions

Commands are available to users with access to ticket channels. The bot respects Discord's permission system:

- Only staff with channel access can use ticket commands
- Super admins (configured in `config.toml`) have full access
- Additional permissions can be configured through the panel
