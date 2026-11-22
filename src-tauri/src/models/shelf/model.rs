use std::option::Option;
use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Shelf {
    pub(crate) id: Option<i64>,
    pub(crate) name: Option<String>,
    pub(crate) created_at: Option<DateTime<Utc>>,
    pub(crate) modified_at: Option<DateTime<Utc>>,
}

impl Shelf {
    pub fn new(name: &str) -> Self {
        return Self {
            id: None,
            name: Option::from(name.to_string()),
            created_at: Option::from(Utc::now()),
            modified_at: None
        };
    }
}