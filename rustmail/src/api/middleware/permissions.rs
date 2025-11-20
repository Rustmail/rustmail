use crate::db::repr::{ApiKey, Permission};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub struct PermissionError {
    pub required: &'static str,
}

impl IntoResponse for PermissionError {
    fn into_response(self) -> Response {
        (
            StatusCode::FORBIDDEN,
            format!("Missing required permission: {}", self.required),
        )
            .into_response()
    }
}

pub fn check_permission(api_key: &ApiKey, permission: Permission) -> Result<(), PermissionError> {
    if api_key.has_permission(permission) {
        Ok(())
    } else {
        Err(PermissionError {
            required: match permission {
                Permission::CreateTicket => "CREATE_TICKET",
                Permission::ReadTickets => "READ_TICKETS",
                Permission::UpdateTicket => "UPDATE_TICKET",
                Permission::DeleteTicket => "DELETE_TICKET",
                Permission::ReadConfig => "READ_CONFIG",
                Permission::UpdateConfig => "UPDATE_CONFIG",
                Permission::ManageBot => "MANAGE_BOT",
            },
        })
    }
}
