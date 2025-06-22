pub mod init;
pub mod messages;
pub mod threads;

pub use init::{
    init_database, get_last_recovery_timestamp, update_last_recovery_timestamp,
};
pub use messages::{
    get_latest_thread_message, get_message_ids_by_number,
    insert_recovered_message, insert_staff_message,
    update_message_content, get_message_ids_by_message_id, insert_user_message_with_ids,
    delete_message, update_message_numbers_after_deletion,
};
pub use threads::{
    close_thread, create_thread, create_thread_for_user, get_all_opened_threads, get_thread_by_channel_id,
    get_thread_channel_by_user_id, get_thread_id_by_user_id, get_user_id_from_channel_id,
    increment_message_number, thread_exists, update_thread_user_left, is_user_left,
    get_next_message_number, set_alert_for_staff, cancel_alert_for_staff, get_staff_alerts_for_user, mark_alert_as_used,
};
