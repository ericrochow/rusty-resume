use rocket::serde::{json::Json, Serialize};
use std::vec;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    id: i32,
    username: String,
}

#[get("/")]
pub fn get_all_users() -> Json<Vec<User>> {
    let users: Vec<User> = vec![
        User {
            id: 1,
            username: "John".to_string(),
        },
        User {
            id: 2,
            username: "Steve".to_string(),
        },
    ];
    Json(users)
}

#[get("/me")]
pub fn get_my_user() -> Json<User> {
    Json(User {
        id: 1,
        username: "John".to_string(),
    })
}
