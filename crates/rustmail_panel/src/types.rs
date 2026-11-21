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
    ManageCategories,
    ViewBans,
}

impl PanelPermission {
    pub const ALL: [PanelPermission; 8] = [
        PanelPermission::ViewPanel,
        PanelPermission::ManageTickets,
        PanelPermission::ViewBans,
        PanelPermission::ManageCategories,
        PanelPermission::ManageConfig,
        PanelPermission::ManageBot,
        PanelPermission::ManageApiKeys,
        PanelPermission::ManagePermissions,
    ];

    pub fn key(&self) -> &'static str {
        match self {
            PanelPermission::ViewPanel => "view_panel",
            PanelPermission::ManageBot => "manage_bot",
            PanelPermission::ManageConfig => "manage_config",
            PanelPermission::ManageTickets => "manage_tickets",
            PanelPermission::ManageApiKeys => "manage_api_keys",
            PanelPermission::ManagePermissions => "manage_permissions",
            PanelPermission::ManageCategories => "manage_categories",
            PanelPermission::ViewBans => "view_bans",
        }
    }

    pub fn label_key(&self) -> String {
        format!("panel.administration.permissions.{}", self.key())
    }

    pub fn description_key(&self) -> String {
        format!("panel.administration.permission_help.{}", self.key())
    }
}
