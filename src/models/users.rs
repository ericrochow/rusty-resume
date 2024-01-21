use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserCreate {
    pub username: String,
}
