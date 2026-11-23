use std::sync::{Arc, MutexGuard};
use rusqlite::{params, Connection, Statement};
use crate::database::{Database, DatabaseError};
use crate::models::card_template::CardTemplate;
use crate::repositories::card_template::error::CardTemplateRepositoryError;

#[derive(Debug)]
pub struct CardTemplateRepository {
    database: Arc<Database>,
}

impl CardTemplateRepository {
    pub fn new(database: Arc<Database>) -> Self {
        return Self {
            database
        };
    }

    // pub fn read_all(&self) -> Result<Vec<CardTemplate>, CardTemplateRepositoryError> {
    //     //
    // }
    //
    // pub fn read_one(&self, id: i64) -> Result<CardTemplate, CardTemplateRepositoryError> {
    //     //
    // }
    //
    // pub fn create(&self, card_template: &CardTemplate) -> Result<CardTemplate, CardTemplateRepositoryError> {
    //     //
    // }
    //
    // pub fn update(&self, card_template: &CardTemplate) -> Result<CardTemplate, CardTemplateRepositoryError> {
    //     //
    // }
    //
    pub fn delete(&self, id: i64) -> Result<(), CardTemplateRepositoryError> {
        let connection: MutexGuard<Connection> = self
            .database
            .connection
            .lock()
            .map_err(|_| DatabaseError::DatabaseLocked)?;

        let mut statement: Statement = connection
            .prepare("DELETE FROM card_templates WHERE id = ?")
            .map_err(|error| DatabaseError::SQLError(error))?;

        statement
            .execute(params![id])
            .map_err(|error| DatabaseError::SQLError(error))?;

        return Ok(());
    }
}