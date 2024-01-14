use rocket::serde::{json::Json, Serialize};
use std::vec;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Competency {
    id: Uuid,
    competency: String,
}

#[get("/")]
pub fn get_competencies() -> Json<Vec<Competency>> {
    let competencies: Vec<Competency> = vec![Competency {
        id: Uuid::now_v7(),
        competency: "synergy".to_string(),
    }];
    Json(competencies)
}
