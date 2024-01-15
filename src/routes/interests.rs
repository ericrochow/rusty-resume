use crate::models::interests::InterestResponse;
use rocket::serde::json::Json;
use std::vec;
// use uuid::Uuid;

#[get("/")]
pub fn get_all_interests() -> Json<InterestResponse> {
    let interests: InterestResponse = InterestResponse {
        personal: Some(vec!["stuff".to_string(), "things".to_string()]),
        technical: Some(vec!["rust".to_string()]),
    };
    Json(interests)
}

#[get("/<interest_type>")]
pub fn get_by_interests_by_type(interest_type: String) -> Json<InterestResponse> {
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
