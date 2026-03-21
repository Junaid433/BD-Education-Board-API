use scraper::{Html, Selector};
use std::sync::OnceLock;

use crate::exceptions::{AppError, AppResult};

fn td_selector() -> &'static Selector {
    static TD_SELECTOR: OnceLock<Selector> = OnceLock::new();
    TD_SELECTOR.get_or_init(|| Selector::parse("td").expect("valid td selector"))
}

pub fn solve_captcha(html: &str) -> AppResult<String> {
    let document = Html::parse_document(html);

    for td in document.select(td_selector()) {
        let text = td.text().map(str::trim).collect::<String>();
        if text.contains('+') {
            let mut parts = text.split_whitespace();
            if let (Some(left), Some("+"), Some(right), None) =
                (parts.next(), parts.next(), parts.next(), parts.next())
            {
                let left: i32 = left
                    .parse()
                    .map_err(|e: std::num::ParseIntError| AppError::Captcha(e.to_string()))?;
                let right: i32 = right
                    .parse()
                    .map_err(|e: std::num::ParseIntError| AppError::Captcha(e.to_string()))?;
                return Ok((left + right).to_string());
            }
        }
    }

    Err(AppError::Captcha("Captcha not found".to_string()))
}
