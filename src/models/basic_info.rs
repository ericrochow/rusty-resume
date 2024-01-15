use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BasicInfo {
    pub id: Uuid,
    pub fact: String,
    pub value: String,
}
