use axum::extract::Path;
use axum::{Extension, Json};
use serde_json::json;
use sqlx::SqlitePool;

use crate::{
    api_errors::ApiError,
    api_responses::ApiResponse,
    models::{dto::users::CreateUserDto, user::User},
    repository::user_repository::UserRepository,
};

pub async fn create_user(
    Extension(pool): Extension<SqlitePool>,
    Json(new_user): Json<CreateUserDto>,
) -> Result<ApiResponse, ApiError> {
    let user_repo = UserRepository::new(pool);
    // match user_repo.await.create(new_user).await {
    match user_repo.create(new_user).await {
        Ok(user) => Ok(ApiResponse::JsonData(json!(user))),
        Err(e) => Err(ApiError::BadRequest),
    }
}

pub async fn get_by_id(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<ApiResponse, ApiError> {
    let repo = UserRepository::new(pool);
    match repo.get_user_by_id(id).await {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}

#[utoipa::path(get, path="/api/users",responses(
        (status = 200, description = "Список пользователей"),
        (status = 500, description = "Внутренняя ошибка сервера")
    ),
    tag = "users")]
pub async fn all_users(Extension(pool): Extension<SqlitePool>) -> Result<ApiResponse, ApiError> {
    let user_repo = UserRepository::new(pool);

    // match user_repo.await.get_all().await {
    match user_repo.get_all().await {
        Ok(users) => Ok(ApiResponse::JsonData(json!(users))),
        Err(e) => Err(ApiError::BadRequest),
    }
}
