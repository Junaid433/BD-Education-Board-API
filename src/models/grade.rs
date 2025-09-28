use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Grade {
    pub code: String,
    pub subject: String,
    pub grade: String,
}
