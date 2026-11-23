use std::sync::{Arc, MutexGuard};
use chrono::{DateTime, Utc};
use rusqlite::{params, params_from_iter, Connection, Statement};
use crate::algorithms::Algorithm;
use crate::database::{Database, DatabaseError};
use crate::models::stack::{Stack, StackError};
use crate::repositories::stack::StackRepositoryError;

#[derive(Debug)]
pub struct StackRepository {
    database: Arc<Database>,
}

impl StackRepository {
    pub fn new(database: Arc<Database>) -> Self {
        return Self {
            database
        };
    }

    pub fn read_all(&self) -> Result<Vec<Stack>, StackRepositoryError> {
        let connection: MutexGuard<Connection> = self
            .database
            .connection
            .lock()
            .map_err(|error| DatabaseError::DatabaseLocked)?;

        let mut statement: Statement = connection
            .prepare("SELECT * FROM stacks")
            .map_err(|error| DatabaseError::SQLError(error))?;

        let stacks: Vec<Stack> = statement
            .query_map([], |row| Ok(Stack {
                id: row.get(0)?,
                shelf_id: row.get(1)?,
                name: row.get(2)?,
                algorithm: Algorithm::from_name(&row.get::<_, String>(3)?),
                created_at: DateTime::from_timestamp(row.get(4)?, 0),
                modified_at: row
                    .get::<_, Option<i64>>(5)?
                    .and_then(|timestamp| DateTime::from_timestamp(timestamp, 0))
                })
            )
            .map_err(|error| DatabaseError::SQLError(error))?
            .collect::<Result<Vec<Stack>, _>>()
            .map_err(|error| DatabaseError::SQLError(error))?;

        return Ok(stacks);
    }

    pub fn read_one(&self, id: i64) -> Result<Stack, StackRepositoryError> {
        let connection: MutexGuard<Connection> = self
            .database
            .connection
            .lock()
            .map_err(|error| DatabaseError::DatabaseLocked)?;

        let mut statement: Statement = connection
            .prepare("SELECT * FROM stacks WHERE id = ?")
            .map_err(|error| DatabaseError::SQLError(error))?;

        let stack: Stack = statement
            .query_row(
                [id],
                |row| Ok(Stack {
                    id: row.get(0)?,
                    shelf_id: row.get(1)?,
                    name: row.get(2)?,
                    algorithm: Algorithm::from_name(&row.get::<_, String>(3)?),
                    created_at: DateTime::from_timestamp(row.get(4)?, 0),
                    modified_at: row
                        .get::<_, Option<i64>>(5)?
                        .and_then(|timestamp| DateTime::from_timestamp(timestamp, 0))
                })
            )
            .map_err(|error| DatabaseError::SQLError(error))?;

        return Ok(stack);
    }

    pub fn create(&self, stack: &Stack) -> Result<Stack, StackRepositoryError> {
        let algorithm: &Algorithm = stack.algorithm.as_ref().unwrap_or(&Algorithm::SM2);

        let id: i64 = {
            let connection: MutexGuard<Connection> = self
                .database
                .connection
                .lock()
                .map_err(|_| DatabaseError::DatabaseLocked)?;

            let mut statement: Statement = connection
                .prepare("INSERT INTO stacks (shelf_id, name, algorithm) VALUES (?1, ?2, ?3);")
                .map_err(|error| DatabaseError::SQLError(error))?;

            statement
                .execute(params![&stack.shelf_id, &stack.name, algorithm.get_name()])
                .map_err(|error| DatabaseError::SQLError(error))?;

            connection.last_insert_rowid()
        };

        let updated_stack: Stack = self.read_one(id)?;

        return Ok(updated_stack);
    }

    pub fn update(&self, stack: &Stack) -> Result<Stack, StackRepositoryError> {
        if stack.id.is_none() {
            return Err(StackRepositoryError::ModelError(StackError::IdNotFound));
        }

        let mut query_parts: Vec<&str> = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(shelf_id) = &stack.shelf_id {
            query_parts.push("shelf_id = ?");
            values.push(Box::new(shelf_id));
        }

        if let Some(name) = &stack.name {
            query_parts.push("name = ?");
            values.push(Box::new(name));
        }

        if let Some(algorithm) = &stack.algorithm {
            query_parts.push("algorithm = ?");
            values.push(Box::new(algorithm.get_name()));
        }

        query_parts.push("modified_at = ?");
        values.push(Box::new(DateTime::timestamp(&Utc::now())));

        values.push(Box::new(stack.id));
        
        {
            let connection: MutexGuard<Connection> = self
                .database
                .connection
                .lock()
                .map_err(|_| DatabaseError::DatabaseLocked)?;

            let mut statement: Statement = connection
                .prepare(format!(
                    "UPDATE stacks SET {} WHERE id = ?",
                    query_parts.join(", ")
                ).as_str())
                .map_err(|error| DatabaseError::SQLError(error))?;

            statement
                .execute(params_from_iter(values.iter().map(|value| value.as_ref())))
                .map_err(|error| DatabaseError::SQLError(error))?;
        };

        return self.read_one(stack.id.unwrap());
    }

    pub fn delete(&self, id: i64) -> Result<(), StackRepositoryError> {
        let connection: MutexGuard<Connection> = self
            .database
            .connection
            .lock()
            .map_err(|_| DatabaseError::DatabaseLocked)?;

        let mut statement: Statement = connection
            .prepare("DELETE FROM stacks WHERE id = ?")
            .map_err(|error| DatabaseError::SQLError(error))?;

        statement
            .execute(params![id])
            .map_err(|error| DatabaseError::SQLError(error))?;

        return Ok(());
    }
}