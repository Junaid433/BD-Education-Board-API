use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RequestData {
    pub exam: String,
    pub year: String,
    pub board: String,
    pub roll: String,
    pub reg: String,
}
