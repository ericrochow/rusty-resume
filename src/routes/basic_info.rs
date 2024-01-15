use crate::models::basic_info::BasicInfo;
use rocket::serde::json::Json;
use std::vec;
use uuid::Uuid;

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
