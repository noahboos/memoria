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

    // pub fn read_all(&self) -> Result<Vec<NoteTemplate>, NoteTemplateRepositoryError> {
    //     //
    // }
    //
    // pub fn read_one(&self, id: i64) -> Result<NoteTemplate, NoteTemplateRepositoryError> {
    //     //
    // }
    //
    // pub fn create(&self, note_template: &NoteTemplate) -> Result<NoteTemplate, NoteTemplateRepositoryError> {
    //     //
    // }
    //
    // pub fn update(&self, note_template: &NoteTemplate) -> Result<NoteTemplate, NoteTemplateRepositoryError> {
    //     //
    // }
    //
    // pub fn delete(&self, id: i64) -> Result<(), NoteTemplateRepositoryError> {
    //     //
    // }
}