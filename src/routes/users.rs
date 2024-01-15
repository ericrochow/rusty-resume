use crate::models::users::User;
use rocket::serde::json::Json;
use std::vec;
use uuid::Uuid;

#[get("/")]
pub fn get_all_users() -> Json<Vec<User>> {
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

#[get("/me")]
pub fn get_my_user() -> Json<User> {
    Json(User {
        id: Uuid::now_v7(),
        username: "John".to_string(),
    })
}
