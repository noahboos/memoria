use std::sync::{Arc, MutexGuard};
use rusqlite::{params, Connection, Statement};
use crate::database::{Database, DatabaseError};
use crate::models::shelf::Shelf;
use crate::repositories::shelf::ShelfRepositoryError;

pub struct ShelfRepository {
    database: Arc<Database>,
}

impl ShelfRepository {
    pub fn new(database: Arc<Database>) -> Self {
        return Self {
            database
        };
    }

    pub fn create(&self, name: &str) -> Result<Shelf, ShelfRepositoryError> {
        let shelf: Shelf = Shelf::new(name);

        let connection: MutexGuard<Connection> = self
            .database
            .connection
            .lock()
            .map_err(|_| DatabaseError::DatabaseLocked)?;

        let mut statement: Statement = connection
            .prepare("INSERT INTO shelves (name) VALUES (?1);")
            .map_err(|error| DatabaseError::SQLError(error))?;

        statement
            .execute(params![&shelf.name])
            .map_err(|error| DatabaseError::SQLError(error))?;

        return Ok(shelf);
    }
}