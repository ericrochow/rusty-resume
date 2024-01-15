use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Certification {
    pub id: Uuid,
    pub cert: String,
    pub full_name: String,
    pub time: String,
    pub valid: bool,
    pub progress: u8,
}
