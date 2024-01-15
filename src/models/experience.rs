use rocket::serde::Serialize;
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
    pub id: Uuid,
    pub highlight: String,
    pub experience_id: Option<Uuid>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ExperienceDetail {
    pub id: Uuid,
    pub detail: String,
    pub experience_id: Option<Uuid>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ExperienceResponse {
    pub id: Uuid,
    pub employeer: String,
    pub employer_summary: String,
    pub location: String,
    pub job_title: String,
    pub job_summary: String,
    pub time: String,
    pub highlights: Option<Vec<ExperienceHighlight>>,
    pub details: Option<Vec<ExperienceDetail>>,
}
