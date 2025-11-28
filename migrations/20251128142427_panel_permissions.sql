CREATE TABLE IF NOT EXISTS panel_permissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    subject_type TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    permission TEXT NOT NULL,
    granted_by TEXT NOT NULL,
    granted_at INTEGER NOT NULL,
    UNIQUE(subject_type, subject_id, permission)
);

CREATE INDEX idx_panel_perms_subject ON panel_permissions(subject_type, subject_id);
CREATE INDEX idx_panel_perms_permission ON panel_permissions(permission);
