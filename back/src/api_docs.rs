use utoipa::OpenApi;

// use crate::handlers::users::get_by_id;
use crate::models::user::User;

// Определение OpenAPI документации
#[derive(OpenApi)]
#[openapi(
    paths(
        // get_by_id
    // get_by_id
        ),
    components(
        schemas(
            User
            )
    ),
    tags(
        (name = "users", description = "Управление пользователями"),
        (name = "health", description = "Проверка состояния API")
    ),
)]
pub struct ApiDoc;
