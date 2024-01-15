use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Preference {
    pub id: Uuid,
    pub preference: String,
    pub value: String,
}
