use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Competency {
    pub id: Uuid,
    pub competency: String,
}

#[derive(Deserialize)]
pub struct CompetencyCreate {
    pub competency: String,
}
