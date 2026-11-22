use crate::database::DatabaseError;
use crate::models::stack::StackError;

#[derive(Debug)]
pub enum StackRepositoryError {
    DatabaseError(DatabaseError),
    ModelError(StackError),
}

impl From<DatabaseError> for StackRepositoryError {
    fn from(error: DatabaseError) -> Self {
        return Self::DatabaseError(error);
    }
}

impl From<StackError> for StackRepositoryError {
    fn from(error: StackError) -> Self {
        return Self::ModelError(error);
    }
}