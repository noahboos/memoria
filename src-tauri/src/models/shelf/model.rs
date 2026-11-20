use std::option::Option;
use chrono::{DateTime, Utc};

pub struct Shelf {
    pub(crate) id: Option<i64>,
    pub(crate) name: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) modified_at: Option<DateTime<Utc>>,
}

impl Shelf {
    pub fn new(name: &str) -> Self {
        return Self {
            id: None,
            name: name.to_string(),
            created_at: Utc::now(),
            modified_at: None
        };
    }
}