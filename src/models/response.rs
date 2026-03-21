use serde::Serialize;

use crate::models::grade::Grade;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ResultData {
    pub roll: Option<String>,
    pub reg: Option<String>,
    pub name: Option<String>,
    pub father_name: Option<String>,
    pub mother_name: Option<String>,
    pub board: Option<String>,
    pub group: Option<String>,
    pub exam_type: Option<String>,
    pub dob: Option<String>,
    pub institute: Option<String>,
    pub result: Option<String>,
    pub gpa: Option<String>,
    pub grades: Vec<Grade>,
}

impl ResultData {
    pub fn has_identity(&self) -> bool {
        self.roll.is_some() || self.name.is_some()
    }
}
