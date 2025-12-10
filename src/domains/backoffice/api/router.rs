use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    common::middleware::require_roles,
    domains::backoffice::role::model::{
        admin_role_id, finance_role_id, risk_role_id, support_role_id, user_role_id,
    },
};

use super::handler;

pub fn protected_user_routes() -> Router {
    // Admin-only routes (create, update, delete users)
    let admin_routes = Router::new()
        .route("/user", post(handler::create_user))
        .route("/user/{id}", patch(handler::update_user))
        .route("/user/{id}", delete(handler::delete_user))
        .route_layer(middleware::from_fn(require_roles(vec![admin_role_id()])));

    // Authenticated user routes (any role can access their own profile)
    let user_routes = Router::new()
        .route("/user", get(handler::list_users))
        .route("/user/{id}", get(handler::get_user))
        .route("/user/me", get(handler::get_current_user))
        .route_layer(middleware::from_fn(require_roles(vec![
            admin_role_id(),
            support_role_id(),
            risk_role_id(),
            finance_role_id(),
            user_role_id(),
        ])));

    Router::new().merge(admin_routes).merge(user_routes)
}
