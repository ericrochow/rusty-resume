use crate::models::interests::InterestResponse;
use axum::{extract::Path, routing::get, Json, Router};
use std::vec;
// use uuid::Uuid;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_all_interests))
        .route("/:interest_type", get(get_by_interests_by_type))
}

async fn get_all_interests() -> Json<InterestResponse> {
    let interests: InterestResponse = InterestResponse {
        personal: Some(vec!["stuff".to_string(), "things".to_string()]),
        technical: Some(vec!["rust".to_string()]),
    };
    Json(interests)
}

async fn get_by_interests_by_type(Path(interest_type): Path<String>) -> Json<InterestResponse> {
    let mut interests: InterestResponse = InterestResponse {
        personal: None,
        technical: None,
    };

    match interest_type.as_str() {
        "personal" => interests.personal = Some(vec!["stuff".to_string(), "things".to_string()]),
        "technical" => interests.technical = Some(vec!["stuff".to_string(), "things".to_string()]),
        _ => {
            interests.personal = Some(vec!["stuff".to_string(), "things".to_string()]);
            interests.technical = Some(vec!["stuff".to_string(), "things".to_string()]);
        }
    }
    Json(interests)
}
