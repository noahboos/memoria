use crate::database::migration::{Migration, ADD_SHELVES_TABLE_MIGRATION};

pub(super) struct Version {
    pub(super) version: i64,
    pub(super) migrations: &'static [&'static Migration],
}

pub(super) const VERSIONS: &[Version] = &[
    Version {
        version: 1,
        migrations: &[
            &ADD_SHELVES_TABLE_MIGRATION,
        ],
    },
];