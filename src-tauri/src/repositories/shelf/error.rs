use crate::database::DatabaseError;
use crate::models::shelf::ShelfError;

#[derive(Debug)]
pub enum ShelfRepositoryError {
    DatabaseError(DatabaseError),
    ModelError(ShelfError)
}

impl From<DatabaseError> for ShelfRepositoryError {
    fn from(error: DatabaseError) -> Self {
        return Self::DatabaseError(error);
    }
}

impl From<ShelfError> for ShelfRepositoryError {
    fn from(error: ShelfError) -> Self {
        return Self::ModelError(error);
    }
}