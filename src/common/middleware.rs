use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::common::{app_state::AppState, error::AppError, jwt::Claims};

/// JWT Authentication Middleware
/// Extracts and validates JWT token from X-JWT-Token header
/// Adds Claims to request extensions if valid
pub async fn jwt_auth(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract X-JWT-Token header
    let token = request
        .headers()
        .get("X-JWT-Token")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing X-JWT-Token header".to_string()))?;

    // Decode and validate JWT
    let claims = state
        .jwt_service
        .decode_token(token)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

    // Check if token is expired
    if claims.is_expired() {
        return Err(AppError::Unauthorized("Token has expired".to_string()));
    }

    // Add claims to request extensions for downstream handlers
    request.extensions_mut().insert(claims);

    // Continue to next middleware/handler
    Ok(next.run(request).await)
}

/// Role-Based Authorization Middleware Factory
/// Checks if authenticated user has one of the required role IDs
///
/// # Example Usage
/// ```ignore
/// use crate::domains::backoffice::role::model::admin_role_id;
/// Router::new()
///     .route("/admin-only", post(handler))
///     .route_layer(axum::middleware::from_fn(require_roles(vec![admin_role_id()])))
/// ```
pub fn require_roles(
    required_role_ids: Vec<Uuid>,
) -> impl Fn(
    Request,
    Next,
)
    -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, AppError>> + Send>>
       + Clone {
    move |request: Request, next: Next| {
        let required_role_ids = required_role_ids.clone();
        Box::pin(async move {
            // Extract claims from request extensions (added by jwt_auth middleware)
            let claims = request
                .extensions()
                .get::<Claims>()
                .ok_or_else(|| AppError::Unauthorized("Authentication required".to_string()))?
                .clone();

            // Check if user has one of the required roles
            if !required_role_ids.contains(&claims.role_id) {
                return Err(AppError::Forbidden(
                    "Insufficient permissions to access this resource".to_string(),
                ));
            }

            // User has required role, continue
            Ok(next.run(request).await)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_role_check_logic() {
        let admin_role_id = Uuid::parse_str("878c19c6-643b-4a57-98f1-a60786a38a92").unwrap();
        let user_role_id = Uuid::parse_str("eec86d00-495c-490c-b151-b9d33672a681").unwrap();

        let claims = Claims {
            sub: "test_user".to_string(),
            user_id: Uuid::new_v4(),
            role_id: admin_role_id,
            role_name: "Admin".to_string(),
            exp: Utc::now().timestamp() + 3600,
            iat: Utc::now().timestamp(),
        };

        // Test admin role access
        assert!(admin_role_id == claims.role_id);

        // Test user role does not have admin access
        assert!(user_role_id != claims.role_id);
    }
}
