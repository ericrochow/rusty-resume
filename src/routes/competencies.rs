use crate::models::competencies::Competency;
use axum::{
    // extract::Path,
    routing::get,
    Json,
    Router,
};
use std::vec;
use uuid::Uuid;

pub fn create_router() -> Router {
    Router::new().route("/", get(get_competencies))
}

pub async fn get_competencies() -> Json<Vec<Competency>> {
    let competencies: Vec<Competency> = vec![Competency {
        id: Uuid::now_v7(),
        competency: "synergy".to_string(),
    }];
    Json(competencies)
}
