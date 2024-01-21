use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct SideProject {
    pub id: Uuid,
    pub title: String,
    pub tagline: String,
    pub link: String,
}

#[derive(Deserialize)]
pub struct SideProjectCreate {
    pub title: String,
    pub tagline: String,
    pub link: String,
}
