use crate::database::DatabaseError;
use crate::models::card_template::CardTemplateError;

#[derive(Debug)]
pub enum CardTemplateRepositoryError {
    DatabaseError(DatabaseError),
    ModelError(CardTemplateError)
}

impl From<DatabaseError> for CardTemplateRepositoryError {
    fn from(error: DatabaseError) -> Self {
        return Self::DatabaseError(error);
    }
}

impl From<CardTemplateError> for CardTemplateRepositoryError {
    fn from(error: CardTemplateError) -> Self {
        return Self::ModelError(error);
    }
}