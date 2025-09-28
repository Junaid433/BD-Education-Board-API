use std::collections::HashMap;

use crate::exceptions::{AppError, AppResult};
use crate::models::{RequestData, ResultData};
use crate::services::client::HttpClient;
use crate::services::parser::parse_result;
use crate::utils::captcha::solve_captcha;

pub async fn fetch_result(client: &HttpClient, request: &RequestData) -> AppResult<ResultData> {
    let home = client
        .get_homepage()
        .await
        .map_err(|e| AppError::Network(e.to_string()))?;
    let captcha = solve_captcha(&home)?;

    let mut form: HashMap<&str, &str> = HashMap::new();
    form.insert("sr", "3");
    form.insert("et", "2");
    form.insert("exam", request.exam.as_str());
    form.insert("year", request.year.as_str());
    form.insert("board", request.board.as_str());
    form.insert("roll", request.roll.as_str());
    form.insert("reg", request.reg.as_str());
    form.insert("value_s", captcha.as_str());
    form.insert("button2", "Submit");

    let body = client
        .submit_result_form(&form)
        .await
        .map_err(|e| AppError::Network(e.to_string()))?;
    Ok(parse_result(&body))
}
