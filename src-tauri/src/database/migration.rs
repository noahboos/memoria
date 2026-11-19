// Possibilité d'ajouter un hook en tant que membre de la structure.
// Cela faciliterait l'interaction entre les migrations.
// Typiquement utile pour du formatage de données à réaliser en conséquence d'une migration.
pub(super) struct Migration {
    pub(super) request: &'static str,
}