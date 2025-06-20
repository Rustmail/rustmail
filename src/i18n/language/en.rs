use crate::errors::dictionary::ErrorDictionary;
use crate::errors::dictionary::ErrorMessage;

pub fn load_english_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert(
        "database.connection_failed".to_string(),
        ErrorMessage::new("Failed to connect to the database")
            .with_description("The bot couldn't establish a connection to the database")
            .with_help(
                "Check database configuration and ensure the database server is running",
            ),
    );
    dict.messages.insert(
        "database.query_failed".to_string(),
        ErrorMessage::new("Database query failed: {error}")
            .with_description("A database operation failed unexpectedly"),
    );
    dict.messages.insert(
        "database.not_found".to_string(),
        ErrorMessage::new("Record not found in database")
            .with_description("The requested data could not be found"),
    );
    dict.messages.insert(
        "discord.channel_not_found".to_string(),
        ErrorMessage::new("Channel not found").with_description(
            "The specified channel doesn't exist or the bot doesn't have access to it",
        ),
    );
    dict.messages.insert(
        "discord.user_not_found".to_string(),
        ErrorMessage::new("User not found")
            .with_description("The specified user doesn't exist or is not accessible"),
    );
    dict.messages.insert(
        "discord.permission_denied".to_string(),
        ErrorMessage::new("Permission denied").with_description(
            "The bot doesn't have the required permissions to perform this action",
        ),
    );
    dict.messages.insert(
        "discord.dm_creation_failed".to_string(),
        ErrorMessage::new("Failed to create DM channel")
            .with_description("Couldn't create a direct message channel with the user"),
    );
    dict.messages.insert(
        "command.invalid_format".to_string(),
        ErrorMessage::new("Invalid command format")
            .with_description("The command syntax is incorrect")
            .with_help("Use `{prefix}help` to see the correct command format"),
    );
    dict.messages.insert(
        "command.missing_arguments".to_string(),
        ErrorMessage::new("Missing required arguments")
            .with_description("This command requires additional parameters"),
    );
    dict.messages.insert(
        "command.invalid_arguments".to_string(),
        ErrorMessage::new("Invalid arguments: {arguments}")
            .with_description("One or more arguments are invalid"),
    );
    dict.messages.insert(
        "command.unknown_command".to_string(),
        ErrorMessage::new("Unknown command: {command}")
            .with_description("The specified command doesn't exist")
            .with_help("Use `{prefix}help` to see available commands"),
    );
    dict.messages.insert(
        "command.insufficient_permissions".to_string(),
        ErrorMessage::new("Insufficient permissions")
            .with_description("You don't have the required permissions to use this command"),
    );
    dict.messages.insert(
        "thread.not_found".to_string(),
        ErrorMessage::new("Thread not found")
            .with_description("No active thread found for this user or channel"),
    );
    dict.messages.insert(
        "thread.already_exists".to_string(),
        ErrorMessage::new("Thread already exists")
            .with_description("You already have an active support thread"),
    );
    dict.messages.insert(
        "thread.creation_failed".to_string(),
        ErrorMessage::new("Failed to create thread")
            .with_description("An error occurred while creating the support thread"),
    );
    dict.messages.insert(
        "message.not_found".to_string(),
        ErrorMessage::new("Message not found")
            .with_description("The specified message could not be found"),
    );
    dict.messages.insert(
        "message.number_not_found".to_string(),
        ErrorMessage::new("Message #{number} not found")
            .with_description("No message with this number exists"),
    );
    dict.messages.insert(
        "message.edit_failed".to_string(),
        ErrorMessage::new("Failed to edit message")
            .with_description("An error occurred while editing the message"),
    );
    dict.messages.insert(
        "message.send_failed".to_string(),
        ErrorMessage::new("Failed to send message")
            .with_description("An error occurred while sending the message"),
    );
    dict.messages.insert(
        "message.too_long".to_string(),
        ErrorMessage::new("Message is too long")
            .with_description("Discord messages cannot exceed 2000 characters"),
    );
    dict.messages.insert(
        "message.empty".to_string(),
        ErrorMessage::new("Message cannot be empty")
            .with_description("Please provide a message to send"),
    );
    dict.messages.insert(
        "validation.invalid_input".to_string(),
        ErrorMessage::new("Invalid input: {input}")
            .with_description("The provided input is not valid"),
    );
    dict.messages.insert(
        "validation.out_of_range".to_string(),
        ErrorMessage::new("Value out of range: {range}")
            .with_description("The value must be within the specified range"),
    );
    dict.messages.insert(
        "validation.required_field_missing".to_string(),
        ErrorMessage::new("Required field missing: {field}")
            .with_description("This field is required and cannot be empty"),
    );
    dict.messages.insert(
        "permission.not_staff_member".to_string(),
        ErrorMessage::new("You are not a staff member")
            .with_description("This command is only available to staff members"),
    );
    dict.messages.insert(
        "permission.user_blocked".to_string(),
        ErrorMessage::new("User is blocked")
            .with_description("This user has been blocked from using the support system"),
    );
    dict.messages.insert(
        "success.message_sent".to_string(),
        ErrorMessage::new("Message sent successfully! (Message #{number})")
            .with_description("Your message has been delivered")
            .with_help("Use `{prefix}edit {number}` to modify this message"),
    );
    dict.messages.insert(
        "success.message_edited".to_string(),
        ErrorMessage::new("Message edited successfully")
            .with_description("The message has been updated in both the thread and DM"),
    );
    dict.messages.insert(
        "success.thread_created".to_string(),
        ErrorMessage::new("Support thread created")
            .with_description("A new support thread has been created for you"),
    );
    dict.messages.insert(
        "general.loading".to_string(),
        ErrorMessage::new("Loading...")
            .with_description("Please wait while the operation completes"),
    );
    dict.messages.insert(
        "general.processing".to_string(),
        ErrorMessage::new("Processing your request...")
            .with_description("This may take a few moments"),
    );
    dict.messages.insert(
        "thread.closed".to_string(),
        ErrorMessage::new("Thank you for contacting support! Your ticket is now closed.")
            .with_description("The support ticket has been closed and the conversation ended."),
    );
    dict.messages.insert(
        "reply.missing_content".to_string(),
        ErrorMessage::new("Please provide a message to send to the user.")
            .with_description("You must provide a message to reply to the user."),
    );
    dict.messages.insert(
        "reply.send_failed_thread".to_string(),
        ErrorMessage::new("Failed to send the message to the channel.")
            .with_description("The bot could not send the message to the thread channel."),
    );
    dict.messages.insert(
        "reply.send_failed_dm".to_string(),
        ErrorMessage::new("Failed to send the message to the user in DM.")
            .with_description("The bot could not send the message to the user's DM."),
    );
    dict.messages.insert(
        "edit.validation.invalid_format".to_string(),
        ErrorMessage::new("❌ Invalid command format. Usage: `edit <number> <new message>`")
            .with_description("The edit command format is invalid."),
    );
    dict.messages.insert(
        "edit.validation.missing_number".to_string(),
        ErrorMessage::new("❌ Invalid format. Message number is missing. Example: `edit 3 New message`")
            .with_description("The message number is missing in the edit command."),
    );
    dict.messages.insert(
        "edit.validation.missing_content".to_string(),
        ErrorMessage::new("❌ Invalid format. Content is missing. Example: `edit 3 New message`")
            .with_description("The new content is missing in the edit command."),
    );
    dict.messages.insert(
        "edit.validation.invalid_number".to_string(),
        ErrorMessage::new("❌ The message number is invalid. It must be a positive number.")
            .with_description("The message number must be positive."),
    );
    dict.messages.insert(
        "edit.validation.empty_content".to_string(),
        ErrorMessage::new("❌ The new message cannot be empty.")
            .with_description("The new message content cannot be empty."),
    );
    dict.messages.insert(
        "reply_numbering.confirmation".to_string(),
        ErrorMessage::new("✅ Message sent! (Message #{number}) - Use `{prefix}edit {number}` to edit this message.")
            .with_description("Confirmation after sending a message with its number."),
    );
    dict.messages.insert(
        "reply_numbering.preview".to_string(),
        ErrorMessage::new("(Message #{number} - Use `{prefix}edit {number}` to edit)")
            .with_description("Preview of the message number for editing."),
    );
    dict.messages.insert(
        "reply_numbering.footer".to_string(),
        ErrorMessage::new("Message #{number} • {prefix}edit {number} to edit")
            .with_description("Footer for embeds with message number and edit command."),
    );
    dict.messages.insert(
        "reply_numbering.text_footer".to_string(),
        ErrorMessage::new("*Message #{number} - `{prefix}edit {number}` to edit*")
            .with_description("Footer for plain text messages with message number and edit command."),
    );
} 