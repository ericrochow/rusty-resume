use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct BasicInfo {
    pub id: Uuid,
    pub fact: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct BasicInfoCrete {
    pub fact: String,
    pub value: String,
}
