use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SocialLink {
    pub id: Uuid,
    pub platform: String,
    pub link: String,
}
