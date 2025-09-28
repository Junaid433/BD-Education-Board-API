use scraper::{Html, Selector};

use crate::exceptions::{AppError, AppResult};

pub fn solve_captcha(html: &str) -> AppResult<String> {
    let document = Html::parse_document(html);
    let td_selector = Selector::parse("td").unwrap();

    for td in document.select(&td_selector) {
        let text = td.text().collect::<String>().trim().to_string();
        if text.contains('+') {
            let parts: Vec<&str> = text.split_whitespace().collect();
            if parts.len() == 3 {
                let left: i32 = parts[0]
                    .parse()
                    .map_err(|e: std::num::ParseIntError| AppError::Captcha(e.to_string()))?;
                let right: i32 = parts[2]
                    .parse()
                    .map_err(|e: std::num::ParseIntError| AppError::Captcha(e.to_string()))?;
                return Ok((left + right).to_string());
            }
        }
    }

    Err(AppError::Captcha("Captcha not found".to_string()))
}
