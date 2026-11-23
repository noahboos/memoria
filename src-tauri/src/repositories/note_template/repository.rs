use std::sync::Arc;
use crate::database::Database;
use crate::models::note_template::NoteTemplate;
use crate::repositories::note_template::error::NoteTemplateRepositoryError;

#[derive(Debug)]
pub struct NoteTemplateRepository {
    database: Arc<Database>,
}

impl NoteTemplateRepository {
    pub fn new(database: Arc<Database>) -> Self {
        return Self {
            database
        };
    }

    pub fn read_all(&self) -> Result<Vec<NoteTemplate>, NoteTemplateRepositoryError> {
        let connection: MutexGuard<Connection> = self
            .database
            .connection
            .lock()
            .map_err(|error| DatabaseError::DatabaseLocked)?;

        let mut statement: Statement = connection
            .prepare("SELECT * FROM note_templates")
            .map_err(|error| DatabaseError::SQLError(error))?;

        let note_templates: Vec<NoteTemplate> = statement
            .query_map([], |row| Ok(NoteTemplate {
                    id: row.get("id")?,
                    name: row.get("name")?,
                    description: row.get("description")?,
                    data: row.get("data")?,
                    created_at: DateTime::from_timestamp(row.get("created_at")?, 0),
                    modified_at: row
                        .get::<_, Option<i64>>("modified_at")?
                        .and_then(|timestamp| DateTime::from_timestamp(timestamp, 0))
                })
            )
            .map_err(|error| DatabaseError::SQLError(error))?
            .collect::<Result<Vec<NoteTemplate>, _>>()
            .map_err(|error| DatabaseError::SQLError(error))?;

        return Ok(note_templates);
    }

    pub fn read_one(&self, id: i64) -> Result<NoteTemplate, NoteTemplateRepositoryError> {
        let connection: MutexGuard<Connection> = self
            .database
            .connection
            .lock()
            .map_err(|error| DatabaseError::DatabaseLocked)?;

        let mut statement: Statement = connection
            .prepare("SELECT * FROM note_templates WHERE id = ?")
            .map_err(|error| DatabaseError::SQLError(error))?;

        let note_template: NoteTemplate = statement
            .query_row(
                [id],
                |row| Ok(NoteTemplate {
                    id: row.get("id")?,
                    name: row.get("name")?,
                    description: row.get("description")?,
                    data: row.get("data")?,
                    created_at: DateTime::from_timestamp(row.get("created_at")?, 0),
                    modified_at: row
                        .get::<_, Option<i64>>("modified_at")?
                        .and_then(|timestamp| DateTime::from_timestamp(timestamp, 0))
                })
            )
            .map_err(|error| DatabaseError::SQLError(error))?;

        return Ok(note_template);
    }

    pub fn create(&self, note_template: &NoteTemplate) -> Result<NoteTemplate, NoteTemplateRepositoryError> {
        let id: i64 = {
            let connection: MutexGuard<Connection> = self
                .database
                .connection
                .lock()
                .map_err(|_| DatabaseError::DatabaseLocked)?;

            let mut statement: Statement = connection
                .prepare("INSERT INTO note_templates (name, description, data) VALUES (?1, ?2, ?3);")
                .map_err(|error| DatabaseError::SQLError(error))?;

            statement
                .execute(params![&note_template.name, &note_template.description, &note_template.data])
                .map_err(|error| DatabaseError::SQLError(error))?;

            connection.last_insert_rowid()
        };

        let updated_note_template: NoteTemplate = self.read_one(id)?;

        return Ok(updated_note_template);
    }

    pub fn update(&self, note_template: &NoteTemplate) -> Result<NoteTemplate, NoteTemplateRepositoryError> {
        if note_template.id.is_none() {
            return Err(NoteTemplateRepositoryError::ModelError(NoteTemplateError::IdNotFound));
        }

        let mut query_parts: Vec<&str> = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(name) = &note_template.name {
            query_parts.push("name = ?");
            values.push(Box::new(name));
        }

        if let Some(description) = &note_template.description {
            query_parts.push("description = ?");
            values.push(Box::new(description));
        }

        if let Some(data) = &note_template.data {
            query_parts.push("data = ?");
            values.push(Box::new(data));
        }

        query_parts.push("modified_at = ?");
        values.push(Box::new(DateTime::timestamp(&Utc::now())));

        values.push(Box::new(note_template.id));

        {
            let connection: MutexGuard<Connection> = self
                .database
                .connection
                .lock()
                .map_err(|_| DatabaseError::DatabaseLocked)?;

            let mut statement: Statement = connection
                .prepare(format!(
                    "UPDATE note_templates SET {} WHERE id = ?",
                    query_parts.join(", ")
                ).as_str())
                .map_err(|error| DatabaseError::SQLError(error))?;

            statement
                .execute(params_from_iter(values.iter().map(|value| value.as_ref())))
                .map_err(|error| DatabaseError::SQLError(error))?;

        };

        return self.read_one(note_template.id.unwrap());
    }

    // pub fn delete(&self, id: i64) -> Result<(), NoteTemplateRepositoryError> {
    //     //
    // }
}