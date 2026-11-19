use crate::database::migration::{Migration};

pub(super) struct Version {
    pub(super) version: i64,
    pub(super) migrations: &'static [&'static Migration],
}

pub(super) const VERSIONS: &[Version] = &[
    // Version {
    //     version: 1,
    //     migrations: &[
    //         // First migration here.
    //     ],
    // },
];