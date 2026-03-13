use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Default, Serialize, Deserialize, Debug, sqlx::FromRow, ToSchema)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}
