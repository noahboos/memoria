use std::sync::Arc;
use crate::database::Database;

pub struct ShelfRepository {
    database: Arc<Database>,
}

impl ShelfRepository {
    pub fn new(database: Arc<Database>) -> Self {
        return Self {
            database
        };
    }
}