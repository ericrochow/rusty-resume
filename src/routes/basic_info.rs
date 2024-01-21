use crate::models::basic_info::BasicInfo;
// use crate::models::response::IntoResponse;
use axum::{extract::Path, routing::get, Json, Router};
use std::vec;
use uuid::Uuid;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_basic_info))
        .route("/:id", get(get_basic_info_fact))
}

async fn get_basic_info() -> Json<Vec<BasicInfo>> {
    let basic_info: Vec<BasicInfo> = vec![BasicInfo {
        id: Uuid::now_v7(),
        fact: "foo".to_string(),
        value: "bar".to_string(),
    }];
    Json(basic_info)
}

async fn get_basic_info_fact(Path(fact): Path<String>) -> Json<BasicInfo> {
    let basic_info: BasicInfo = BasicInfo {
        id: Uuid::now_v7(),
        fact: fact.to_string(),
        value: "bar".to_string(),
    };
    Json(basic_info)
}
