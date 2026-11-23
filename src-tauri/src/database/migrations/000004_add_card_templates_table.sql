CREATE TABLE IF NOT EXISTS card_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    note_template_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    markup TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    modified_at INTEGER DEFAULT NULL,
    
    FOREIGN KEY (note_template_id) REFERENCES note_templates(id) ON DELETE CASCADE
);