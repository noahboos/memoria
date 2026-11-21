use std::sync::{Arc, MutexGuard};
use chrono::{DateTime, Utc};
use rusqlite::{params, params_from_iter, Connection, Statement};
use crate::database::{Database, DatabaseError};
use crate::models::shelf::{Shelf, ShelfError};
use crate::repositories::shelf::ShelfRepositoryError;

#[derive(Debug)]
pub struct ShelfRepository {
    database: Arc<Database>,
}

impl ShelfRepository {
    pub fn new(database: Arc<Database>) -> Self {
        return Self {
            database
        };
    }

    pub fn read_all(&self) -> Result<Vec<Shelf>, ShelfRepositoryError> {
        let connection: MutexGuard<Connection> = self
            .database
            .connection
            .lock()
            .map_err(|error| DatabaseError::DatabaseLocked)?;

        let mut statement: Statement = connection
            .prepare("SELECT * FROM shelves")
            .map_err(|error| DatabaseError::SQLError(error))?;

        let shelves: Vec<Shelf> = statement
            .query_map([], |row| Ok(Shelf {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: DateTime::from_timestamp(row.get(2)?, 0),
                    modified_at: row
                        .get::<_, Option<i64>>(3)?
                        .and_then(|timestamp| DateTime::from_timestamp(timestamp, 0))
                })
            )
            .map_err(|error| DatabaseError::SQLError(error))?
            .collect::<Result<Vec<Shelf>, _>>()
            .map_err(|error| DatabaseError::SQLError(error))?;

        drop(statement);
        drop(connection);

        return Ok(shelves);
    }

    pub fn read_one(&self, id: i64) -> Result<Shelf, ShelfRepositoryError> {
        let connection: MutexGuard<Connection> = self
            .database
            .connection
            .lock()
            .map_err(|error| DatabaseError::DatabaseLocked)?;

        let mut statement: Statement = connection
            .prepare("SELECT * FROM shelves WHERE id = ?1")
            .map_err(|error| DatabaseError::SQLError(error))?;

        let shelf: Shelf = statement
            .query_row(
                [id],
                |row| Ok(Shelf {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: DateTime::from_timestamp(row.get(2)?, 0),
                    modified_at: row
                        .get::<_, Option<i64>>(3)?
                        .and_then(|timestamp| DateTime::from_timestamp(timestamp, 0))
                })
            )
            .map_err(|error| DatabaseError::SQLError(error))?;

        drop(statement);
        drop(connection);

        return Ok(shelf);
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

        drop(statement);
        drop(connection);

        return Ok(shelf);
    }

    pub fn update(&self, shelf: &Shelf) -> Result<Shelf, ShelfRepositoryError> {
        if shelf.id.is_none() {
            return Err(ShelfRepositoryError::ModelError(ShelfError::IdNotFound));
        }

        let mut query_parts: Vec<&str> = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(name) = &shelf.name {
            query_parts.push("name = ?");
            values.push(Box::new(name));
        }

        query_parts.push("modified_at = ?");
        values.push(Box::new(DateTime::timestamp(&Utc::now())));

        values.push(Box::new(shelf.id));

        let connection: MutexGuard<Connection> = self
            .database
            .connection
            .lock()
            .map_err(|_| DatabaseError::DatabaseLocked)?;

        let mut statement: Statement = connection
            .prepare(format!(
                "UPDATE shelves SET {} WHERE id = ?",
                query_parts.join(", ")
            ).as_str())
            .map_err(|error| DatabaseError::SQLError(error))?;

        statement
            .execute(params_from_iter(values.iter().map(|value| value.as_ref())))
            .map_err(|error| DatabaseError::SQLError(error))?;

        drop(statement);
        drop(connection);

        return self.read_one(shelf.id.unwrap());
    }
}