use std::error::Error;
use std::path::{Path, PathBuf};
use rusqlite::{Connection, Result, Transaction};
use std::sync::Mutex;
use tauri::{App, Manager};
use super::version::VERSIONS;

pub struct Database {
    pub(crate) connection: Mutex<Connection>,
}

impl Database {
    pub fn create(app: &App) -> Result<Self, Box<dyn Error>> {
        let path: PathBuf =  app.path().app_data_dir()?.join("database.db");

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        return Ok(Self::open(&path)?);
    }

    pub fn open(path: &Path) -> Result<Self> {
        let mut connection: Connection = Connection::open(path)?;

        connection.pragma_update(None, "foreign_keys", "ON")?;

        Self::migrate(&mut connection)?;

        return Ok(Self {
            connection: Mutex::new(connection),
        });
    }

    fn migrate(connection: &mut Connection) -> Result<()> {
        let _version: i64 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;

        if VERSIONS.is_empty() {
            return Ok(());
        }

        if (_version > VERSIONS.last().unwrap().version) {
            return Err(rusqlite::Error::InvalidQuery);
        }

        let transaction: Transaction = connection.transaction()?;

        for version in VERSIONS {
            if version.version > _version {
                for migration in version.migrations {
                    transaction.execute_batch(migration.statements)?;
                }

                transaction.pragma_update(None, "user_version", version.version)?;
            }
        }

        transaction.commit()?;

        return Ok(());
    }
}