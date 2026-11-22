use chrono::{DateTime, Utc};
use crate::algorithms::Algorithm;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Stack {
    pub(crate) id: Option<i64>,
    pub(crate) shelf_id: Option<i64>,
    pub(crate) name: Option<String>,
    pub(crate) algorithm: Option<Algorithm>,
    pub(crate) created_at: Option<DateTime<Utc>>,
    pub(crate) modified_at: Option<DateTime<Utc>>,
}

impl Stack {
    //
}