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
    match user_repo.await.create(new_user).await {
        Ok(user) => Ok(ApiResponse::JsonData(json!(user))),
        Err(e) => Err(ApiError::BadRequest),
    }
}

pub async fn all_users(Extension(pool): Extension<SqlitePool>) -> Result<ApiResponse, ApiError> {
    let user_repo = UserRepository::new(pool);

    match user_repo.await.get_all().await {
        Ok(users) => Ok(ApiResponse::JsonData(json!(users))),
        Err(e) => Err(ApiError::BadRequest),
    }

    // let mut all_users: Vec<User> = vec![];
    // let u1 = User {
    //     id: 1,
    //     name: String::from("John Doe"),
    //     email: String::from("jL2r5@example.com"),
    // };

    // let u2 = User {
    //     id: 2,
    //     name: String::from("Gagagagag Doe"),
    //     email: String::from("hasdasd5@xcxcsle.com"),
    // };

    // all_users.push(u1);
    // all_users.push(u2);
}
