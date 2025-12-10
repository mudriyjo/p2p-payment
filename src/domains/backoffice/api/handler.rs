use crate::common::{app_state::AppState, dto::ApiResponse, error::AppError, jwt::Claims};
use crate::domains::backoffice::dto::user_dto::{
    CreateUserRequest, ListUsersQuery, UpdateUserRequest, UserResponse,
};
use axum::{
    extract::{Extension, Path, Query},
    Json,
};

use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/v1/user/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID to fetch")
    ),
    responses(
        (status = 200, description = "User found successfully", body = inline(ApiResponse<UserResponse>),
         example = json!({
             "status": 200,
             "message": "success",
             "data": {
                 "id": "550e8400-e29b-41d4-a716-446655440000",
                 "username": "john_doe",
                 "email": "john@example.com",
                 "is_active": true,
                 "role": {
                     "role_id": "878c19c6-643b-4a57-98f1-a60786a38a92",
                     "role_name": "Admin",
                     "role_description": "Administrator with full access"
                 },
                 "created_at": "2024-12-10T10:30:00Z",
                 "updated_at": "2024-12-10T10:30:00Z"
             }
         })
        ),
        (status = 401, description = "Missing or invalid JWT token"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "User not found")
    ),
    security(
        ("jwt_token" = [])
    ),
    tag = "Users",
    summary = "Get user by ID",
    description = "Retrieves a user's information by their unique ID. Requires authentication with a valid JWT token. User information includes role details."
)]
pub async fn get_user(
    Extension(state): Extension<Arc<AppState>>,
    Extension(_claims): Extension<Claims>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = state.user_get_use_case.execute(user_id).await?;

    let response = UserResponse::from(user);

    Ok(Json(ApiResponse::success(response)))
}

pub async fn get_current_user(
    Extension(state): Extension<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = state.user_get_use_case.execute(claims.user_id).await?;

    Ok(Json(ApiResponse::success(UserResponse::from(user))))
}

pub async fn list_users(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<ListUsersQuery>,
) -> Result<Json<ApiResponse<Vec<UserResponse>>>, AppError> {
    let users = if let Some(search_query) = params.search {
        state
            .user_get_use_case
            .search(&search_query, Some(params.limit))
            .await?
    } else {
        state
            .user_get_use_case
            .list(params.limit, params.offset)
            .await?
    };

    let response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    Ok(Json(ApiResponse::success(response)))
}

pub async fn create_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = state.user_create_use_case.execute(request).await?;

    Ok(Json(ApiResponse::success(UserResponse::from(user))))
}

pub async fn update_user(
    Extension(state): Extension<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = state.user_update_use_case.execute(user_id, request).await?;

    Ok(Json(ApiResponse::success(UserResponse::from(user))))
}

pub async fn delete_user(
    Extension(state): Extension<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.user_delete_use_case.execute(user_id).await?;

    Ok(Json(ApiResponse::success(())))
}
