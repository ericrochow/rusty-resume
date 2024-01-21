use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Certification {
    pub id: Uuid,
    pub cert: String,
    pub full_name: String,
    pub time: String,
    pub valid: bool,
    pub progress: u8,
}

#[derive(Deserialize)]
pub struct CertificationCreate {
    pub cert: String,
    pub full_name: String,
    pub time: String,
    pub valid: bool,
    pub progress: u8,
}
