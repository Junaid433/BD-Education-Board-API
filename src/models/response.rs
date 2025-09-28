use serde::Serialize;

use crate::models::grade::Grade;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ResultData {
    pub roll: String,
    pub reg: String,
    pub name: String,
    pub father_name: String,
    pub mother_name: String,
    pub board: String,
    pub group: String,
    pub exam_type: String,
    pub dob: String,
    pub institute: String,
    pub result: String,
    pub gpa: String,
    pub grades: Vec<Grade>,
}
