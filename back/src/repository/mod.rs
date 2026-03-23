use std::sync::Arc;

use sqlx::SqlitePool;

use crate::repository::user_repository::UserRepository;

pub mod user_repository;

pub struct ApplicationRepo {
    pub user_repo: UserRepository,
}

impl ApplicationRepo {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            user_repo: UserRepository::new(pool),
        }
    }
}
