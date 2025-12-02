use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub fn to_display(&self) -> &str {
        match self {
            PanelPermission::ViewPanel => "View Panel",
            PanelPermission::ManageBot => "Manage Bot",
            PanelPermission::ManageConfig => "Manage Config",
            PanelPermission::ManageTickets => "Manage Tickets",
            PanelPermission::ManageApiKeys => "Manage API Keys",
            PanelPermission::ManagePermissions => "Manage Permissions",
        }
    }
}
