use axum::{http, response::IntoResponse, Json};
use serde_json::Value;

pub enum ApiError {
    BadRequest,
    SpecialError(Value),
    // Forbidden,
    // Unauthorized,
    InternalServerError,
    TemplateError(String),
}

// impl From<askama::Error> for ApiError {
//     fn from(value: askama::Error) -> Self {
//         ApiError::TemplateError(String::from(value.to_string()))
//     }
// }

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::TemplateError(text) => (http::StatusCode::BAD_REQUEST, text).into_response(),
            Self::BadRequest => (http::StatusCode::BAD_REQUEST).into_response(),
            Self::InternalServerError => (http::StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            Self::SpecialError(data) => (http::StatusCode::BAD_GATEWAY, Json(data)).into_response(),
        }
    }
}
