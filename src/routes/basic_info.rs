use rocket::serde::{json::Json, Serialize};
use std::vec;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BasicInfo {
    id: i32,
    fact: String,
    value: String,
}

#[get("/basic_info")]
pub fn get_basic_info() -> Json<Vec<BasicInfo>> {
    let basic_info: Vec<BasicInfo> = vec![BasicInfo {
        id: 1,
        fact: "foo".to_string(),
        value: "bar".to_string(),
    }];
    Json(basic_info)
}
