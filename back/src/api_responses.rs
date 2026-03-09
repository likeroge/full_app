use axum::{http, response::IntoResponse, Json};
use serde_json::Value;

pub enum ApiResponse {
    Ok,
    Created,
    JsonData(Value),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Ok => (http::StatusCode::OK).into_response(),
            Self::Created => (http::StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (http::StatusCode::OK, Json(data)).into_response(),
        }
    }
}
