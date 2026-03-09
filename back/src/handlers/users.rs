use serde_json::json;

use crate::{api_errors::ApiError, api_responses::ApiResponse, models::user::User};

pub async fn all_users() -> Result<ApiResponse, ApiError> {
    let mut all_users: Vec<User> = vec![];
    let u1 = User {
        id: 1,
        name: String::from("John Doe"),
        email: String::from("jL2r5@example.com"),
    };

    let u2 = User {
        id: 2,
        name: String::from("Gagagagag Doe"),
        email: String::from("hasdasd5@xcxcsle.com"),
    };

    all_users.push(u1);
    all_users.push(u2);

    Ok(ApiResponse::JsonData(json!(all_users)))
}

