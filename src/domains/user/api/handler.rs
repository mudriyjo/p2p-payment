use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::common::{
    app_state::AppState,
    dto::ApiResponse,
    error::AppError,
    jwt::Claims,
};
use crate::domains::user::dto::user_dto::{
    UserResponse, 
    // UpdateUserRequest, 
    ListUsersQuery,
    // DeleteUserRequest,
};

pub async fn get_user(
    Extension(state): Extension<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = state
        .user_get_use_case
        .execute(user_id, Some(claims.user_id))
        .await?;

    let response = UserResponse::from(user);

    Ok(Json(ApiResponse::success(response)))
}

pub async fn get_current_user(
    Extension(state): Extension<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = state
        .user_get_use_case
        .execute(claims.user_id, Some(claims.user_id))
        .await?;

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

    let response: Vec<UserResponse> = users
        .into_iter()
        .map(UserResponse::from)
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

// pub async fn update_user(
//     Extension(state): Extension<Arc<AppState>>,
//     Extension(claims): Extension<Claims>,
//     Path(user_id): Path<Uuid>,
//     Json(request): Json<UpdateUserRequest>,
// ) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
//     let update_request = UpdateUserRequest {
//         username: request.username,
//         email: request.email,
//     };

//     let user = state
//         .user_update_profile_use_case
//         .execute(user_id, claims.user_id, update_request)
//         .await?;

//     Ok(Json(ApiResponse::success(UserResponse::from(user))))
// }

// pub async fn delete_user(
//     Extension(state): Extension<Arc<AppState>>,
//     Extension(claims): Extension<Claims>,
//     Path(user_id): Path<Uuid>,
//     Json(request): Json<DeleteUserRequest>,
// ) -> Result<Json<ApiResponse<()>>, AppError> {
//     state
//         .user_delete_use_case
//         .execute(user_id, claims.user_id, request.password_confirmation)
//         .await?;

//     Ok(Json(ApiResponse::success(())))
// }