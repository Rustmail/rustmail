use crate::errors::dictionary::DictionaryMessage;
use crate::errors::dictionary::ErrorDictionary;

pub fn load_english_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        DictionaryMessage::new("Failed to connect to the database")
            .with_description("The bot couldn't establish a connection to the database"),
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
            "The specified channel doesn't exist or the bot doesn't have access to it",
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
            "The bot doesn't have the required permissions to perform this action",
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
        "discord.user_is_a_bot".to_string(),
        DictionaryMessage::new("The specified user is a bot."),
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
        DictionaryMessage::new("The specified user is a bot, please choose another one."),
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
            .with_description("The bot could not send the message to the thread channel."),
    );
    dict.messages.insert(
        "reply.send_failed_dm".to_string(),
        DictionaryMessage::new("Failed to send the message to the user in DM.")
            .with_description("The bot could not send the message to the user's DM."),
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
            .with_description("The bot configuration is incorrect"),
    );
    dict.messages.insert(
        "general.unknown_error".to_string(),
        DictionaryMessage::new("Unknown error: {message}")
            .with_description("An unexpected error occurred"),
    );

    dict.messages.insert(
        "recovery.messages_recovered".to_string(),
        DictionaryMessage::new("📥 **{count} message(s) recovered** during bot downtime")
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
        "alert.set_failed".to_string(),
        DictionaryMessage::new("❌ Failed to set alert")
            .with_description("An error occurred while setting the alert"),
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
        DictionaryMessage::new("❌ Failed to fetch server categories")
            .with_description("The bot couldn't retrieve the list of categories from the server"),
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
        DictionaryMessage::new("✅ Thread moved to category '{category}' by {staff}")
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
        DictionaryMessage::new("❌ You cannot create a thread for a bot user."),
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
        "slash_command.new_thread_user_id_argument".to_string(),
        DictionaryMessage::new("The ID of the user to create the thread for"),
    );
    dict.messages.insert(
        "slash_command.close_command_description".to_string(),
        DictionaryMessage::new("Close the current thread"),
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
}
