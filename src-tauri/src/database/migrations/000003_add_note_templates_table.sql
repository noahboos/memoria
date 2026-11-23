CREATE TABLE IF NOT EXISTS note_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    description TEXT DEFAULT NULL,
    data TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    modified_at INTEGER DEFAULT NULL
);