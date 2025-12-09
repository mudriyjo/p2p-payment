use axum::{
    routing::{get, patch, delete, post},
    Router,
};

use super::handler;

pub fn public_user_routes() -> Router {
    Router::new()
        .route("/user", get(handler::list_users))
        .route("/user/:id", get(handler::get_user))
}

pub fn protected_user_routes() -> Router {
    Router::new()
        .route("/user/me", get(handler::get_current_user))
        // .route("/user/:id", patch(handler::update_user))
        // .route("/user/:id", delete(handler::delete_user))
}