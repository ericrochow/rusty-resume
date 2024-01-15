use crate::models::competencies::Competency;
use rocket::serde::json::Json;
use std::vec;
use uuid::Uuid;

#[get("/")]
pub fn get_competencies() -> Json<Vec<Competency>> {
    let competencies: Vec<Competency> = vec![Competency {
        id: Uuid::now_v7(),
        competency: "synergy".to_string(),
    }];
    Json(competencies)
}
