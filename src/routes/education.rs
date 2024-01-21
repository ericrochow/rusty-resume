use crate::models::education::Education;
use axum::{extract::Path, routing::get, Json, Router};
use std::vec;
use uuid::Uuid;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_education_history))
        .route("/:id", get(get_education_item))
}

async fn get_education_history() -> Json<Vec<Education>> {
    let education: Vec<Education> = vec![Education {
        id: Uuid::now_v7(),
        institution: "School of Hard Knocks".to_string(),
        degree: "BA".to_string(),
        graduation_date: 1987,
        gpa: 5.0,
    }];
    Json(education)
}

async fn get_education_item(Path(education_id): Path<Uuid>) -> Json<Education> {
    let education: Education = Education {
        id: education_id,
        institution: "School of Hard Knocks".to_string(),
        degree: "BA".to_string(),
        graduation_date: 1987,
        gpa: 5.0,
    };
    Json(education)
}
