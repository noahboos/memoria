use std::sync::{Arc, MutexGuard};
use chrono::{DateTime, Utc};
use rusqlite::{params, params_from_iter, Connection, Statement};
use crate::database::{Database, DatabaseError};
use crate::models::card_template::{CardTemplate, CardTemplateError};
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
                    front_markup: row.get("front_markup")?,
                    back_markup: row.get("back_markup")?,
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

    pub fn read_one(&self, id: i64) -> Result<CardTemplate, CardTemplateRepositoryError> {
        let connection: MutexGuard<Connection> = self
            .database
            .connection
            .lock()
            .map_err(|error| DatabaseError::DatabaseLocked)?;

        let mut statement: Statement = connection
            .prepare("SELECT * FROM card_templates WHERE id = ?")
            .map_err(|error| DatabaseError::SQLError(error))?;

        let card_template: CardTemplate = statement
            .query_row(
                [id],
                |row| Ok(CardTemplate {
                    id: row.get("id")?,
                    note_template_id: row.get("note_template_id")?,
                    name: row.get("name")?,
                    front_markup: row.get("front_markup")?,
                    back_markup: row.get("back_markup")?,
                    created_at: DateTime::from_timestamp(row.get("created_at")?, 0),
                    modified_at: row
                        .get::<_, Option<i64>>("modified_at")?
                        .and_then(|timestamp| DateTime::from_timestamp(timestamp, 0))
                })
            )
            .map_err(|error| DatabaseError::SQLError(error))?;

        return Ok(card_template);
    }

    pub fn create(&self, card_template: &CardTemplate) -> Result<CardTemplate, CardTemplateRepositoryError> {
        let id: i64 = {
            let connection: MutexGuard<Connection> = self
                .database
                .connection
                .lock()
                .map_err(|_| DatabaseError::DatabaseLocked)?;

            let mut statement: Statement = connection
                .prepare("INSERT INTO card_templates (note_template_id, name, front_markup, back_markup) VALUES (?1, ?2, ?3, ?4);")
                .map_err(|error| DatabaseError::SQLError(error))?;

            statement
                .execute(params![&card_template.note_template_id, &card_template.name, &card_template.front_markup, &card_template.back_markup])
                .map_err(|error| DatabaseError::SQLError(error))?;

            connection.last_insert_rowid()
        };

        let updated_card_template: CardTemplate = self.read_one(id)?;

        return Ok(updated_card_template);
    }

    pub fn update(&self, card_template: &CardTemplate) -> Result<CardTemplate, CardTemplateRepositoryError> {
        if card_template.id.is_none() {
            return Err(CardTemplateRepositoryError::ModelError(CardTemplateError::IdNotFound));
        }

        let mut query_parts: Vec<&str> = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(note_template_id) = &card_template.note_template_id {
            query_parts.push("note_template_id = ?");
            values.push(Box::new(note_template_id));
        }

        if let Some(name) = &card_template.name {
            query_parts.push("name = ?");
            values.push(Box::new(name));
        }

        if let Some(front_markup) = &card_template.front_markup {
            query_parts.push("front_markup = ?");
            values.push(Box::new(front_markup));
        }

        if let Some(back_markup) = &card_template.back_markup {
            query_parts.push("back_markup = ?");
            values.push(Box::new(back_markup));
        }

        query_parts.push("modified_at = ?");
        values.push(Box::new(DateTime::timestamp(&Utc::now())));

        values.push(Box::new(card_template.id));

        {
            let connection: MutexGuard<Connection> = self
                .database
                .connection
                .lock()
                .map_err(|_| DatabaseError::DatabaseLocked)?;

            let mut statement: Statement = connection
                .prepare(format!(
                    "UPDATE card_templates SET {} WHERE id = ?",
                    query_parts.join(", ")
                ).as_str())
                .map_err(|error| DatabaseError::SQLError(error))?;

            statement
                .execute(params_from_iter(values.iter().map(|value| value.as_ref())))
                .map_err(|error| DatabaseError::SQLError(error))?;
        };

        return self.read_one(card_template.id.unwrap());
    }

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