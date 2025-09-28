pub mod board;
pub mod exam;
pub mod year;

pub const BASE_URL: &str = "http://www.educationboardresults.gov.bd";
pub const RESULT_ENDPOINT: &str = "result.php";

pub struct AppConfig {
    pub timeout_seconds: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 10,
        }
    }
}
