use crate::models::experience::{ExperienceDetail, ExperienceHighlight, ExperienceResponse};
use rocket::serde::json::Json;
use std::vec;
use uuid::Uuid;

#[get("/")]
pub fn get_experience_history() -> Json<Vec<ExperienceResponse>> {
    let experience: Vec<ExperienceResponse> = vec![ExperienceResponse {
        id: Uuid::now_v7(),
        employeer: "Acme, Inc.".to_string(),
        employer_summary: "Supplying coyotes with explosives since 1920.".to_string(),
        location: "Warner Bros. Studios".to_string(),
        job_title: "Head of R&D".to_string(),
        job_summary: "Blew stuff up lol".to_string(),
        time: "1992 - 1992".to_string(),
        highlights: Some(vec![ExperienceHighlight {
            id: Uuid::now_v7(),
            highlight: "Did a thing".to_string(),
            experience_id: None,
        }]),
        details: Some(vec![ExperienceDetail {
            id: Uuid::now_v7(),
            detail: "Greenlit an anvil".to_string(),
            experience_id: None,
        }]),
    }];
    Json(experience)
}
