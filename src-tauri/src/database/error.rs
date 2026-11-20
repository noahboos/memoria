#[derive(Debug)]
pub enum DatabaseError {
    DatabaseLocked,
    SQLError(rusqlite::Error),
}

impl From<rusqlite::Error> for DatabaseError {
    fn from(error: rusqlite::Error) -> Self {
        return Self::SQLError(error);
    }
}