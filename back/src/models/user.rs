use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}
