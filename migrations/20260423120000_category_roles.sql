-- Link ticket categories to Discord roles to mention when a ticket is opened in a category

CREATE TABLE IF NOT EXISTS ticket_category_roles (
    category_id TEXT NOT NULL,
    role_id     TEXT NOT NULL,
    created_at  INTEGER NOT NULL,
    PRIMARY KEY (category_id, role_id),
    FOREIGN KEY (category_id) REFERENCES ticket_categories(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_ticket_category_roles_category
    ON ticket_category_roles(category_id);
