use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Skill {
    pub id: Uuid,
    pub skill: String,
    pub level: u8,
}

#[derive(Deserialize)]
pub struct SkillCreate {
    pub skill: String,
    pub level: u8,
}
