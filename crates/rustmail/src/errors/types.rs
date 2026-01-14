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
    NotFound(String),
    InsertFailed(String),
}

#[derive(Debug, Clone)]
pub enum DiscordError {
    ApiError(String),
    UserNotFound,
    UserIsABot,
    ChannelCreationFailed,
    FailedToFetchCategories,
    FailedToMoveChannel,
    ShardManagerNotFound,
}

#[derive(Debug, Clone)]
pub enum CommandError {
    InvalidFormat,
    MissingArguments,
    InvalidArguments(String),
    UnknownCommand(String),
    CommandFailed(String),
    NotInThread(),
    UserHasAlreadyAThread(),
    UserHasAlreadyAThreadWithLink(String, String),
    ClosureAlreadyScheduled,
    NoSchedulableClosureToCancel,
    SendDmFailed,
    DiscordDeleteFailed,
    ReminderAlreadyExists,
    AlertDoesNotExist,
    InvalidReminderFormat,
    TicketAlreadyTaken,
    TicketAlreadyReleased,
    AlertSetFailed,
    SnippetNotFound(String),
    SnippetAlreadyExists(String),
    InvalidSnippetKeyFormat,
    SnippetContentTooLong,
    StatusIsMissing,
    InvalidStatusValue,
    MaintenanceModeNotAllowed,
    ReminderAlreadySubscribed(String),
    ReminderAlreadyUnsubscribed(String),
    ReminderRoleRequired(String),
    ReminderRoleNotFound(String),
    ReminderAlreadyCompleted(String),
}

#[derive(Debug, Clone)]
pub enum ThreadError {
    ThreadNotFound,
    UserNotInTheServer,
    UserStillInServer,
    NotAThreadChannel,
    CategoryNotFound,
}

#[derive(Debug, Clone)]
pub enum MessageError {
    MessageNotFound(String),
    MessageEmpty,
    EditFailed(String),
    DmAccessFailed(String),
}

#[derive(Debug, Clone)]
pub enum ConfigError {
    FileNotFound,
    ParseError(String),
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidInput(String),
    RequiredFieldMissing(String),
}

#[derive(Debug, Clone)]
pub enum PermissionError {
    InsufficientPermissions,
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
            DatabaseError::NotFound(item) => write!(f, "Not found: {}", item),
            DatabaseError::InsertFailed(item) => write!(f, "Failed to insert: {}", item),
        }
    }
}

impl fmt::Display for DiscordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscordError::ApiError(msg) => write!(f, "Discord API error: {}", msg),
            DiscordError::UserNotFound => write!(f, "User not found"),
            DiscordError::UserIsABot => write!(f, "User is a rustmail"),
            DiscordError::ChannelCreationFailed => write!(f, "Failed to create channel"),
            DiscordError::FailedToFetchCategories => write!(f, "Failed to fetch categories"),
            DiscordError::FailedToMoveChannel => write!(f, "Failed to move_thread channel"),
            DiscordError::ShardManagerNotFound => write!(f, "Shard manager not found"),
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
            CommandError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
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
            CommandError::DiscordDeleteFailed => write!(f, "Failed to delete message on Discord"),
            CommandError::ReminderAlreadyExists => write!(f, "A reminder already exists"),
            CommandError::AlertDoesNotExist => write!(f, "Alert does not exist"),
            CommandError::InvalidReminderFormat => write!(f, "Invalid reminder format"),
            CommandError::TicketAlreadyTaken => write!(f, "Ticket already taken"),
            CommandError::TicketAlreadyReleased => write!(f, "Ticket already released"),
            CommandError::AlertSetFailed => write!(f, "Alert set failed"),
            CommandError::SnippetNotFound(name) => write!(f, "Snippet not found: {}", name),
            CommandError::InvalidSnippetKeyFormat => write!(f, "Invalid snippet key format"),
            CommandError::SnippetContentTooLong => write!(f, "Snippet content is too long"),
            CommandError::SnippetAlreadyExists(name) => {
                write!(f, "Snippet already exists: {}", name)
            }
            CommandError::StatusIsMissing => write!(f, "Status is missing"),
            CommandError::InvalidStatusValue => write!(f, "Invalid status value"),
            CommandError::MaintenanceModeNotAllowed => {
                write!(f, "Only admins can enable maintenance mode")
            }
            CommandError::ReminderAlreadySubscribed(role) => {
                write!(f, "Already subscribed to reminders for role {}", role)
            }
            CommandError::ReminderAlreadyUnsubscribed(role) => {
                write!(f, "Already unsubscribed from reminders for role {}", role)
            }
            CommandError::ReminderRoleRequired(role) => {
                write!(f, "You must have the {} role to perform this action", role)
            }
            CommandError::ReminderRoleNotFound(role) => {
                write!(f, "Role {} not found", role)
            }
            CommandError::ReminderAlreadyCompleted(reminder_id) => {
                write!(f, "Reminder {} has already been completed", reminder_id)
            }
        }
    }
}

impl fmt::Display for ThreadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThreadError::ThreadNotFound => write!(f, "Thread not found"),
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
            MessageError::MessageEmpty => write!(f, "Message is empty"),
            MessageError::EditFailed(msg) => write!(f, "Failed to edit message: {}", msg),
            MessageError::DmAccessFailed(msg) => write!(f, "Dm access failed: {}", msg),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound => write!(f, "Configuration file not found"),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidInput(input) => write!(f, "Invalid input: {}", input),
            ValidationError::RequiredFieldMissing(field) => {
                write!(f, "Required field missing: {}", field)
            }
        }
    }
}

impl fmt::Display for PermissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PermissionError::InsufficientPermissions => write!(f, "Insufficient permissions"),
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
