use crate::prelude::errors::*;

pub fn load_english_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        DictionaryMessage::new("Failed to connect to the database")
            .with_description("The rustmail couldn't establish a connection to the database"),
    );
    dict.messages.insert(
        "database.query_failed".to_string(),
        DictionaryMessage::new("Database query failed: {error}")
            .with_description("A database operation failed"),
    );
    dict.messages.insert(
        "database.not_found".to_string(),
        DictionaryMessage::new("Record not found in database")
            .with_description("The requested data could not be found"),
    );
    dict.messages.insert(
        "discord.channel_not_found".to_string(),
        DictionaryMessage::new("Channel not found").with_description(
            "The specified channel doesn't exist or the rustmail doesn't have access to it",
        ),
    );
    dict.messages.insert(
        "discord.user_not_found".to_string(),
        DictionaryMessage::new("User not found")
            .with_description("The specified user doesn't exist or is not accessible"),
    );
    dict.messages.insert(
        "discord.permission_denied".to_string(),
        DictionaryMessage::new("Permission denied").with_description(
            "The rustmail doesn't have the required permissions to perform this action",
        ),
    );
    dict.messages.insert(
        "discord.dm_creation_failed".to_string(),
        DictionaryMessage::new("Failed to create DM channel")
            .with_description("Couldn't create a direct message channel with the user"),
    );
    dict.messages.insert(
        "discord.api_error".to_string(),
        DictionaryMessage::new("Discord API error")
            .with_description("An error occurred while communicating with Discord"),
    );
    dict.messages.insert(
        "discord.attachment_too_large".to_string(),
        DictionaryMessage::new("Your attachment is too large! Discord has a file size limit of 8 MB for attachments. Please reduce the file size or send a link."),
    );
    dict.messages.insert(
        "discord.user_is_a_bot".to_string(),
        DictionaryMessage::new("The specified user is a rustmail."),
    );
    dict.messages.insert(
        "discord.shard_manager_not_found".to_string(),
        DictionaryMessage::new("Shard manager not found."),
    );
    dict.messages.insert(
        "command.invalid_format".to_string(),
        DictionaryMessage::new("Invalid command format")
            .with_description("The command syntax is incorrect")
            .with_help("Use `{prefix}help` to see the correct command format"),
    );
    dict.messages.insert(
        "command.missing_arguments".to_string(),
        DictionaryMessage::new("Missing required arguments")
            .with_description("This command requires additional parameters"),
    );
    dict.messages.insert(
        "command.invalid_arguments".to_string(),
        DictionaryMessage::new("Invalid arguments: {arguments}")
            .with_description("One or more arguments are invalid"),
    );
    dict.messages.insert(
        "command.unknown_command".to_string(),
        DictionaryMessage::new("Unknown command: {command}")
            .with_description("The specified command doesn't exist")
            .with_help("Use `{prefix}help` to see available commands"),
    );
    dict.messages.insert(
        "command.unknown_slash_command".to_string(),
        DictionaryMessage::new("Unknown Slash Command: {command}"),
    );
    dict.messages.insert(
        "command.insufficient_permissions".to_string(),
        DictionaryMessage::new("Insufficient permissions")
            .with_description("You don't have the required permissions to use this command"),
    );
    dict.messages.insert(
        "thread.not_found".to_string(),
        DictionaryMessage::new("Thread not found")
            .with_description("No active thread found for this user or channel"),
    );
    dict.messages.insert(
        "thread.already_exists".to_string(),
        DictionaryMessage::new("Thread already exists")
            .with_description("You already have an active support thread"),
    );
    dict.messages.insert(
        "thread.creation_failed".to_string(),
        DictionaryMessage::new("Failed to create thread")
            .with_description("An error occurred while creating the support thread"),
    );
    dict.messages.insert(
        "snippet.already_exist".to_string(),
        DictionaryMessage::new("The snippet with key '{key}' already exists."),
    );
    dict.messages.insert(
        "thread.user_still_in_server".to_string(),
        DictionaryMessage::new("User still in the server.")
            .with_description("Use the 'close' command to close this ticket."),
    );
    dict.messages.insert(
        "thread.not_a_thread_channel".to_string(),
        DictionaryMessage::new("This channel is not a ticket channel."),
    );
    dict.messages.insert(
        "thread.modal_invalid_user_id".to_string(),
        DictionaryMessage::new("Invalid user ID"),
    );
    dict.messages.insert(
        "thread.category_not_found".to_string(),
        DictionaryMessage::new("Category not found in the server."),
    );
    dict.messages.insert(
        "message.not_found".to_string(),
        DictionaryMessage::new("Message not found")
            .with_description("The specified message could not be found"),
    );
    dict.messages.insert(
        "message.number_not_found".to_string(),
        DictionaryMessage::new("Message #{number} not found")
            .with_description("No message with this number exists"),
    );
    dict.messages.insert(
        "message.edit_failed".to_string(),
        DictionaryMessage::new("Failed to edit message")
            .with_description("An error occurred while editing the message"),
    );
    dict.messages.insert(
        "message.send_failed".to_string(),
        DictionaryMessage::new("Failed to send message")
            .with_description("An error occurred while sending the message"),
    );
    dict.messages.insert(
        "message.too_long".to_string(),
        DictionaryMessage::new("Message is too long")
            .with_description("Discord messages cannot exceed 2000 characters"),
    );
    dict.messages.insert(
        "message.empty".to_string(),
        DictionaryMessage::new("Message cannot be empty")
            .with_description("Please provide a message to send"),
    );
    dict.messages.insert(
        "validation.invalid_input".to_string(),
        DictionaryMessage::new("Invalid input: {input}")
            .with_description("The provided input is not valid"),
    );
    dict.messages.insert(
        "validation.out_of_range".to_string(),
        DictionaryMessage::new("Value out of range: {range}")
            .with_description("The value must be within the specified range"),
    );
    dict.messages.insert(
        "validation.required_field_missing".to_string(),
        DictionaryMessage::new("Required field missing: {field}")
            .with_description("This field is required and cannot be empty"),
    );
    dict.messages.insert(
        "permission.not_staff_member".to_string(),
        DictionaryMessage::new("You are not a staff member")
            .with_description("This command is only available to staff members"),
    );
    dict.messages.insert(
        "permission.user_blocked".to_string(),
        DictionaryMessage::new("User is blocked")
            .with_description("This user has been blocked from using the support system"),
    );
    dict.messages.insert(
        "success.message_sent".to_string(),
        DictionaryMessage::new("Message sent successfully! (Message #{number})")
            .with_description("Your message has been delivered")
            .with_help("Use `{prefix}edit {number}` to modify this message"),
    );
    dict.messages.insert(
        "success.message_edited".to_string(),
        DictionaryMessage::new("Message edited successfully")
            .with_description("The message has been updated in both the thread and DM"),
    );
    dict.messages.insert(
        "success.thread_created".to_string(),
        DictionaryMessage::new("Support thread created")
            .with_description("A new support thread has been created for you"),
    );
    dict.messages
        .insert("general.yes".to_string(), DictionaryMessage::new("Yes"));
    dict.messages
        .insert("general.no".to_string(), DictionaryMessage::new("No"));
    dict.messages.insert(
        "general.loading".to_string(),
        DictionaryMessage::new("Loading...")
            .with_description("Please wait while the operation completes"),
    );
    dict.messages.insert(
        "general.processing".to_string(),
        DictionaryMessage::new("Processing your request...")
            .with_description("This may take a few moments"),
    );
    dict.messages.insert(
        "thread.closed".to_string(),
        DictionaryMessage::new("Thank you for contacting support! Your ticket is now closed.")
            .with_description("The support ticket has been closed and the conversation ended."),
    );
    dict.messages.insert(
        "thread.ask_to_close".to_string(),
        DictionaryMessage::new("Close"),
    );
    dict.messages.insert(
        "thread.ask_to_keep_open".to_string(),
        DictionaryMessage::new("Keep open"),
    );
    dict.messages.insert(
        "thread.modal_bot_user".to_string(),
        DictionaryMessage::new("The specified user is a rustmail, please choose another one."),
    );
    dict.messages.insert(
        "thread.thread_closing".to_string(),
        DictionaryMessage::new(
            "The ticket will close in {seconds} seconds at the request of {user}.",
        ),
    );
    dict.messages.insert(
        "thread.action_in_progress".to_string(),
        DictionaryMessage::new("An action is already in progress, please wait."),
    );
    dict.messages.insert(
        "thread.modal_user_not_found".to_string(),
        DictionaryMessage::new("The specified user could not be found, please choose another one."),
    );
    dict.messages.insert(
        "thread.will_remain_open".to_string(),
        DictionaryMessage::new("The thread will remain open."),
    );
    dict.messages.insert(
        "thread.ask_create_ticket".to_string(),
        DictionaryMessage::new("This channel was created in the support ticket category. Would you like to create one?")
    );
    dict.messages.insert(
        "thread.modal_to_create_ticket".to_string(),
        DictionaryMessage::new("Create a ticket"),
    );
    dict.messages.insert(
        "thread.created".to_string(),
        DictionaryMessage::new("Ticket created: {channel}")
            .with_description("A new support ticket was opened or retrieved"),
    );
    dict.messages.insert(
        "thread.unknown_action".to_string(),
        DictionaryMessage::new("Unknown action")
            .with_description("The requested ticket action is unknown"),
    );
    dict.messages.insert(
        "reply.missing_content".to_string(),
        DictionaryMessage::new("Please provide a message to send to the user.")
            .with_description("You must provide a message to reply to the user."),
    );
    dict.messages.insert(
        "reply.send_failed_thread".to_string(),
        DictionaryMessage::new("Failed to send the message to the channel.")
            .with_description("The rustmail could not send the message to the thread channel."),
    );
    dict.messages.insert(
        "reply.send_failed_dm".to_string(),
        DictionaryMessage::new("Failed to send the message to the user in DM.")
            .with_description("The rustmail could not send the message to the user's DM."),
    );
    dict.messages.insert(
        "edit.validation.invalid_format".to_string(),
        DictionaryMessage::new("❌ Invalid command format. Usage: `edit <number> <new message>`")
            .with_description("The edit command format is invalid."),
    );
    dict.messages.insert(
        "edit.validation.missing_number".to_string(),
        DictionaryMessage::new(
            "❌ Invalid format. Message number is missing. Example: `edit 3 New message`",
        )
        .with_description("The message number is missing in the edit command."),
    );
    dict.messages.insert(
        "edit.validation.missing_content".to_string(),
        DictionaryMessage::new(
            "❌ Invalid format. Content is missing. Example: `edit 3 New message`",
        )
        .with_description("The new content is missing in the edit command."),
    );
    dict.messages.insert(
        "edit.validation.invalid_number".to_string(),
        DictionaryMessage::new("❌ The message number is invalid. It must be a positive number.")
            .with_description("The message number must be positive."),
    );
    dict.messages.insert(
        "edit.validation.empty_content".to_string(),
        DictionaryMessage::new("❌ The new message cannot be empty.")
            .with_description("The new message content cannot be empty."),
    );
    dict.messages.insert(
        "edit.modification_from_user".to_string(),
        DictionaryMessage::new(
            "The user edited their message.\n\nBefore:\n{before}\n\nAfter:\n{after}\n\nLink: {link}",
        ),
    );
    dict.messages.insert(
        "edit.modification_from_staff".to_string(),
        DictionaryMessage::new(
            "A staff edited their message.\n\nBefore:\n{before}\n\nAfter:\n{after}\n\nLink: {link}",
        ),
    );
    dict.messages.insert(
        "reply_numbering.confirmation".to_string(),
        DictionaryMessage::new("✅ Message sent! (Message #{number}) - Use `{prefix}edit {number}` to edit this message.")
            .with_description("Confirmation after sending a message with its number."),
    );
    dict.messages.insert(
        "reply_numbering.preview".to_string(),
        DictionaryMessage::new("(Message #{number} - Use `{prefix}edit {number}` to edit)")
            .with_description("Preview of the message number for editing."),
    );
    dict.messages.insert(
        "reply_numbering.footer".to_string(),
        DictionaryMessage::new("Message #{number} • {prefix}edit {number} to edit")
            .with_description("Footer for embeds with message number and edit command."),
    );
    dict.messages.insert(
        "reply_numbering.text_footer".to_string(),
        DictionaryMessage::new("*Message #{number} - `{prefix}edit {number}` to edit*")
            .with_description(
                "Footer for plain text messages with message number and edit command.",
            ),
    );
    dict.messages.insert(
        "permission.insufficient_permissions".to_string(),
        DictionaryMessage::new("Insufficient permissions")
            .with_description("You don't have the required permissions for this action"),
    );
    dict.messages.insert(
        "server.wrong_guild_single".to_string(),
        DictionaryMessage::new("Wrong server")
            .with_description("You must be in the main server to open a ticket")
            .with_help("Join the main server to contact support"),
    );
    dict.messages.insert(
        "server.wrong_guild_dual".to_string(),
        DictionaryMessage::new("Wrong server")
            .with_description("You must be in the community server to open a ticket")
            .with_help("Join the community server to contact support"),
    );
    dict.messages.insert(
        "server.not_in_community".to_string(),
        DictionaryMessage::new("User not found in community server")
            .with_description("The user must be a member of the community server"),
    );
    dict.messages.insert(
        "user.left_server".to_string(),
        DictionaryMessage::new("❌ **ERROR** : Unable to send message because user **{username}** is no longer a member of the community server.")
            .with_description("The user has left the community server"),
    );
    dict.messages.insert(
        "user.left_server_close".to_string(),
        DictionaryMessage::new("ℹ️ **INFORMATION** : The ticket has been closed. User **{username}** is no longer a member of the community server, so no closure message was sent to them.")
            .with_description("Information when closing a ticket for a user who has left"),
    );
    dict.messages.insert(
        "user.left_server_notification".to_string(),
        DictionaryMessage::new("⚠️ **ALERT** : User **{username}** (ID: {user_id}) has left the server.\n\nThe thread remains open but you can no longer send messages to this user.")
            .with_description("Notification when a user leaves the server"),
    );
    dict.messages.insert(
        "reply.user_not_found".to_string(),
        DictionaryMessage::new("User not found")
            .with_description("The user doesn't exist or is not accessible"),
    );
    dict.messages.insert(
        "config.invalid_configuration".to_string(),
        DictionaryMessage::new("Invalid configuration")
            .with_description("The rustmail configuration is incorrect"),
    );
    dict.messages.insert(
        "general.unknown_error".to_string(),
        DictionaryMessage::new("Unknown error: {message}")
            .with_description("An unexpected error occurred"),
    );

    dict.messages.insert(
        "recovery.messages_recovered".to_string(),
        DictionaryMessage::new("📥 **{count} message(s) recovered** during rustmail downtime")
            .with_description("Notification of recovered missing messages"),
    );
    dict.messages.insert(
        "recovery.summary".to_string(),
        DictionaryMessage::new("Recovery completed: {total} messages recovered in {threads} threads ({failed} failures)")
            .with_description("Summary of message recovery"),
    );
    dict.messages.insert(
        "recovery.started".to_string(),
        DictionaryMessage::new("🔄 Starting recovery of missing messages...")
            .with_description("Recovery start notification"),
    );
    dict.messages.insert(
        "recovery.completed".to_string(),
        DictionaryMessage::new("✅ Message recovery completed")
            .with_description("Recovery completion notification"),
    );
    dict.messages.insert(
        "alert.not_in_thread".to_string(),
        DictionaryMessage::new("❌ This command can only be used in a support thread")
            .with_description("The alert command must be used in a thread channel"),
    );
    dict.messages.insert(
        "alert.alert_not_found".to_string(),
        DictionaryMessage::new("No alert set for this thread"),
    );
    dict.messages.insert(
        "command.not_in_thread".to_string(),
        DictionaryMessage::new("This command can only be used in a support thread"),
    );
    dict.messages.insert(
        "alert.set_failed".to_string(),
        DictionaryMessage::new("You already have an alert for this thread!"),
    );
    dict.messages.insert(
        "alert.confirmation".to_string(),
        DictionaryMessage::new(
            "🔔 Alert set! You will be notified when {user} sends their next message",
        )
        .with_description("Confirmation that the alert has been set"),
    );
    dict.messages.insert(
        "alert.ping_message".to_string(),
        DictionaryMessage::new("**New message received from {user}!**")
            .with_description("Ping staff when user sends a new message after alert command"),
    );
    dict.messages.insert(
        "alert.cancel_failed".to_string(),
        DictionaryMessage::new("❌ Failed to cancel alert")
            .with_description("An error occurred while canceling the alert"),
    );
    dict.messages.insert(
        "alert.cancel_confirmation".to_string(),
        DictionaryMessage::new(
            "🔕 Alert canceled! You will no longer be notified when {user} sends message",
        )
        .with_description("Confirmation that the alert has been canceled"),
    );
    dict.messages.insert(
        "move_thread.not_in_thread".to_string(),
        DictionaryMessage::new("❌ This command can only be used in a support thread")
            .with_description("The move_thread command must be used in a thread channel"),
    );
    dict.messages.insert(
        "move_thread.missing_category".to_string(),
        DictionaryMessage::new(
            "❌ Please specify a category name. Usage: `{prefix}move_thread <category_name>`",
        )
        .with_description("The category name is missing in the move_thread command"),
    );
    dict.messages.insert(
        "move_thread.failed_to_fetch_categories".to_string(),
        DictionaryMessage::new("❌ Failed to fetch server categories").with_description(
            "The rustmail couldn't retrieve the list of categories from the server",
        ),
    );
    dict.messages.insert(
        "move_thread.category_not_found".to_string(),
        DictionaryMessage::new("❌ Category '{category}' not found")
            .with_description("No category with that name exists on the server"),
    );
    dict.messages.insert(
        "move_thread.failed_to_move".to_string(),
        DictionaryMessage::new("❌ Failed to move_thread thread to the specified category")
            .with_description("An error occurred while moving the thread"),
    );
    dict.messages.insert(
        "move_thread.success".to_string(),
        DictionaryMessage::new("✅ Thread moved to category **{category}** by <@{staff}>")
            .with_description("The thread has been successfully moved to the new category"),
    );
    dict.messages.insert(
        "new_thread.missing_user".to_string(),
        DictionaryMessage::new(
            "❌ Please specify a user. Usage: `{prefix}new <user_id_or_mention>`",
        )
        .with_description("The user ID or mention is missing in the new_thread command"),
    );
    dict.messages.insert(
        "new_thread.user_has_thread".to_string(),
        DictionaryMessage::new("❌ This user already has an active support thread")
            .with_description("The user already has an open thread"),
    );
    dict.messages.insert(
        "new_thread.user_has_thread_with_link".to_string(),
        DictionaryMessage::new(
            "❌ {user} already has an active support thread\n\n📎 **Thread link:** <#{channel_id}>",
        )
        .with_description("The user already has an open thread with a link to it"),
    );
    dict.messages.insert(
        "new_thread.user_not_found".to_string(),
        DictionaryMessage::new("❌ User not found")
            .with_description("The specified user doesn't exist or is not accessible"),
    );
    dict.messages.insert(
        "new_thread.user_not_in_community".to_string(),
        DictionaryMessage::new("❌ User is not a member of the community server")
            .with_description("The user must be in the community server to create a thread"),
    );
    dict.messages.insert(
        "new_thread.user_is_a_bot".to_string(),
        DictionaryMessage::new("❌ You cannot create a thread for a rustmail user."),
    );
    dict.messages.insert(
        "new_thread.channel_creation_failed".to_string(),
        DictionaryMessage::new("❌ Failed to create support thread channel")
            .with_description("An error occurred while creating the thread channel"),
    );
    dict.messages.insert(
        "new_thread.database_error".to_string(),
        DictionaryMessage::new("❌ Failed to create thread in database")
            .with_description("An error occurred while saving the thread to the database"),
    );
    dict.messages.insert(
        "new_thread.welcome_message".to_string(),
        DictionaryMessage::new("🎫 **Support thread created for {user}**\n\nThis thread has been created by staff. You can now communicate with the support team.")
            .with_description("Welcome message in the newly created thread"),
    );
    dict.messages.insert(
        "new_thread.dm_notification".to_string(),
        DictionaryMessage::new("🎫 **Support thread opened**\n\nA staff member has initiated a support conversation with you. You can now communicate with the support team.")
            .with_description("DM notification sent to the user when a thread is created"),
    );
    dict.messages.insert(
        "new_thread.success_with_dm".to_string(),
        DictionaryMessage::new("✅ Support thread created for {user} in {channel_id} by {staff}\n\nDM notification sent successfully.")
            .with_description("Success message when thread is created and DM is sent"),
    );
    dict.messages.insert(
        "new_thread.success_without_dm".to_string(),
        DictionaryMessage::new("✅ Support thread created for {user} in <#{channel_id}> by {staff}\n\n⚠️ Could not send DM notification (user may have DMs disabled).")
            .with_description("Success message when thread is created but DM fails"),
    );
    dict.messages.insert(
        "delete.not_in_thread".to_string(),
        DictionaryMessage::new("❌ This command can only be used in a support thread")
            .with_description("The delete command must be used in a thread channel"),
    );
    dict.messages.insert(
        "delete.missing_number".to_string(),
        DictionaryMessage::new(
            "❌ Please specify a message number. Usage: `{prefix}delete <number>`",
        )
        .with_description("The message number is missing in the delete command"),
    );
    dict.messages.insert(
        "delete.message_not_found".to_string(),
        DictionaryMessage::new("❌ Message #{number} not found")
            .with_description("No message with this number exists in this thread"),
    );
    dict.messages.insert(
        "command.discord_delete_failed".to_string(),
        DictionaryMessage::new("❌ Failed to delete message from Discord")
            .with_description("An error occurred while deleting the message from Discord"),
    );
    dict.messages.insert(
        "delete.database_delete_failed".to_string(),
        DictionaryMessage::new("❌ Failed to delete message from database")
            .with_description("An error occurred while deleting the message from the database"),
    );
    dict.messages.insert(
        "delete.success".to_string(),
        DictionaryMessage::new("✅ Message #{number} has been deleted successfully")
            .with_description("Confirmation that the message has been deleted"),
    );
    dict.messages.insert(
        "delete.removed_by_user".to_string(),
        DictionaryMessage::new("User {userid} deleted their message: \n\n{content}")
            .with_description(
                "Log entry when the end user deletes their DM message (mirrored in thread)",
            )
            .with_help("Parameters: content, number (optional if staff message)"),
    );
    dict.messages.insert(
        "delete.removed_by_staff".to_string(),
        DictionaryMessage::new("Staff {userid} deleted a message: \n\n{content}")
            .with_description("Log entry when a staff member deletes a message inside the thread or via DM mirror")
            .with_help("Parameters: content, number (optional), link (optional)"),
    );
    dict.messages.insert(
        "add_staff.add_success".to_string(),
        DictionaryMessage::new("The user {user} has been added to the ticket successfully."),
    );
    dict.messages.insert(
        "add_staff.remove_success".to_string(),
        DictionaryMessage::new("The user {user} has been removed from the ticket successfully."),
    );
    dict.messages.insert(
        "id.show_id".to_string(),
        DictionaryMessage::new("ID of {user} : {id}"),
    );
    dict.messages.insert(
        "close.closure_canceled".to_string(),
        DictionaryMessage::new("Closure canceled."),
    );
    dict.messages.insert(
        "close.auto_canceled_on_message".to_string(),
        DictionaryMessage::new(
            "Scheduled closure has been automatically canceled because a message was received.",
        ),
    );
    dict.messages.insert(
        "close.replacing_existing_closure".to_string(),
        DictionaryMessage::new("⚠️ Warning: A closure was already scheduled in {old_time}. It will be replaced by the new one."),
    );
    dict.messages.insert(
        "close.no_scheduled_closures_to_cancel".to_string(),
        DictionaryMessage::new("No scheduled closures to cancel."),
    );
    dict.messages.insert(
        "close.closure_already_scheduled".to_string(),
        DictionaryMessage::new(
            "A closing is already scheduled in {seconds} seconds. Use !close cancel to cancel it.",
        ),
    );
    dict.messages.insert(
        "close.closing".to_string(),
        DictionaryMessage::new("This ticket will be closed in {time}."),
    );
    dict.messages.insert(
        "close.silent_closing".to_string(),
        DictionaryMessage::new("This ticket will be closed silently in {time}."),
    );
    dict.messages.insert(
        "logs.ticket_closed".to_string(),
        DictionaryMessage::new("Ticket closed for user **{username}** (ID: {user_id})\n[View log on panel]({panel_url})"),
    );
    dict.messages.insert(
        "feature.not_implemented".to_string(),
        DictionaryMessage::new("This feature is not yet implemented."),
    );
    dict.messages.insert(
        "slash_command.id_command_description".to_string(),
        DictionaryMessage::new("Get ID of the user in the thread"),
    );
    dict.messages.insert(
        "slash_command.move_command_description".to_string(),
        DictionaryMessage::new("Move the current thread to another category"),
    );
    dict.messages.insert(
        "slash_command.move_command_name_argument".to_string(),
        DictionaryMessage::new("The name of the category to move the thread to"),
    );
    dict.messages.insert(
        "slash_command.new_thread_command_description".to_string(),
        DictionaryMessage::new("Create a new support thread for a user"),
    );
    dict.messages.insert(
        "slash_command.new_thread_user_id_argument".to_string(),
        DictionaryMessage::new("The ID of the user to create the thread for"),
    );
    dict.messages.insert(
        "slash_command.close_command_description".to_string(),
        DictionaryMessage::new("Close the current thread"),
    );
    dict.messages.insert(
        "slash_command.close_time_before_close_argument".to_string(),
        DictionaryMessage::new("The time to wait before closing the ticket (ex: 1s, 1m, 1h, 1d)"),
    );
    dict.messages.insert(
        "slash_command.close_silent_argument".to_string(),
        DictionaryMessage::new("Set to true to close the ticket without notifying the user"),
    );
    dict.messages.insert(
        "slash_command.close_cancel_argument".to_string(),
        DictionaryMessage::new("Set to true to cancel a scheduled closure"),
    );
    dict.messages.insert(
        "slash_command.edit_command_description".to_string(),
        DictionaryMessage::new("Edit a previously sent message"),
    );
    dict.messages.insert(
        "slash_command.edit_message_id_argument".to_string(),
        DictionaryMessage::new("The ID of the message to edit. You can find this ID by looking at the bottom of the message."),
    );
    dict.messages.insert(
        "slash_command.edit_message_argument".to_string(),
        DictionaryMessage::new("The new content for the message."),
    );
    dict.messages.insert(
        "slash_command.add_staff_command_description".to_string(),
        DictionaryMessage::new(
            "Add a staff member to the current ticket to which he does not have access",
        ),
    );
    dict.messages.insert(
        "slash_command.add_staff_user_id_argument".to_string(),
        DictionaryMessage::new("The ID of the staff to add to the ticket"),
    );
    dict.messages.insert(
        "slash_command.remove_staff_command_description".to_string(),
        DictionaryMessage::new("Remove a staff member from the current ticket"),
    );
    dict.messages.insert(
        "slash_command.remove_staff_user_id_argument".to_string(),
        DictionaryMessage::new("The ID of the staff to remove from the ticket"),
    );
    dict.messages.insert(
        "slash_command.alert_command_description".to_string(),
        DictionaryMessage::new(
            "Set or cancel an alert for the next message from the user in this thread",
        ),
    );
    dict.messages.insert(
        "slash_command.alert_cancel_argument".to_string(),
        DictionaryMessage::new("Set to true to cancel the alert"),
    );
    dict.messages.insert(
        "slash_command.force_close_command_description".to_string(),
        DictionaryMessage::new("Force close the current thread which the user has left the server"),
    );
    dict.messages.insert(
        "slash_command.reply_command_description".to_string(),
        DictionaryMessage::new("Send a message to the user in this thread"),
    );
    dict.messages.insert(
        "slash_command.reply_message_argument_description".to_string(),
        DictionaryMessage::new("The content of the message to send to the user"),
    );
    dict.messages.insert(
        "slash_command.reply_snippet_argument_description".to_string(),
        DictionaryMessage::new("Use a snippet instead of typing a message"),
    );
    dict.messages.insert(
        "slash_command.reply_attachment_argument_description".to_string(),
        DictionaryMessage::new("An optional attachment to send to the user"),
    );
    dict.messages.insert(
        "slash_command.reply_anonymous_argument_description".to_string(),
        DictionaryMessage::new("Send the message anonymously"),
    );
    dict.messages.insert(
        "slash_command.delete_command_description".to_string(),
        DictionaryMessage::new("Delete a message from the thread and the user's DM"),
    );
    dict.messages.insert(
        "slash_command.delete_message_id_argument_description".to_string(),
        DictionaryMessage::new("The ID of the message to delete. You can find this ID by looking at the bottom of the message."),
    );
    dict.messages.insert(
        "slash_command.recover_command_description".to_string(),
        DictionaryMessage::new(
            "Retrieve messages missed during the rustmail's downtime (This process is automatic).",
        ),
    );
    dict.messages.insert(
        "slash_command.help_command_description".to_string(),
        DictionaryMessage::new("Show the help message"),
    );
    dict.messages.insert(
        "reminder.registered_without_content".to_string(),
        DictionaryMessage::new("⏰ Reminder recorded for {time} ({remaining_time})!"),
    );
    dict.messages.insert(
        "reminder.registered_with_content".to_string(),
        DictionaryMessage::new("⏰ Reminder recorded for {time} ({remaining_time})!\n\n{content}"),
    );
    dict.messages.insert(
        "reminder.show_with_content".to_string(),
        DictionaryMessage::new("⏰ Reminder <@{user}>: \n\n{content} !"),
    );
    dict.messages.insert(
        "reminder.show_without_content".to_string(),
        DictionaryMessage::new("⏰ Reminder <@{user}>!"),
    );
    dict.messages.insert(
        "slash_command.add_reminder_command_description".to_string(),
        DictionaryMessage::new("Add a reminder for yourself"),
    );
    dict.messages.insert(
        "slash_command.add_reminder_time_argument_description".to_string(),
        DictionaryMessage::new("The time when the reminder should trigger (format: HH:MM)"),
    );
    dict.messages.insert(
        "slash_command.add_reminder_content_argument_description".to_string(),
        DictionaryMessage::new("Optional content for the reminder"),
    );
    dict.messages.insert(
        "remove_reminder.confirmation".to_string(),
        DictionaryMessage::new("Reminder **#{id}** has been removed successfully"),
    );
    dict.messages.insert(
        "slash_command.remove_reminder_command_description".to_string(),
        DictionaryMessage::new("Remove one of your reminders"),
    );
    dict.messages.insert(
        "slash_command.remove_reminder_id_argument".to_string(),
        DictionaryMessage::new("The ID of the reminder to remove"),
    );
    dict.messages.insert(
        "logs_command.next".to_string(),
        DictionaryMessage::new("Next"),
    );
    dict.messages.insert(
        "logs_command.prev".to_string(),
        DictionaryMessage::new("Previous"),
    );
    dict.messages.insert(
        "slash_commands.logs_command_description".to_string(),
        DictionaryMessage::new("View the logs of a specific user"),
    );
    dict.messages.insert(
        "slash_commands.logs_id_argument_description".to_string(),
        DictionaryMessage::new("The ID of the user to view logs for"),
    );
    dict.messages.insert(
        "slash_commands.no_logs_found".to_string(),
        DictionaryMessage::new("No logs found for this user."),
    );
    dict.messages.insert(
        "new_thread.show_logs".to_string(),
        DictionaryMessage::new(
            "This user has {count} previous rustmail ticket(s). Use `{prefix}logs` to view them.",
        ),
    );
    dict.messages.insert(
        "reminder.reminder_already_exists".to_string(),
        DictionaryMessage::new("You already have a reminder scheduled for that time."),
    );
    dict.messages.insert(
        "help.add_reminder".to_string(),
        DictionaryMessage::new("Sets a reminder for a specific time. To do so, use `!remind <HH:MM> <reminder content>` or `!rem <HH:MM> <reminder content>`. If the specified time has already passed today, the reminder will be scheduled for tomorrow."),
    );
    dict.messages.insert(
        "help.add_staff".to_string(),
        DictionaryMessage::new("Adds a staff member to a ticket. To do so, use `!addmod <staff_id>` or `!am <staff_id>` inside a ticket."),
    );
    dict.messages.insert(
        "help.alert".to_string(),
        DictionaryMessage::new("Sets an alert for a user when they send a new message. To create an alert, use `!alert` inside a ticket. To cancel a scheduled alert, use `!alert cancel` or `!alert c`."),
    );
    dict.messages.insert(
        "help.close".to_string(),
        DictionaryMessage::new("Closes the current ticket. You can specify a delay before closing using `!close <duration (d, h, m or s)>` or `!c <duration (d, h, m or s)>`. You can also add the `--silent` or `-s` option to avoid notifying the user that their ticket has been closed. To cancel a scheduled closure, use `!close --cancel`, `!close -c`, or `!close cancel`."),
    );
    dict.messages.insert(
        "help.delete".to_string(),
        DictionaryMessage::new("Deletes a specific message within a thread. To do so, use `!delete <message_id>` inside a ticket."),
    );
    dict.messages.insert(
        "help.edit".to_string(),
        DictionaryMessage::new("Edits the content of a previously sent message in a ticket. To edit a message, use `!edit <message_id> <new content>` or `!e <message_id> <new content>` inside a ticket."),
    );
    dict.messages.insert(
        "help.force_close".to_string(),
        DictionaryMessage::new("Force-closes a ticket when an error prevents normal closure. This command will be removed in future versions. To force-close a ticket, use `!force_close` or `!fc` inside a ticket."),
    );
    dict.messages.insert(
        "help.help".to_string(),
        DictionaryMessage::new("Displays a list of all available commands with a short description. To view the help message, use `!help`. If you want help with a specific command, type `!help <command_name>`."),
    );
    dict.messages.insert(
        "help.id".to_string(),
        DictionaryMessage::new("Displays the Discord ID of the user associated with the ticket. To view the user's ID, use `!id` inside a ticket."),
    );
    dict.messages.insert(
        "help.logs".to_string(),
        DictionaryMessage::new("Retrieves logs from all previous tickets of a user. You can either specify a Discord ID (`!logs <discord_id>`) or run the command inside a ticket to get that ticket’s logs."),
    );
    dict.messages.insert(
        "help.move".to_string(),
        DictionaryMessage::new("Moves the current ticket to another category. To move a ticket, use `!move <category>` or `!mv <category>` inside the ticket."),
    );
    dict.messages.insert(
        "help.new_thread".to_string(),
        DictionaryMessage::new("Creates a new ticket for a specified user. To create a ticket, use `!new_thread <user>` or `!nt <user>`."),
    );
    dict.messages.insert(
        "help.recover".to_string(),
        DictionaryMessage::new("Starts the process of recovering missing messages in Modmail tickets. This process runs automatically, but you can trigger it manually if needed. To do so, use `!recover`."),
    );
    dict.messages.insert(
        "help.remove_reminder".to_string(),
        DictionaryMessage::new("Deletes a reminder you previously set. To remove a reminder, use `!unremind <id>` or `!urem <id>`."),
    );
    dict.messages.insert(
        "help.remove_staff".to_string(),
        DictionaryMessage::new("Removes a staff member from the current ticket. To remove a staff member, use `!delmod <user>` or `!dm <user>` inside the ticket."),
    );
    dict.messages.insert(
        "help.reply".to_string(),
        DictionaryMessage::new("Replies in a ticket. To reply, use `!reply <message> [attachment]` or `!r <message> [attachment]` inside the ticket. If you want to reply anonymously, use `!anonreply`, `!ar`, or specify the option in the slash command `/reply`."),
    );
    dict.messages.insert(
        "help.message".to_string(),
        DictionaryMessage::new("## Commands:\n\n**All commands** are also available as **__slash commands__** with the **__same name__**.\n\nIf you want help with a specific command, type `!help <command_name>`.\n\n"),
    );
    dict.messages.insert(
        "help.take".to_string(),
        DictionaryMessage::new("Allows you to take ownership of a ticket by replacing its name with yours. To take a ticket, use `!take` in the ticket."),
    );
    dict.messages.insert(
        "help.release".to_string(),
        DictionaryMessage::new("Releases ownership of a ticket previously taken with the `!take` command. To release a ticket, use `!release` in the ticket."),
    );
    dict.messages.insert(
        "help.ping".to_string(),
        DictionaryMessage::new("Shows the actual latency of the bot."),
    );
    dict.messages.insert(
        "add_reminder.helper".to_string(),
        DictionaryMessage::new(
            "Incorrect format. Use : `{prefix}remind or {prefix}rem <HH:MM> [content]`",
        ),
    );
    dict.messages.insert(
        "take.ticket_already_taken".to_string(),
        DictionaryMessage::new("You have already taken this ticket."),
    );
    dict.messages.insert(
        "take.confirmation".to_string(),
        DictionaryMessage::new("The ticket is now taken by {staff}.\nDue to **Discord's API**, the channel name change may take up to **10 minutes**."),
    );
    dict.messages.insert(
        "take.timeout".to_string(),
        DictionaryMessage::new(
            "⚠️ **Discord’s API** enforces a limit of **2** channel updates every **10 minutes**.
            The action will be **__automatically__** applied once the cooldown expires.",
        ),
    );
    dict.messages.insert(
        "slash_command.take_command_description".to_string(),
        DictionaryMessage::new("Take ownership of the current ticket."),
    );
    dict.messages.insert(
        "slash_command.take_command_description".to_string(),
        DictionaryMessage::new("Take ownership of the current ticket."),
    );
    dict.messages.insert(
        "slash_command.release_command_description".to_string(),
        DictionaryMessage::new("Release ownership of the current ticket."),
    );
    dict.messages.insert(
        "release.ticket_already_taken".to_string(),
        DictionaryMessage::new("The ticket is not taken by anyone."),
    );
    dict.messages.insert(
        "release.confirmation".to_string(),
        DictionaryMessage::new("The ticket has been released by {staff}.\nDue to **Discord's API**, the channel name change may take up to **10 minutes**."),
    );
    dict.messages.insert(
        "slash_command.help_command_argument_desc".to_string(),
        DictionaryMessage::new("The command to get help with"),
    );
    dict.messages.insert(
        "slash_command.ping_command_desc".to_string(),
        DictionaryMessage::new("Check the Discord bot latency."),
    );
    dict.messages.insert(
        "slash_command.ping_command".to_string(),
        DictionaryMessage::new("## Latency\n\nGateway latency: **{gateway_latency}** ms\nMinimal REST latency (GET /gateway): **{api_latency}** ms\nREST latency (message send): **{message_latency}** ms"),
    );

    dict.messages.insert(
        "slash_command.snippet_command_description".to_string(),
        DictionaryMessage::new("Manage message snippets/templates"),
    );
    dict.messages.insert(
        "slash_command.snippet_command_help".to_string(),
        DictionaryMessage::new(
            "Manage message snippets/templates\n\n\
            **Subcommands:**\n\
            • `/snippet create <key> <content>` - Create a new snippet\n\
            • `/snippet list` - List all available snippets\n\
            • `/snippet show <key>` - Display a specific snippet's content\n\
            • `/snippet edit <key> <content>` - Update an existing snippet\n\
            • `/snippet delete <key>` - Delete a snippet\n\
            • `/snippet use <key>` - Use a snippet to reply\n\n\
            **Quick usage:**\n\
            • Slash command: `/snippet use <key>` or `/reply snippet:<key>`\n\
            • Text command: `!snippet <key>` or `!reply {{key}}`",
        ),
    );
    dict.messages.insert(
        "slash_command.snippet_create_description".to_string(),
        DictionaryMessage::new("Create a new snippet"),
    );
    dict.messages.insert(
        "slash_command.snippet_list_description".to_string(),
        DictionaryMessage::new("List all snippets"),
    );
    dict.messages.insert(
        "slash_command.snippet_show_description".to_string(),
        DictionaryMessage::new("Show a snippet"),
    );
    dict.messages.insert(
        "slash_command.snippet_edit_description".to_string(),
        DictionaryMessage::new("Edit a snippet"),
    );
    dict.messages.insert(
        "slash_command.snippet_delete_description".to_string(),
        DictionaryMessage::new("Delete a snippet"),
    );
    dict.messages.insert(
        "slash_command.snippet_use_description".to_string(),
        DictionaryMessage::new("Use a snippet to reply in a ticket"),
    );
    dict.messages.insert(
        "slash_command.snippet_key_argument".to_string(),
        DictionaryMessage::new("Snippet key (alphanumeric, dashes, underscores)"),
    );
    dict.messages.insert(
        "slash_command.snippet_content_argument".to_string(),
        DictionaryMessage::new("Snippet content (max 4000 characters)"),
    );
    dict.messages.insert(
        "slash_command.reply_snippet_argument".to_string(),
        DictionaryMessage::new("Use a snippet instead of typing a message"),
    );

    dict.messages.insert(
        "snippet.invalid_key_format".to_string(),
        DictionaryMessage::new(
            "Snippet key must contain only alphanumeric characters, dashes, and underscores.",
        ),
    );
    dict.messages.insert(
        "snippet.content_too_long".to_string(),
        DictionaryMessage::new("Snippet content must be 4000 characters or less."),
    );
    dict.messages.insert(
        "snippet.created".to_string(),
        DictionaryMessage::new("Snippet `{key}` created successfully!"),
    );
    dict.messages.insert(
        "snippet.creation_failed".to_string(),
        DictionaryMessage::new("Failed to create snippet: {error}"),
    );
    dict.messages.insert(
        "snippet.updated".to_string(),
        DictionaryMessage::new("Snippet `{key}` updated successfully!"),
    );
    dict.messages.insert(
        "snippet.update_failed".to_string(),
        DictionaryMessage::new("Failed to update snippet: {error}"),
    );
    dict.messages.insert(
        "snippet.deleted".to_string(),
        DictionaryMessage::new("Snippet `{key}` deleted successfully!"),
    );
    dict.messages.insert(
        "snippet.deletion_failed".to_string(),
        DictionaryMessage::new("Failed to delete snippet: {error}"),
    );
    dict.messages.insert(
        "snippet.not_found".to_string(),
        DictionaryMessage::new("Snippet `{key}` not found."),
    );
    dict.messages.insert(
        "snippet.list_empty".to_string(),
        DictionaryMessage::new("No snippets found."),
    );
    dict.messages.insert(
        "snippet.no_snippets_found".to_string(),
        DictionaryMessage::new("No snippets found."),
    );
    dict.messages.insert(
        "snippet.list_title".to_string(),
        DictionaryMessage::new("📝 Available Snippets"),
    );
    dict.messages.insert(
        "snippet.list_more".to_string(),
        DictionaryMessage::new("...and {count} more"),
    );
    dict.messages.insert(
        "snippet.show_title".to_string(),
        DictionaryMessage::new("📝 Snippet: {key}"),
    );
    dict.messages.insert(
        "snippet.created_by".to_string(),
        DictionaryMessage::new("Created by"),
    );
    dict.messages.insert(
        "snippet.created_at".to_string(),
        DictionaryMessage::new("Created at"),
    );
    dict.messages.insert(
        "snippet.unknown_subcommand".to_string(),
        DictionaryMessage::new("Unknown subcommand"),
    );
    dict.messages.insert(
        "snippet.text_usage".to_string(),
        DictionaryMessage::new("Usage: `!snippet <create|list|show|edit|delete> [args]`"),
    );
    dict.messages.insert(
        "snippet.text_create_usage".to_string(),
        DictionaryMessage::new("Usage: `!snippet create <key> <content>`"),
    );
    dict.messages.insert(
        "snippet.text_show_usage".to_string(),
        DictionaryMessage::new("Usage: `!snippet show <key>`"),
    );
    dict.messages.insert(
        "snippet.text_edit_usage".to_string(),
        DictionaryMessage::new("Usage: `!snippet edit <key> <content>`"),
    );
    dict.messages.insert(
        "snippet.text_delete_usage".to_string(),
        DictionaryMessage::new("Usage: `!snippet delete <key>`"),
    );
    dict.messages.insert(
        "snippet.unknown_text_subcommand".to_string(),
        DictionaryMessage::new(
            "Unknown subcommand. Use: `create`, `list`, `show`, `edit`, or `delete`",
        ),
    );
    dict.messages.insert(
        "snippet.used".to_string(),
        DictionaryMessage::new("Snippet '**{key}**' used successfully!"),
    );
    dict.messages.insert(
        "audit_log.reason".to_string(),
        DictionaryMessage::new("Reason"),
    );
    dict.messages.insert(
        "audit_log.channel".to_string(),
        DictionaryMessage::new("Channel"),
    );
    dict.messages.insert(
        "audit_log.target".to_string(),
        DictionaryMessage::new("Target"),
    );
    dict.messages.insert(
        "audit_log.unknown".to_string(),
        DictionaryMessage::new("Unknown"),
    );
    dict.messages.insert(
        "audit_log.unknown_action".to_string(),
        DictionaryMessage::new("Unknown Action (code: {code})"),
    );
    dict.messages.insert(
        "audit_log.member.kick".to_string(),
        DictionaryMessage::new("Member Kicked"),
    );
    dict.messages.insert(
        "audit_log.member.prune".to_string(),
        DictionaryMessage::new("Members Pruned"),
    );
    dict.messages.insert(
        "audit_log.member.ban_add".to_string(),
        DictionaryMessage::new("Member Banned"),
    );
    dict.messages.insert(
        "audit_log.member.ban_remove".to_string(),
        DictionaryMessage::new("Member Unbanned"),
    );
    dict.messages.insert(
        "audit_log.member.update".to_string(),
        DictionaryMessage::new("Member Updated"),
    );
    dict.messages.insert(
        "audit_log.member.role_update".to_string(),
        DictionaryMessage::new("Member Roles Updated"),
    );
    dict.messages.insert(
        "audit_log.member.member_move".to_string(),
        DictionaryMessage::new("Member Moved"),
    );
    dict.messages.insert(
        "audit_log.member.member_disconnect".to_string(),
        DictionaryMessage::new("Member Disconnected"),
    );
    dict.messages.insert(
        "audit_log.member.bot_add".to_string(),
        DictionaryMessage::new("Bot Added"),
    );
    dict.messages.insert(
        "audit_log.member.unknown".to_string(),
        DictionaryMessage::new("Unknown Member Action"),
    );
    dict.messages.insert(
        "audit_log.channel.create".to_string(),
        DictionaryMessage::new("Channel Created"),
    );
    dict.messages.insert(
        "audit_log.channel.update".to_string(),
        DictionaryMessage::new("Channel Updated"),
    );
    dict.messages.insert(
        "audit_log.channel.delete".to_string(),
        DictionaryMessage::new("Channel Deleted"),
    );
    dict.messages.insert(
        "audit_log.channel.unknown".to_string(),
        DictionaryMessage::new("Unknown Channel Action"),
    );
    dict.messages.insert(
        "audit_log.channel_overwrite.create".to_string(),
        DictionaryMessage::new("Permission Overwrite Created"),
    );
    dict.messages.insert(
        "audit_log.channel_overwrite.update".to_string(),
        DictionaryMessage::new("Permission Overwrite Updated"),
    );
    dict.messages.insert(
        "audit_log.channel_overwrite.delete".to_string(),
        DictionaryMessage::new("Permission Overwrite Deleted"),
    );
    dict.messages.insert(
        "audit_log.channel_overwrite.unknown".to_string(),
        DictionaryMessage::new("Unknown Permission Overwrite Action"),
    );
    dict.messages.insert(
        "audit_log.role.create".to_string(),
        DictionaryMessage::new("Role Created"),
    );
    dict.messages.insert(
        "audit_log.role.update".to_string(),
        DictionaryMessage::new("Role Updated"),
    );
    dict.messages.insert(
        "audit_log.role.delete".to_string(),
        DictionaryMessage::new("Role Deleted"),
    );
    dict.messages.insert(
        "audit_log.role.unknown".to_string(),
        DictionaryMessage::new("Unknown Role Action"),
    );
    dict.messages.insert(
        "audit_log.invite.create".to_string(),
        DictionaryMessage::new("Invite Created"),
    );
    dict.messages.insert(
        "audit_log.invite.update".to_string(),
        DictionaryMessage::new("Invite Updated"),
    );
    dict.messages.insert(
        "audit_log.invite.delete".to_string(),
        DictionaryMessage::new("Invite Deleted"),
    );
    dict.messages.insert(
        "audit_log.invite.unknown".to_string(),
        DictionaryMessage::new("Unknown Invite Action"),
    );
    dict.messages.insert(
        "audit_log.webhook.create".to_string(),
        DictionaryMessage::new("Webhook Created"),
    );
    dict.messages.insert(
        "audit_log.webhook.update".to_string(),
        DictionaryMessage::new("Webhook Updated"),
    );
    dict.messages.insert(
        "audit_log.webhook.delete".to_string(),
        DictionaryMessage::new("Webhook Deleted"),
    );
    dict.messages.insert(
        "audit_log.webhook.unknown".to_string(),
        DictionaryMessage::new("Unknown Webhook Action"),
    );
    dict.messages.insert(
        "audit_log.emoji.create".to_string(),
        DictionaryMessage::new("Emoji Created"),
    );
    dict.messages.insert(
        "audit_log.emoji.update".to_string(),
        DictionaryMessage::new("Emoji Updated"),
    );
    dict.messages.insert(
        "audit_log.emoji.delete".to_string(),
        DictionaryMessage::new("Emoji Deleted"),
    );
    dict.messages.insert(
        "audit_log.emoji.unknown".to_string(),
        DictionaryMessage::new("Unknown Emoji Action"),
    );
    dict.messages.insert(
        "audit_log.message.delete".to_string(),
        DictionaryMessage::new("Message Deleted"),
    );
    dict.messages.insert(
        "audit_log.message.bulk_delete".to_string(),
        DictionaryMessage::new("Messages Bulk Deleted"),
    );
    dict.messages.insert(
        "audit_log.message.pin".to_string(),
        DictionaryMessage::new("Message Pinned"),
    );
    dict.messages.insert(
        "audit_log.message.unpin".to_string(),
        DictionaryMessage::new("Message Unpinned"),
    );
    dict.messages.insert(
        "audit_log.message.unknown".to_string(),
        DictionaryMessage::new("Unknown Message Action"),
    );
    dict.messages.insert(
        "audit_log.guild.update".to_string(),
        DictionaryMessage::new("Server Updated"),
    );
    dict.messages.insert(
        "audit_log.integration.create".to_string(),
        DictionaryMessage::new("Integration Created"),
    );
    dict.messages.insert(
        "audit_log.integration.update".to_string(),
        DictionaryMessage::new("Integration Updated"),
    );
    dict.messages.insert(
        "audit_log.integration.delete".to_string(),
        DictionaryMessage::new("Integration Deleted"),
    );
    dict.messages.insert(
        "audit_log.integration.unknown".to_string(),
        DictionaryMessage::new("Unknown Integration Action"),
    );
    dict.messages.insert(
        "audit_log.stage_instance.create".to_string(),
        DictionaryMessage::new("Stage Instance Created"),
    );
    dict.messages.insert(
        "audit_log.stage_instance.update".to_string(),
        DictionaryMessage::new("Stage Instance Updated"),
    );
    dict.messages.insert(
        "audit_log.stage_instance.delete".to_string(),
        DictionaryMessage::new("Stage Instance Deleted"),
    );
    dict.messages.insert(
        "audit_log.stage_instance.unknown".to_string(),
        DictionaryMessage::new("Unknown Stage Instance Action"),
    );
    dict.messages.insert(
        "audit_log.sticker.create".to_string(),
        DictionaryMessage::new("Sticker Created"),
    );
    dict.messages.insert(
        "audit_log.sticker.update".to_string(),
        DictionaryMessage::new("Sticker Updated"),
    );
    dict.messages.insert(
        "audit_log.sticker.delete".to_string(),
        DictionaryMessage::new("Sticker Deleted"),
    );
    dict.messages.insert(
        "audit_log.sticker.unknown".to_string(),
        DictionaryMessage::new("Unknown Sticker Action"),
    );
    dict.messages.insert(
        "audit_log.scheduled_event.create".to_string(),
        DictionaryMessage::new("Scheduled Event Created"),
    );
    dict.messages.insert(
        "audit_log.scheduled_event.update".to_string(),
        DictionaryMessage::new("Scheduled Event Updated"),
    );
    dict.messages.insert(
        "audit_log.scheduled_event.delete".to_string(),
        DictionaryMessage::new("Scheduled Event Deleted"),
    );
    dict.messages.insert(
        "audit_log.scheduled_event.unknown".to_string(),
        DictionaryMessage::new("Unknown Scheduled Event Action"),
    );
    dict.messages.insert(
        "audit_log.thread.create".to_string(),
        DictionaryMessage::new("Thread Created"),
    );
    dict.messages.insert(
        "audit_log.thread.update".to_string(),
        DictionaryMessage::new("Thread Updated"),
    );
    dict.messages.insert(
        "audit_log.thread.delete".to_string(),
        DictionaryMessage::new("Thread Deleted"),
    );
    dict.messages.insert(
        "audit_log.thread.unknown".to_string(),
        DictionaryMessage::new("Unknown Thread Action"),
    );
    dict.messages.insert(
        "audit_log.automod.rule_create".to_string(),
        DictionaryMessage::new("AutoMod Rule Created"),
    );
    dict.messages.insert(
        "audit_log.automod.rule_update".to_string(),
        DictionaryMessage::new("AutoMod Rule Updated"),
    );
    dict.messages.insert(
        "audit_log.automod.rule_delete".to_string(),
        DictionaryMessage::new("AutoMod Rule Deleted"),
    );
    dict.messages.insert(
        "audit_log.automod.block_message".to_string(),
        DictionaryMessage::new("AutoMod Blocked Message"),
    );
    dict.messages.insert(
        "audit_log.automod.send_alert_message".to_string(),
        DictionaryMessage::new("AutoMod Sent Alert"),
    );
    dict.messages.insert(
        "audit_log.automod.user_communication_disabled".to_string(),
        DictionaryMessage::new("AutoMod Timed Out User"),
    );
    dict.messages.insert(
        "audit_log.automod.unknown".to_string(),
        DictionaryMessage::new("Unknown AutoMod Action"),
    );
    dict.messages.insert(
        "audit_log.creator_monetization.request_created".to_string(),
        DictionaryMessage::new("Monetization Request Created"),
    );
    dict.messages.insert(
        "audit_log.creator_monetization.terms_accepted".to_string(),
        DictionaryMessage::new("Monetization Terms Accepted"),
    );
    dict.messages.insert(
        "audit_log.creator_monetization.unknown".to_string(),
        DictionaryMessage::new("Unknown Monetization Action"),
    );
    dict.messages.insert(
        "audit_log.voice_channel_status.update".to_string(),
        DictionaryMessage::new("Voice Channel Status Updated"),
    );
    dict.messages.insert(
        "audit_log.voice_channel_status.delete".to_string(),
        DictionaryMessage::new("Voice Channel Status Deleted"),
    );
    dict.messages.insert(
        "audit_log.voice_channel_status.unknown".to_string(),
        DictionaryMessage::new("Unknown Voice Status Action"),
    );
    dict.messages.insert(
        "audit_log.change.afk_channel".to_string(),
        DictionaryMessage::new("AFK Channel"),
    );
    dict.messages.insert(
        "audit_log.change.afk_timeout".to_string(),
        DictionaryMessage::new("AFK Timeout"),
    );
    dict.messages.insert(
        "audit_log.change.permissions_allow".to_string(),
        DictionaryMessage::new("Permissions Allowed"),
    );
    dict.messages.insert(
        "audit_log.change.application".to_string(),
        DictionaryMessage::new("Application ID"),
    );
    dict.messages.insert(
        "audit_log.change.archived".to_string(),
        DictionaryMessage::new("Archived"),
    );
    dict.messages.insert(
        "audit_log.change.asset".to_string(),
        DictionaryMessage::new("Asset"),
    );
    dict.messages.insert(
        "audit_log.change.auto_archive_duration".to_string(),
        DictionaryMessage::new("Auto Archive Duration"),
    );
    dict.messages.insert(
        "audit_log.change.available".to_string(),
        DictionaryMessage::new("Available"),
    );
    dict.messages.insert(
        "audit_log.change.avatar".to_string(),
        DictionaryMessage::new("Avatar"),
    );
    dict.messages.insert(
        "audit_log.change.banner".to_string(),
        DictionaryMessage::new("Banner"),
    );
    dict.messages.insert(
        "audit_log.change.bitrate".to_string(),
        DictionaryMessage::new("Bitrate"),
    );
    dict.messages.insert(
        "audit_log.change.channel".to_string(),
        DictionaryMessage::new("Channel"),
    );
    dict.messages.insert(
        "audit_log.change.invite_code".to_string(),
        DictionaryMessage::new("Invite Code"),
    );
    dict.messages.insert(
        "audit_log.change.color".to_string(),
        DictionaryMessage::new("Color"),
    );
    dict.messages.insert(
        "audit_log.change.timeout".to_string(),
        DictionaryMessage::new("Timeout"),
    );
    dict.messages.insert(
        "audit_log.change.deaf".to_string(),
        DictionaryMessage::new("Deafened"),
    );
    dict.messages.insert(
        "audit_log.change.default_auto_archive".to_string(),
        DictionaryMessage::new("Default Auto Archive"),
    );
    dict.messages.insert(
        "audit_log.change.default_notifications".to_string(),
        DictionaryMessage::new("Default Notifications"),
    );
    dict.messages.insert(
        "audit_log.change.permissions_deny".to_string(),
        DictionaryMessage::new("Permissions Denied"),
    );
    dict.messages.insert(
        "audit_log.change.description".to_string(),
        DictionaryMessage::new("Description"),
    );
    dict.messages.insert(
        "audit_log.change.discovery_splash".to_string(),
        DictionaryMessage::new("Discovery Splash"),
    );
    dict.messages.insert(
        "audit_log.change.enable_emoticons".to_string(),
        DictionaryMessage::new("Enable Emoticons"),
    );
    dict.messages.insert(
        "audit_log.change.entity_type".to_string(),
        DictionaryMessage::new("Entity Type"),
    );
    dict.messages.insert(
        "audit_log.change.expire_behavior".to_string(),
        DictionaryMessage::new("Expire Behavior"),
    );
    dict.messages.insert(
        "audit_log.change.expire_grace_period".to_string(),
        DictionaryMessage::new("Expire Grace Period"),
    );
    dict.messages.insert(
        "audit_log.change.explicit_content_filter".to_string(),
        DictionaryMessage::new("Explicit Content Filter"),
    );
    dict.messages.insert(
        "audit_log.change.format_type".to_string(),
        DictionaryMessage::new("Format Type"),
    );
    dict.messages.insert(
        "audit_log.change.guild".to_string(),
        DictionaryMessage::new("Server ID"),
    );
    dict.messages.insert(
        "audit_log.change.hoist".to_string(),
        DictionaryMessage::new("Hoisted"),
    );
    dict.messages.insert(
        "audit_log.change.icon".to_string(),
        DictionaryMessage::new("Icon"),
    );
    dict.messages.insert(
        "audit_log.change.id".to_string(),
        DictionaryMessage::new("ID"),
    );
    dict.messages.insert(
        "audit_log.change.image".to_string(),
        DictionaryMessage::new("Image"),
    );
    dict.messages.insert(
        "audit_log.change.invitable".to_string(),
        DictionaryMessage::new("Invitable"),
    );
    dict.messages.insert(
        "audit_log.change.inviter".to_string(),
        DictionaryMessage::new("Inviter"),
    );
    dict.messages.insert(
        "audit_log.change.location".to_string(),
        DictionaryMessage::new("Location"),
    );
    dict.messages.insert(
        "audit_log.change.locked".to_string(),
        DictionaryMessage::new("Locked"),
    );
    dict.messages.insert(
        "audit_log.change.max_age".to_string(),
        DictionaryMessage::new("Max Age"),
    );
    dict.messages.insert(
        "audit_log.change.max_uses".to_string(),
        DictionaryMessage::new("Max Uses"),
    );
    dict.messages.insert(
        "audit_log.change.mentionable".to_string(),
        DictionaryMessage::new("Mentionable"),
    );
    dict.messages.insert(
        "audit_log.change.mfa_level".to_string(),
        DictionaryMessage::new("MFA Level"),
    );
    dict.messages.insert(
        "audit_log.change.mute".to_string(),
        DictionaryMessage::new("Muted"),
    );
    dict.messages.insert(
        "audit_log.change.name".to_string(),
        DictionaryMessage::new("Name"),
    );
    dict.messages.insert(
        "audit_log.change.nickname".to_string(),
        DictionaryMessage::new("Nickname"),
    );
    dict.messages.insert(
        "audit_log.change.nsfw".to_string(),
        DictionaryMessage::new("NSFW"),
    );
    dict.messages.insert(
        "audit_log.change.owner".to_string(),
        DictionaryMessage::new("Owner"),
    );
    dict.messages.insert(
        "audit_log.change.permission_overwrites".to_string(),
        DictionaryMessage::new("Permission Overwrites"),
    );
    dict.messages.insert(
        "audit_log.change.permissions".to_string(),
        DictionaryMessage::new("Permissions"),
    );
    dict.messages.insert(
        "audit_log.change.position".to_string(),
        DictionaryMessage::new("Position"),
    );
    dict.messages.insert(
        "audit_log.change.preferred_locale".to_string(),
        DictionaryMessage::new("Preferred Locale"),
    );
    dict.messages.insert(
        "audit_log.change.privacy_level".to_string(),
        DictionaryMessage::new("Privacy Level"),
    );
    dict.messages.insert(
        "audit_log.change.prune_delete_days".to_string(),
        DictionaryMessage::new("Prune Delete Days"),
    );
    dict.messages.insert(
        "audit_log.change.public_updates_channel".to_string(),
        DictionaryMessage::new("Public Updates Channel"),
    );
    dict.messages.insert(
        "audit_log.change.slowmode".to_string(),
        DictionaryMessage::new("Slowmode"),
    );
    dict.messages.insert(
        "audit_log.change.region".to_string(),
        DictionaryMessage::new("Region"),
    );
    dict.messages.insert(
        "audit_log.change.roles_added".to_string(),
        DictionaryMessage::new("Roles Added"),
    );
    dict.messages.insert(
        "audit_log.change.roles_removed".to_string(),
        DictionaryMessage::new("Roles Removed"),
    );
    dict.messages.insert(
        "audit_log.change.rules_channel".to_string(),
        DictionaryMessage::new("Rules Channel"),
    );
    dict.messages.insert(
        "audit_log.change.splash".to_string(),
        DictionaryMessage::new("Splash"),
    );
    dict.messages.insert(
        "audit_log.change.status".to_string(),
        DictionaryMessage::new("Status"),
    );
    dict.messages.insert(
        "audit_log.change.system_channel".to_string(),
        DictionaryMessage::new("System Channel"),
    );
    dict.messages.insert(
        "audit_log.change.tags".to_string(),
        DictionaryMessage::new("Tags"),
    );
    dict.messages.insert(
        "audit_log.change.temporary".to_string(),
        DictionaryMessage::new("Temporary"),
    );
    dict.messages.insert(
        "audit_log.change.topic".to_string(),
        DictionaryMessage::new("Topic"),
    );
    dict.messages.insert(
        "audit_log.change.type".to_string(),
        DictionaryMessage::new("Type"),
    );
    dict.messages.insert(
        "audit_log.change.unicode_emoji".to_string(),
        DictionaryMessage::new("Unicode Emoji"),
    );
    dict.messages.insert(
        "audit_log.change.user_limit".to_string(),
        DictionaryMessage::new("User Limit"),
    );
    dict.messages.insert(
        "audit_log.change.uses".to_string(),
        DictionaryMessage::new("Uses"),
    );
    dict.messages.insert(
        "audit_log.change.vanity_url".to_string(),
        DictionaryMessage::new("Vanity URL"),
    );
    dict.messages.insert(
        "audit_log.change.verification_level".to_string(),
        DictionaryMessage::new("Verification Level"),
    );
    dict.messages.insert(
        "audit_log.change.widget_channel".to_string(),
        DictionaryMessage::new("Widget Channel"),
    );
    dict.messages.insert(
        "audit_log.change.widget_enabled".to_string(),
        DictionaryMessage::new("Widget Enabled"),
    );
    dict.messages.insert(
        "audit_log.change.system_channel_flags".to_string(),
        DictionaryMessage::new("System Channel Flags"),
    );
}
