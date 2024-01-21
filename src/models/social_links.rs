use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct SocialLink {
    pub id: Uuid,
    pub platform: String,
    pub link: String,
}

#[derive(Deserialize)]
pub struct SocialLinkCreate {
    pub platform: String,
    pub link: String,
}
