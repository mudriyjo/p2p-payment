use std::sync::Arc;
use axum::Server;
use color_eyre::eyre::Result;
use std::net::SocketAddr;
use clean_axum_demo::{app::create_app, common::bootstrap};

#[tokio::main]
async fn main() -> Result<()> {
    let state = bootstrap::initialize_app().await?;

    let app = create_app(Arc::clone(&state));

    let addr: SocketAddr = state
        .config
        .server_address()
        .parse()
        .expect("Invalid server address");

    tracing::info!("🚀 Server starting on http://{}", addr);
    tracing::info!("📚 API docs available at http://{}/docs", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}