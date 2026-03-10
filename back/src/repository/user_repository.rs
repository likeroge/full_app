use sqlx::{SqlitePool, query_as};

use crate::models::{dto::users::CreateUserDto, user::User};

pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub async fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = query_as!(User, "SELECT id, name, email FROM users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    pub async fn create(&self, user_dto: CreateUserDto) -> Result<User, sqlx::Error> {
        let result = query_as!(
            User,
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
            user_dto.name,
            user_dto.email
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }
}
