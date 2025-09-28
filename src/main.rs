mod api;
mod config;
mod exceptions;
mod models;
mod services;
mod utils;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::fetch::{self, ApiState};
use crate::config::AppConfig;
use crate::services::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "eduboardapi=info,tower_http=info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_target(false),
        )
        .init();

    let config = AppConfig::default();
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
    
    let app: Router = fetch::router(state).layer(trace_layer);

    let addr: SocketAddr = "0.0.0.0:3000".parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!(
        "API Server Running...\nHost: {} \nPort: {}",
        addr.ip(),
        addr.port()
    );
    axum::serve(listener, app).await?;

    Ok(())
}