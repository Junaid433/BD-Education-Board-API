use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use tracing::instrument;

use crate::exceptions::AppError;
use crate::models::RequestData;
use crate::services::{HttpClient, fetch_result};

#[derive(Clone)]
pub struct ApiState {
    pub client: Arc<HttpClient>,
}

#[derive(Debug, Deserialize)]
pub struct FetchPayload {
    pub exam: String,
    pub year: String,
    pub board: String,
    pub roll: String,
    pub reg: String,
}

impl From<FetchPayload> for RequestData {
    fn from(value: FetchPayload) -> Self {
        Self {
            exam: value.exam,
            year: value.year,
            board: value.board,
            roll: value.roll,
            reg: value.reg,
        }
    }
}

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/fetch", get(fetch_get).post(fetch_post))
        .with_state(state)
}

#[instrument(skip(state))]
async fn fetch_get(State(state): State<ApiState>, Query(payload): Query<FetchPayload>) -> Response {
    handle_fetch(state, payload).await
}

#[instrument(skip(state))]
async fn fetch_post(State(state): State<ApiState>, Json(payload): Json<FetchPayload>) -> Response {
    handle_fetch(state, payload).await
}

fn map_error(err: AppError) -> Response {
    match err {
        AppError::Network(msg) => (StatusCode::BAD_GATEWAY, msg).into_response(),
        AppError::Parse(msg) => (StatusCode::BAD_GATEWAY, msg).into_response(),
        AppError::Captcha(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
    }
}

#[instrument(skip(state), fields(exam = %payload.exam, year = %payload.year, board = %payload.board, roll = %payload.roll))]
async fn handle_fetch(state: ApiState, payload: FetchPayload) -> Response {
    let request: RequestData = payload.into();
    match fetch_result(state.client.as_ref(), &request).await {
        Ok(result) => Json(result).into_response(),
        Err(err) => map_error(err),
    }
}
