// Possibilité d'ajouter un hook en tant que membre de la structure.
// Cela faciliterait l'interaction entre les migrations.
// Typiquement utile pour du formatage de données à réaliser en conséquence d'une migration.
#[derive(Debug)]
pub(super) struct Migration {
    pub(super) statements: &'static str,
}

pub(super) const ADD_SHELVES_TABLE_MIGRATION: Migration = Migration {
    statements: include_str!("./migrations/000001_add_shelves_table.sql"),
};

pub(super) const ADD_STACKS_TABLE_MIGRATION: Migration = Migration {
    statements: include_str!("./migrations/000002_add_stacks_table.sql"),
};

pub(super) const ADD_NOTE_TEMPLATES_TABLE_MIGRATION: Migration = Migration {
    statements: include_str!("./migrations/000003_add_note_templates_table.sql"),
};

pub(super) const ADD_CARD_TEMPLATES_TABLE_MIGRATION: Migration = Migration {
    statements: include_str!("./migrations/000004_add_card_templates_table.sql"),
};

pub(super) const SPLIT_MARKUP_ATTRIBUTE_INTO_FRONT_MARKUP_AND_BACK_MARKUP_ATTRIBUTES_IN_CARD_TEMPLATES_TABLE_MIGRATION: Migration = Migration {
    statements: include_str!("./migrations/000005_split_markup_attribute_into_front_markup_and_back_markup_attributes_in_card_templates_table.sql"),
};