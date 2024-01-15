use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SideProject {
    pub id: Uuid,
    pub title: String,
    pub tagline: String,
    pub link: String,
}
