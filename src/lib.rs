pub mod api;
pub mod config;
pub mod exceptions;
pub mod models;
pub mod services;
pub mod utils;

use std::sync::Arc;

use axum::Router;
use tower_http::LatencyUnit;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::api::fetch::{self, ApiState};
use crate::config::AppConfig;
use crate::services::HttpClient;

pub fn build_app() -> Router {
    build_app_with_config(AppConfig::default())
}

pub fn build_app_with_config(config: AppConfig) -> Router {
    let client = Arc::new(HttpClient::new(config));
    let state = ApiState { client };

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Millis),
        );

    fetch::router(state).layer(trace_layer)
}
