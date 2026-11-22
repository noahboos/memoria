#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Algorithm {
    SM2,
    FSRS
}

impl Algorithm {
    pub fn get_name(&self) -> String {
        return match self {
            Algorithm::SM2 => "SM2".to_string(),
            Algorithm::FSRS => "FSRS".to_string()
        }
    }

    pub fn from_name(name: &str) -> Option<Algorithm> {
        return match name {
            "SM2" => Some(Algorithm::SM2),
            "FSRS" => Some(Algorithm::FSRS),
            _ => None
        }
    }
}