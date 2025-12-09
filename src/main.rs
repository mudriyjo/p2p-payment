use std::sync::Arc;
use color_eyre::eyre::Result;
use p2p_payment::{app::create_app, common::bootstrap};

#[tokio::main]
async fn main() -> Result<()> {
    let state = bootstrap::initialize_app().await?;

    let app = create_app(Arc::clone(&state));

    let addr = state.config.server_address();

    tracing::info!("🚀 Server starting on http://{}", addr);
    tracing::info!("📚 API docs available at http://{}/docs", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}