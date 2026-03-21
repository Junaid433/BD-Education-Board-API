use crate::exceptions::{AppError, AppResult};
use crate::models::{RequestData, ResultData};
use crate::services::client::HttpClient;
use crate::services::parser::parse_result;
use crate::utils::captcha::solve_captcha;
use serde::Serialize;

#[derive(Serialize)]
struct ResultForm<'a> {
    sr: &'static str,
    et: &'static str,
    exam: &'a str,
    year: &'a str,
    board: &'a str,
    roll: &'a str,
    reg: &'a str,
    value_s: &'a str,
    button2: &'static str,
}

pub async fn fetch_result(client: &HttpClient, request: &RequestData) -> AppResult<ResultData> {
    let (session, home) = client
        .get_homepage()
        .await
        .map_err(|e| AppError::Network(e.to_string()))?;
    let captcha = solve_captcha(&home)?;

    let form = ResultForm {
        sr: "3",
        et: "2",
        exam: request.exam.as_str(),
        year: request.year.as_str(),
        board: request.board.as_str(),
        roll: request.roll.as_str(),
        reg: request.reg.as_str(),
        value_s: captcha.as_str(),
        button2: "Submit",
    };

    let body = client
        .submit_result_form(&session, &form)
        .await
        .map_err(|e| AppError::Network(e.to_string()))?;
    let mut result = parse_result(&body)?;
    if result.reg.is_none() {
        result.reg = Some(request.reg.clone());
    }
    Ok(result)
}
