use std::sync::Arc;

use axum::{Extension, extract::Path};
use serde_json::json;

use crate::{api_errors::ApiError, api_responses::ApiResponse, repository::ApplicationRepo};

pub trait Crud {
    async fn get_all(app_repo: Extension<Arc<ApplicationRepo>>) -> Result<ApiResponse, ApiError>;
    // async fn get_by_id(Extension(pool): Extension<SqlitePool>, Path(id): Path<i32>);
    async fn get_by_id(
        app_repo: Extension<Arc<ApplicationRepo>>,
        id: Path<i32>,
    ) -> Result<ApiResponse, ApiError>;
}

#[derive(Default)]
pub struct FlightsHandler {}

impl Crud for FlightsHandler {
    async fn get_all(
        Extension(app_repo): Extension<Arc<ApplicationRepo>>,
    ) -> Result<ApiResponse, ApiError> {
        match app_repo.user_repo.get_all().await {
            Ok(ac) => Ok(ApiResponse::JsonData(json!(ac))),
            Err(e) => Err(ApiError::BadRequest),
        }
    }

    async fn get_by_id(
        Extension(repo): Extension<Arc<ApplicationRepo>>,
        Path(id): Path<i32>,
    ) -> Result<ApiResponse, ApiError> {
        match repo.user_repo.get_all().await {
            Ok(ac) => Ok(ApiResponse::JsonData(json!(ac))),
            Err(_) => Err(ApiError::BadRequest),
        }
    }
}
