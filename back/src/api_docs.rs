use utoipa::OpenApi;

use crate::models::user::User;

// Определение OpenAPI документации
#[derive(OpenApi)]
#[openapi(
    paths(

    ),
    components(
        schemas()
    ),
    tags(
        (name = "users", description = "Управление пользователями"),
        (name = "health", description = "Проверка состояния API")
    ),
)]
pub struct ApiDoc;
