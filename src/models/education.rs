use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Education {
    pub id: Uuid,
    pub institution: String,
    pub degree: String,
    pub graduation_date: u16,
    pub gpa: f32,
}
