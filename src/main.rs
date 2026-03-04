use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use eduboardapi::build_app;

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

    let port = env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr: SocketAddr = format!("0.0.0.0:{port}").parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!(
        "API Server Running...\nHost: {} \nPort: {}",
        addr.ip(),
        addr.port()
    );
    axum::serve(listener, build_app()).await?;

    Ok(())
}
