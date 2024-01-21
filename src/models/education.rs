use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Education {
    pub id: Uuid,
    pub institution: String,
    pub degree: String,
    pub graduation_date: u16,
    pub gpa: f32,
}

#[derive(Deserialize)]
pub struct EducationCreate {
    pub institution: String,
    pub degree: String,
    pub graduation_date: u16,
    pub gpa: f32,
}
