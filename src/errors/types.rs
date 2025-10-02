use serenity::Error as SerenityError;
use sqlx::Error as SqlxError;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug, Clone)]
pub enum ModmailError {
    Database(DatabaseError),

    Discord(DiscordError),

    Command(CommandError),

    Thread(ThreadError),

    Message(MessageError),

    Config(ConfigError),

    Validation(ValidationError),

    Permission(PermissionError),

    Generic(String),
}

#[derive(Debug, Clone)]
pub enum DatabaseError {
    ConnectionFailed,
    QueryFailed(String),
    TransactionFailed,
    MigrationFailed,
    InvalidData(String),
    NotFound(String),
    InsertFailed(String),
    UpdateFailed(String),
    DeleteFailed(String),
}

#[derive(Debug, Clone)]
pub enum DiscordError {
    ApiError(String),
    ChannelNotFound,
    UserNotFound,
    UserIsABot,
    GuildNotFound,
    MessageNotFound,
    PermissionDenied,
    RateLimited,
    InvalidToken,
    WebhookError(String),
    EmbedTooLarge,
    MessageTooLong,
    ChannelCreationFailed,
    DmCreationFailed,
    FailedToFetchCategories,
    FailedToMoveChannel,
}

#[derive(Debug, Clone)]
pub enum CommandError {
    InvalidFormat,
    MissingArguments,
    InvalidArguments(String),
    UnknownCommand(String),
    UnknownSlashCommand(String),
    CommandFailed(String),
    InsufficientPermissions,
    CommandNotAvailable,
    CooldownActive(u64),
    NotInThread(),
    UserHasAlreadyAThread(),
    UserHasAlreadyAThreadWithLink(String, String),
    ClosureAlreadyScheduled,
    NoSchedulableClosureToCancel,
    SendDmFailed,
}

#[derive(Debug, Clone)]
pub enum ThreadError {
    ThreadNotFound,
    ThreadAlreadyExists,
    ThreadCreationFailed,
    ThreadClosingFailed,
    InvalidThreadState,
    UserNotInThread,
    ChannelNotThread,
    ThreadExpired,
    UserNotInTheServer,
    UserStillInServer,
    NotAThreadChannel,
    CategoryNotFound,
}

#[derive(Debug, Clone)]
pub enum MessageError {
    MessageNotFound(String),
    MessageTooLong,
    MessageEmpty,
    AttachmentTooLarge,
    AttachmentDownloadFailed,
    EditFailed(String),
    DeleteFailed(String),
    SendFailed(String),
    InvalidMessageFormat,
    MessageNumberNotFound(i64),
    DuplicateMessageNumber,
    DmAccessFailed(String),
}

#[derive(Debug, Clone)]
pub enum ConfigError {
    FileNotFound,
    ParseError(String),
    MissingField(String),
    InvalidValue(String),
    InvalidColor(String),
    InvalidChannelId,
    InvalidGuildId,
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidInput(String),
    OutOfRange(String),
    TooShort(String),
    TooLong(String),
    InvalidFormat(String),
    RequiredFieldMissing(String),
    InvalidCharacters(String),
}

#[derive(Debug, Clone)]
pub enum PermissionError {
    InsufficientPermissions,
    NotStaffMember,
    NotThreadOwner,
    BotMissingPermissions(String),
    UserBlocked,
    ChannelNotAccessible,
}

impl fmt::Display for ModmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModmailError::Database(err) => write!(f, "Database error: {}", err),
            ModmailError::Discord(err) => write!(f, "Discord error: {}", err),
            ModmailError::Command(err) => write!(f, "Command error: {}", err),
            ModmailError::Thread(err) => write!(f, "Thread error: {}", err),
            ModmailError::Message(err) => write!(f, "Message error: {}", err),
            ModmailError::Config(err) => write!(f, "Config error: {}", err),
            ModmailError::Validation(err) => write!(f, "Validation error: {}", err),
            ModmailError::Permission(err) => write!(f, "Permission error: {}", err),
            ModmailError::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::ConnectionFailed => write!(f, "Failed to connect to database"),
            DatabaseError::QueryFailed(query) => write!(f, "Query failed: {}", query),
            DatabaseError::TransactionFailed => write!(f, "Transaction failed"),
            DatabaseError::MigrationFailed => write!(f, "Database migration failed"),
            DatabaseError::InvalidData(data) => write!(f, "Invalid data: {}", data),
            DatabaseError::NotFound(item) => write!(f, "Not found: {}", item),
            DatabaseError::InsertFailed(item) => write!(f, "Failed to insert: {}", item),
            DatabaseError::UpdateFailed(item) => write!(f, "Failed to update: {}", item),
            DatabaseError::DeleteFailed(item) => write!(f, "Failed to delete: {}", item),
        }
    }
}

impl fmt::Display for DiscordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscordError::ApiError(msg) => write!(f, "Discord API error: {}", msg),
            DiscordError::ChannelNotFound => write!(f, "Channel not found"),
            DiscordError::UserNotFound => write!(f, "User not found"),
            DiscordError::UserIsABot => write!(f, "User is a bot"),
            DiscordError::GuildNotFound => write!(f, "Guild not found"),
            DiscordError::MessageNotFound => write!(f, "Message not found"),
            DiscordError::PermissionDenied => write!(f, "Permission denied"),
            DiscordError::RateLimited => write!(f, "Rate limited"),
            DiscordError::InvalidToken => write!(f, "Invalid bot token"),
            DiscordError::WebhookError(msg) => write!(f, "Webhook error: {}", msg),
            DiscordError::EmbedTooLarge => write!(f, "Embed too large"),
            DiscordError::MessageTooLong => write!(f, "Message too long"),
            DiscordError::ChannelCreationFailed => write!(f, "Failed to create channel"),
            DiscordError::DmCreationFailed => write!(f, "Failed to create DM channel"),
            DiscordError::FailedToFetchCategories => write!(f, "Failed to fetch categories"),
            DiscordError::FailedToMoveChannel => write!(f, "Failed to move_thread channel"),
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::InvalidFormat => write!(f, "Invalid command format"),
            CommandError::MissingArguments => write!(f, "Missing arguments"),
            CommandError::InvalidArguments(arg) => write!(f, "Invalid arguments: {}", arg),
            CommandError::UnknownCommand(cmd) => write!(f, "Unknown command: {}", cmd),
            CommandError::UnknownSlashCommand(cmd) => write!(f, "Unknown slash command: {}", cmd),
            CommandError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            CommandError::InsufficientPermissions => write!(f, "Insufficient permissions"),
            CommandError::CommandNotAvailable => write!(f, "Command not available"),
            CommandError::CooldownActive(seconds) => {
                write!(f, "Cooldown active: {} seconds", seconds)
            }
            CommandError::NotInThread() => write!(f, "This command can only be used in a thread"),
            CommandError::UserHasAlreadyAThread() => {
                write!(f, "The user already has an open thread")
            }
            CommandError::UserHasAlreadyAThreadWithLink(user, channel_id) => write!(
                f,
                "The user {} already has an open thread: {}",
                user, channel_id
            ),
            CommandError::ClosureAlreadyScheduled => {
                write!(f, "Thread closure is already scheduled")
            }
            CommandError::NoSchedulableClosureToCancel => {
                write!(f, "No schedulable closure to cancel")
            }
            CommandError::SendDmFailed => write!(f, "Failed to send DM to user"),
        }
    }
}

impl fmt::Display for ThreadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThreadError::ThreadNotFound => write!(f, "Thread not found"),
            ThreadError::ThreadAlreadyExists => write!(f, "Thread already exists"),
            ThreadError::ThreadCreationFailed => write!(f, "Failed to create thread"),
            ThreadError::ThreadClosingFailed => write!(f, "Failed to close thread"),
            ThreadError::InvalidThreadState => write!(f, "Invalid thread state"),
            ThreadError::UserNotInThread => write!(f, "User not in thread"),
            ThreadError::ChannelNotThread => write!(f, "Channel is not a thread"),
            ThreadError::ThreadExpired => write!(f, "Thread has expired"),
            ThreadError::UserNotInTheServer => write!(f, "User has left the server"),
            ThreadError::UserStillInServer => write!(
                f,
                "User still in the server. Use the 'close' command to close this ticket."
            ),
            ThreadError::NotAThreadChannel => write!(f, "This channel is not a ticket channel"),
            ThreadError::CategoryNotFound => write!(f, "Category not found"),
        }
    }
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageError::MessageNotFound(msg) => write!(f, "Message not found: {}", msg),
            MessageError::MessageTooLong => write!(f, "Message too long"),
            MessageError::MessageEmpty => write!(f, "Message is empty"),
            MessageError::AttachmentTooLarge => write!(f, "Attachment too large"),
            MessageError::AttachmentDownloadFailed => write!(f, "Failed to download attachment"),
            MessageError::EditFailed(msg) => write!(f, "Failed to edit message: {}", msg),
            MessageError::DeleteFailed(msg) => write!(f, "Failed to delete message: {}", msg),
            MessageError::SendFailed(msg) => write!(f, "Failed to send message: {}", msg),
            MessageError::InvalidMessageFormat => write!(f, "Invalid message format"),
            MessageError::MessageNumberNotFound(num) => {
                write!(f, "Message number {} not found", num)
            }
            MessageError::DuplicateMessageNumber => write!(f, "Duplicate message number"),
            MessageError::DmAccessFailed(msg) => write!(f, "Dm access failed: {}", msg),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound => write!(f, "Configuration file not found"),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::MissingField(field) => write!(f, "Missing field: {}", field),
            ConfigError::InvalidValue(value) => write!(f, "Invalid value: {}", value),
            ConfigError::InvalidColor(color) => write!(f, "Invalid color: {}", color),
            ConfigError::InvalidChannelId => write!(f, "Invalid channel ID"),
            ConfigError::InvalidGuildId => write!(f, "Invalid guild ID"),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidInput(input) => write!(f, "Invalid input: {}", input),
            ValidationError::OutOfRange(range) => write!(f, "Out of range: {}", range),
            ValidationError::TooShort(field) => write!(f, "Too short: {}", field),
            ValidationError::TooLong(field) => write!(f, "Too long: {}", field),
            ValidationError::InvalidFormat(format) => write!(f, "Invalid format: {}", format),
            ValidationError::RequiredFieldMissing(field) => {
                write!(f, "Required field missing: {}", field)
            }
            ValidationError::InvalidCharacters(chars) => write!(f, "Invalid characters: {}", chars),
        }
    }
}

impl fmt::Display for PermissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PermissionError::InsufficientPermissions => write!(f, "Insufficient permissions"),
            PermissionError::NotStaffMember => write!(f, "Not a staff member"),
            PermissionError::NotThreadOwner => write!(f, "Not the thread owner"),
            PermissionError::BotMissingPermissions(perm) => {
                write!(f, "Bot missing permissions: {}", perm)
            }
            PermissionError::UserBlocked => write!(f, "User is blocked"),
            PermissionError::ChannelNotAccessible => write!(f, "Channel not accessible"),
        }
    }
}

impl From<SqlxError> for ModmailError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::RowNotFound => {
                ModmailError::Database(DatabaseError::NotFound("Row".to_string()))
            }
            SqlxError::Database(db_err) => {
                ModmailError::Database(DatabaseError::QueryFailed(db_err.to_string()))
            }
            _ => ModmailError::Database(DatabaseError::QueryFailed(err.to_string())),
        }
    }
}

impl From<SerenityError> for ModmailError {
    fn from(err: SerenityError) -> Self {
        match err {
            SerenityError::Http(http_err) => {
                ModmailError::Discord(DiscordError::ApiError(http_err.to_string()))
            }
            SerenityError::Model(model_err) => {
                ModmailError::Discord(DiscordError::ApiError(model_err.to_string()))
            }
            _ => ModmailError::Discord(DiscordError::ApiError(err.to_string())),
        }
    }
}

impl From<toml::de::Error> for ModmailError {
    fn from(err: toml::de::Error) -> Self {
        ModmailError::Config(ConfigError::ParseError(err.to_string()))
    }
}

impl From<io::Error> for ModmailError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => ModmailError::Config(ConfigError::FileNotFound),
            _ => ModmailError::Generic(err.to_string()),
        }
    }
}

#[macro_export]
macro_rules! database_error {
    ($variant:ident) => {
        ModmailError::Database(DatabaseError::$variant)
    };
    ($variant:ident, $msg:expr) => {
        ModmailError::Database(DatabaseError::$variant($msg.to_string()))
    };
}

#[macro_export]
macro_rules! discord_error {
    ($variant:ident) => {
        ModmailError::Discord(DiscordError::$variant)
    };
    ($variant:ident, $msg:expr) => {
        ModmailError::Discord(DiscordError::$variant($msg.to_string()))
    };
}

#[macro_export]
macro_rules! command_error {
    ($variant:ident) => {
        ModmailError::Command(CommandError::$variant)
    };
    ($variant:ident, $msg:expr) => {
        ModmailError::Command(CommandError::$variant($msg.to_string()))
    };
}

#[macro_export]
macro_rules! thread_error {
    ($variant:ident) => {
        ModmailError::Thread(ThreadError::$variant)
    };
}

#[macro_export]
macro_rules! message_error {
    ($variant:ident) => {
        ModmailError::Message(MessageError::$variant)
    };
    ($variant:ident, $msg:expr) => {
        ModmailError::Message(MessageError::$variant($msg.to_string()))
    };
}

#[macro_export]
macro_rules! validation_error {
    ($variant:ident, $msg:expr) => {
        ModmailError::Validation(ValidationError::$variant($msg.to_string()))
    };
}

#[macro_export]
macro_rules! permission_error {
    ($variant:ident) => {
        ModmailError::Permission(PermissionError::$variant)
    };
    ($variant:ident, $msg:expr) => {
        ModmailError::Permission(PermissionError::$variant($msg.to_string()))
    };
}

pub type ModmailResult<T> = Result<T, ModmailError>;

impl error::Error for ModmailError {}
impl error::Error for DatabaseError {}
impl error::Error for DiscordError {}
impl error::Error for CommandError {}
impl error::Error for ThreadError {}
impl error::Error for MessageError {}
impl error::Error for ConfigError {}
impl error::Error for ValidationError {}
impl error::Error for PermissionError {}
