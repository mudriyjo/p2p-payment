use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    common::middleware::require_roles,
    domains::backoffice::{
        dto::user_dto::{UserResponse, RoleInfo},
        role::model::{
            admin_role_id, finance_role_id, risk_role_id, support_role_id, user_role_id,
        },
    },
};

use utoipa::OpenApi;

use super::handler;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::handler::get_user,
    ),
    components(schemas(UserResponse, RoleInfo)),
    tags(
        (name = "Users", description = "User management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct UserApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};

            let mut security_scheme = SecurityScheme::ApiKey(
                ApiKey::Header(ApiKeyValue::new("X-JWT-Token"))
            );

            // Add description to the security scheme
            if let SecurityScheme::ApiKey(api_key) = &mut security_scheme {
                *api_key = ApiKey::Header(
                    ApiKeyValue::with_description("X-JWT-Token", "JWT token for authentication. Click 'Authorize' button above to add your token.")
                );
            }

            components.add_security_scheme("jwt_token", security_scheme);
        }
    }
}

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
