use crate::models::skills::Skill;
use rocket::serde::json::Json;
use std::vec;
use uuid::Uuid;

#[get("/")]
pub fn get_all_skills() -> Json<Vec<Skill>> {
    let skills: Vec<Skill> = vec![Skill {
        id: Uuid::now_v7(),
        skill: "hoops".to_string(),
        level: 90,
    }];
    Json(skills)
}
