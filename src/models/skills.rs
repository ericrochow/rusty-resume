use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Skill {
    pub id: Uuid,
    pub skill: String,
    pub level: u8,
}
