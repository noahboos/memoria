use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NoteTemplate {
    pub(crate) id: Option<i64>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) data: Option<String>,
    pub(crate) created_at: Option<DateTime<Utc>>,
    pub(crate) modified_at: Option<DateTime<Utc>>,
}

impl NoteTemplate {
    //
}