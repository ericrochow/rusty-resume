use crate::models::users::User;
use axum::{
    // extract::Path,
    routing::get,
    Json,
    Router,
};
use std::vec;
use uuid::Uuid;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_all_users))
        .route("/me", get(get_my_user))
}

async fn get_all_users() -> Json<Vec<User>> {
    let users: Vec<User> = vec![
        User {
            id: Uuid::now_v7(),
            username: "John".to_string(),
        },
        User {
            id: Uuid::now_v7(),
            username: "Steve".to_string(),
        },
    ];
    Json(users)
}

async fn get_my_user() -> Json<User> {
    Json(User {
        id: Uuid::now_v7(),
        username: "John".to_string(),
    })
}
