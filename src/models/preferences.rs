use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Preference {
    pub id: Uuid,
    pub preference: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct PreferenceCreate {
    pub preference: String,
    pub value: String,
}
