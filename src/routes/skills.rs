use crate::models::skills::Skill;
use axum::{
    // extract::Path,
    routing::get,
    Json,
    Router,
};
use std::vec;
use uuid::Uuid;

pub fn create_router() -> Router {
    Router::new().route("/", get(get_all_skills))
}

async fn get_all_skills() -> Json<Vec<Skill>> {
    let skills: Vec<Skill> = vec![Skill {
        id: Uuid::now_v7(),
        skill: "hoops".to_string(),
        level: 90,
    }];
    Json(skills)
}
