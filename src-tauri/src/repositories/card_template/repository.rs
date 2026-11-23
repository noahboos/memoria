use std::sync::Arc;
use crate::database::Database;
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

    // pub fn read_all(&self) -> Result<Vec<CardTemplate>, CardTemplateRepositoryError> {
    //     //
    // }
    // 
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
    // pub fn delete(&self, id: i64) -> Result<(), CardTemplateRepositoryError> {
    //     //
    // }
}