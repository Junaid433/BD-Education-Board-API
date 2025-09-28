use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum AppError {
    Network(String),
    Parse(String),
    Captcha(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            AppError::Network(msg) => write!(f, "Network error: {}", msg),
            AppError::Parse(msg) => write!(f, "Parse error: {}", msg),
            AppError::Captcha(msg) => write!(f, "Captcha error: {}", msg),
        }
    }
}

impl Error for AppError {}

pub type AppResult<T> = Result<T, AppError>;
