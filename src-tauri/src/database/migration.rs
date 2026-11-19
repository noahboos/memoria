// Possibilité d'ajouter un hook en tant que membre de la structure.
// Cela faciliterait l'interaction entre les migrations.
// Typiquement utile pour du formatage de données à réaliser en conséquence d'une migration.
pub(super) struct Migration {
    pub(super) request: &'static str,
}

pub(super) const ADD_SHELVES_TABLE_MIGRATION: Migration = Migration {
    request: include_str!("./migrations/000001_add_shelves_table.sql"),
};