use crate::models::preferences::Preference;
use rocket::serde::json::Json;
use std::vec;
use uuid::Uuid;

#[get("/")]
pub fn get_preferences() -> Json<Vec<Preference>> {
    let preferences: Vec<Preference> = vec![Preference {
        id: Uuid::now_v7(),
        preference: "color".to_string(),
        value: "blurple".to_string(),
    }];
    Json(preferences)
}

#[get("/<preference>")]
pub fn get_preference(preference: String) -> Json<Preference> {
    let preference: Preference = Preference {
        id: Uuid::now_v7(),
        preference: preference.to_string(),
        value: "right".to_string(),
    };
    Json(preference)
}
