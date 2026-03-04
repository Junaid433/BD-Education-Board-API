use eduboardapi::build_app;
use tower::ServiceBuilder;
use vercel_runtime::Error;
use vercel_runtime::axum::VercelLayer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = ServiceBuilder::new()
        .layer(VercelLayer::new())
        .service(build_app());

    vercel_runtime::run(app).await
}
