use std::sync::Arc;

use sqlx::{SqlitePool, query_as};

use crate::models::{dto::users::CreateUserDto, user::User};

pub struct UserRepository {
    pool: Arc<SqlitePool>,
}

impl UserRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = query_as!(User, "SELECT id, name, email FROM users")
            .fetch_all(&*self.pool)
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
        .fetch_one(&*self.pool)
        .await?;
        Ok(result)
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        let result = query_as!(User, "SELECT id,name,email from users where id=$1", id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(result)
    }
}
