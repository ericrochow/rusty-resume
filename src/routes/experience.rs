use rocket::serde::{json::Json, Serialize};
use std::vec;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Experience {
    id: Uuid,
    employeer: String,
    employer_summary: String,
    location: String,
    job_title: String,
    job_summary: String,
    time: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ExperienceHighlight {
    id: Uuid,
    highlight: String,
    experience_id: Option<Uuid>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ExperienceDetail {
    id: Uuid,
    detail: String,
    experience_id: Option<Uuid>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ExperienceResponse {
    id: Uuid,
    employeer: String,
    employer_summary: String,
    location: String,
    job_title: String,
    job_summary: String,
    time: String,
    highlights: Option<Vec<ExperienceHighlight>>,
    details: Option<Vec<ExperienceDetail>>,
}

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
