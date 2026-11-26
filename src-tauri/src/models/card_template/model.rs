use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CardTemplate {
    pub(crate) id: Option<i64>,
    pub(crate) note_template_id: Option<i64>,
    pub(crate) name: Option<String>,
    pub(crate) front_markup: Option<String>,
    pub(crate) back_markup: Option<String>,
    pub(crate) created_at: Option<DateTime<Utc>>,
    pub(crate) modified_at: Option<DateTime<Utc>>,
}

impl CardTemplate {
    //
}