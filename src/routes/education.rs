use crate::models::education::Education;
use rocket::serde::json::Json;
use std::vec;
use uuid::Uuid;

#[get("/")]
pub fn get_education_history() -> Json<Vec<Education>> {
    let education: Vec<Education> = vec![Education {
        id: Uuid::now_v7(),
        institution: "School of Hard Knocks".to_string(),
        degree: "BA".to_string(),
        graduation_date: 1987,
        gpa: 5.0,
    }];
    Json(education)
}

#[get("/<education_id>")]
pub fn get_education_item(education_id: Uuid) -> Json<Education> {
    let education: Education = Education {
        id: education_id,
        institution: "School of Hard Knocks".to_string(),
        degree: "BA".to_string(),
        graduation_date: 1987,
        gpa: 5.0,
    };
    Json(education)
}
