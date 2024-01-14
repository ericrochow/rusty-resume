use rocket::serde::{json::Json, Serialize};
use std::vec;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BasicInfo {
    id: Uuid,
    fact: String,
    value: String,
}

#[get("/")]
pub fn get_basic_info() -> Json<Vec<BasicInfo>> {
    let basic_info: Vec<BasicInfo> = vec![BasicInfo {
        id: Uuid::now_v7(),
        fact: "foo".to_string(),
        value: "bar".to_string(),
    }];
    Json(basic_info)
}

#[get("/<fact>")]
pub fn get_basic_info_fact(fact: String) -> Json<BasicInfo> {
    let basic_info: BasicInfo = BasicInfo {
        id: Uuid::now_v7(),
        fact: fact.to_string(),
        value: "bar".to_string(),
    };
    Json(basic_info)
}
