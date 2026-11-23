use std::sync::{Arc, MutexGuard};
use chrono::DateTime;
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

    pub fn read_all(&self) -> Result<Vec<CardTemplate>, CardTemplateRepositoryError> {
        let connection: MutexGuard<Connection> = self
            .database
            .connection
            .lock()
            .map_err(|error| DatabaseError::DatabaseLocked)?;

        let mut statement: Statement = connection
            .prepare("SELECT * FROM card_templates")
            .map_err(|error| DatabaseError::SQLError(error))?;

        let card_templates: Vec<CardTemplate> = statement
            .query_map([], |row| Ok(CardTemplate {
                    id: row.get("id")?,
                    note_template_id: row.get("note_template_id")?,
                    name: row.get("name")?,
                    markup: row.get("markup")?,
                    created_at: DateTime::from_timestamp(row.get("created_at")?, 0),
                    modified_at: row
                        .get::<_, Option<i64>>("modified_at")?
                        .and_then(|timestamp| DateTime::from_timestamp(timestamp, 0))
                })
            )
            .map_err(|error| DatabaseError::SQLError(error))?
            .collect::<Result<Vec<CardTemplate>, _>>()
            .map_err(|error| DatabaseError::SQLError(error))?;

        return Ok(card_templates);
    }

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