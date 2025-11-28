use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelPermission {
    ViewPanel,
    ManageBot,
    ManageConfig,
    ManageTickets,
    ManageApiKeys,
    ManagePermissions,
}

impl PanelPermission {
    pub fn as_str(&self) -> &'static str {
        match self {
            PanelPermission::ViewPanel => "view_panel",
            PanelPermission::ManageBot => "manage_bot",
            PanelPermission::ManageConfig => "manage_config",
            PanelPermission::ManageTickets => "manage_tickets",
            PanelPermission::ManageApiKeys => "manage_api_keys",
            PanelPermission::ManagePermissions => "manage_permissions",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "view_panel" => Some(PanelPermission::ViewPanel),
            "manage_bot" => Some(PanelPermission::ManageBot),
            "manage_config" => Some(PanelPermission::ManageConfig),
            "manage_tickets" => Some(PanelPermission::ManageTickets),
            "manage_api_keys" => Some(PanelPermission::ManageApiKeys),
            "manage_permissions" => Some(PanelPermission::ManagePermissions),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubjectType {
    User,
    Role,
}

impl SubjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SubjectType::User => "user",
            SubjectType::Role => "role",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "user" => Some(SubjectType::User),
            "role" => Some(SubjectType::Role),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelPermissionEntry {
    pub id: i64,
    pub subject_type: SubjectType,
    pub subject_id: String,
    pub permission: PanelPermission,
    pub granted_by: String,
    pub granted_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantPermissionRequest {
    pub subject_type: SubjectType,
    pub subject_id: String,
    pub permission: PanelPermission,
}
