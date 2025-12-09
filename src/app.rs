use axum::{
    Router,
    routing::get,
    http::{StatusCode, Method},
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
};
use crate::common::{AppState, dto::ApiResponse};

pub fn create_app(state: Arc<AppState>) -> Router {
    // TODO: use configuration domain for CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health_check))
        .nest("/api", api_routes(Arc::clone(&state)))   
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404)
}

fn api_routes(state: Arc<AppState>) -> Router {
    use axum::Extension;

    Router::new()
        .merge(crate::domains::user::public_user_routes())
        .merge(crate::domains::user::protected_user_routes())
        .layer(Extension(state))
}

async fn health_check() -> impl IntoResponse {
    Json(ApiResponse::success(HealthResponse {
        status: "Ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

#[derive(serde::Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

async fn handler_404() -> Response {
    let response = ApiResponse::<()>::with_status(
        404,
        "Not found",
        None,
    );

    (StatusCode::NOT_FOUND, Json(response)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_response_structure() {
        let response = HealthResponse {
            status: "healthy".to_string(),
            version: "1.0.0".to_string(),
        };

        assert_eq!(response.status, "Ok");
        assert_eq!(response.version, "1.0.0");
    }
}