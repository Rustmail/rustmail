# Managing Tickets

This guide covers the ticket lifecycle and management features in Rustmail.

---

## Ticket Lifecycle

### 1. Creation

A ticket is created when:

- A user sends a direct message to the bot
- Staff uses `/new_thread` to initiate contact
- (Optional) A user creates a channel in a designated category

When created:
- A channel is created in the inbox category
- The user receives the `welcome_message`
- Staff can see the new channel

### 2. Active Conversation

During the ticket:
- User messages appear in the ticket channel
- Staff respond with `/reply` or `!reply`
- Messages are tracked with unique IDs
- Edit and delete history is preserved

### 3. Closure

Tickets are closed when:
- Staff uses `/close`
- A scheduled closure triggers
- The user leaves the server (if `close_on_leave` is enabled)

On closure:
- The user receives the `close_message` (unless silent)
- The channel is archived or deleted
- Records are preserved in the database

---

## Creating Tickets

### User-Initiated

Users open tickets by sending a DM to the bot:

1. User finds the bot in the server member list
2. User sends a direct message
3. Bot creates a ticket channel
4. Bot sends `welcome_message` to the user

### Staff-Initiated

Staff can create tickets for users:

```
/new_thread user:@Username
!new_thread 123456789012345678
```

Useful for:
- Proactive outreach
- Following up on issues
- Contacting users who can't DM

### Channel Creation (Optional)

If enabled in configuration:

```toml
[thread]
create_ticket_by_create_channel = true
```

Staff can create a channel in the inbox category to start a ticket. The channel name should be the user's ID.

---

## Responding to Tickets

### Standard Reply

```
/reply content:Hello, how can I help you today?
!reply Hello, how can I help you today?
```

The message is sent to the user's DMs and logged in the ticket channel.

### Anonymous Reply

Hide your identity from the user:

```
/reply content:This is from the staff team. anonymous:true
!anonreply This is from the staff team.
```

Anonymous messages show a generic staff label instead of your username.

### Attachments

**Slash command:**
```
/reply content:Here's the document you requested. attachment:<file>
```

**Text command:**
Upload the file with your message:
```
!reply Here's the document you requested.
[Attached file]
```

---

## Editing and Deleting Messages

### Message IDs

Each message has a unique ID shown in the footer. Use this ID for edit and delete operations.

### Editing

```
/edit message_id:42 new_content:Updated message text
!edit 42 Updated message text
```

Edits are:
- Reflected in the user's DMs
- Logged (if `show_log_on_edit` is enabled)
- Tracked in the database

### Deleting

```
/delete message_id:42
!delete 42
```

Deletions:
- Remove the message from user's DMs (if possible)
- Log the deletion (if `show_log_on_delete` is enabled)
- Mark the record as deleted in database

---

## Closing Tickets

### Immediate Close

```
/close
!close
```

The user receives `close_message` and the ticket is archived.

### Silent Close

```
/close silent:true
!close -s
```

No message is sent to the user.

### Scheduled Close

Schedule automatic closure:

```
/close time:2h
!close 2h
```

Supported formats: `30m`, `2h`, `1d`, `1d12h`

### Cancel Scheduled Close

```
/close cancel:true
!close -c
```

### Force Close

For orphaned tickets (user left the server):

```
/force_close
!force_close
```

---

## Staff Assignment

### Claiming a Ticket

```
/take
!take
```

Marks you as the assigned staff member.

### Releasing Assignment

```
/release
!release
```

Removes your assignment.

### Adding/Removing Staff Access

Grant specific staff access:
```
/add_staff user:@StaffMember
!add_staff 123456789012345678
```

Remove access:
```
/remove_staff user:@StaffMember
!remove_staff 123456789012345678
```

---

## Reminders and Alerts

### Setting Reminders

Get pinged after a delay:

```
/add_reminder time:2h content:Check if user responded
!add_reminder 2h Check if user responded
```

### Removing Reminders

```
/remove_reminder reminder_id:5
!remove_reminder 5
```

### Alerts

Get notified on the next user response:

```
/alert
!alert
```

Cancel an alert:
```
/alert cancel:true
!alert cancel
```

---

## Moving Tickets

Relocate a ticket to a different category:

```
/move_thread category:Escalated
!move_thread Escalated
```

Useful for:
- Escalation workflows
- Department routing
- Priority categorization

---

## Ticket Information

### User ID

```
/id
!id
```

Displays the ticket user's Discord ID.

### Logs

```
/logs
!logs
```

View the ticket's history and activity log.

---

## Message Recovery

If the bot goes offline, users may send messages that aren't immediately processed. Rustmail automatically recovers these messages when restarting.

Manual recovery:
```
/recover
!recover
```

---

## Configuration Options

Key settings affecting ticket behavior:

| Option                 | Description                        |
|------------------------|------------------------------------|
| `welcome_message`      | Sent when ticket opens             |
| `close_message`        | Sent when ticket closes            |
| `time_to_close_thread` | Default scheduled close time       |
| `close_on_leave`       | Auto-close when user leaves server |
| `embedded_message`     | Display messages as embeds         |
| `block_quote`          | Use block quotes for messages      |

See [Configuration Reference](../reference/configuration.md) for all options.
