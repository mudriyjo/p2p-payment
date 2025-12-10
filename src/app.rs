use crate::common::{dto::ApiResponse, AppState};
use crate::domains::backoffice::UserApiDoc;
use axum::{
    http::{HeaderName, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use std::{str::FromStr, sync::Arc, time::Duration};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn create_app(state: Arc<AppState>) -> Router {
    let cors = configure_cors(&state);

    Router::new()
        .route("/health", get(health_check))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", UserApiDoc::openapi()))
        .nest("/api/v1", api_routes(Arc::clone(&state)))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404)
}

fn parse_cors_helper<T: FromStr>(parameters: Vec<String>) -> Vec<T> {
    parameters
        .into_iter()
        .filter_map(|param| param.parse::<T>().ok())
        .collect()
}

fn configure_cors(state: &AppState) -> CorsLayer {
    let config = &state.config.cors;

    // Parse origins
    let origins: Vec<_> = parse_cors_helper(config.parse_origins());

    // Parse methods
    let methods: Vec<Method> = parse_cors_helper(config.parse_methods());

    // Parse headers
    let headers: Vec<HeaderName> = parse_cors_helper(config.parse_headers());

    let mut cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods(methods)
        .allow_headers(headers)
        .max_age(Duration::from_secs(config.max_age));

    if config.allow_credentials {
        cors = cors.allow_credentials(true);
    }

    cors
}

fn api_routes(state: Arc<AppState>) -> Router {
    use axum::Extension;

    Router::new()
        .merge(crate::domains::backoffice::protected_user_routes())
        .layer(Extension(state))
}

async fn health_check() -> impl IntoResponse {
      Json(serde_json::json!({
          "status": "Ok",
          "version": env!("CARGO_PKG_VERSION")
      }))
}

async fn handler_404() -> Response {
    let response = ApiResponse::<()>::with_status(404, "Not found", None);

    (StatusCode::NOT_FOUND, Json(response)).into_response()
}