use crate::database::DatabaseError;
use crate::models::note_template::NoteTemplateError;

#[derive(Debug)]
pub enum NoteTemplateRepositoryError {
    DatabaseError(DatabaseError),
    ModelError(NoteTemplateError)
}

impl From<DatabaseError> for NoteTemplateRepositoryError {
    fn from(error: DatabaseError) -> Self {
        return Self::DatabaseError(error);
    }
}

impl From<NoteTemplateError> for NoteTemplateRepositoryError {
    fn from(error: NoteTemplateError) -> Self {
        return Self::ModelError(error);
    }
}