use crate::models::preferences::Preference;
use axum::{extract::Path, routing::get, Json, Router};
use std::vec;
use uuid::Uuid;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_preferences))
        .route("/:preference", get(get_preference))
}

async fn get_preferences() -> Json<Vec<Preference>> {
    let preferences: Vec<Preference> = vec![Preference {
        id: Uuid::now_v7(),
        preference: "color".to_string(),
        value: "blurple".to_string(),
    }];
    Json(preferences)
}

async fn get_preference(Path(preference): Path<String>) -> Json<Preference> {
    let preference: Preference = Preference {
        id: Uuid::now_v7(),
        preference: preference.to_string(),
        value: "right".to_string(),
    };
    Json(preference)
}
