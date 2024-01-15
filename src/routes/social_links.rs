use crate::models::social_links::SocialLink;
use rocket::serde::json::Json;
use std::vec;
use uuid::Uuid;

#[get("/")]
pub fn get_all_social_links() -> Json<Vec<SocialLink>> {
    let social_links: Vec<SocialLink> = vec![SocialLink {
        id: Uuid::now_v7(),
        platform: "linkedin".to_string(),
        link: "https://linkedin.com/in/my_user".to_string(),
    }];
    Json(social_links)
}

#[get("/<platform>")]
pub fn get_social_link_by_platform(platform: String) -> Json<SocialLink> {
    let social_link: SocialLink = SocialLink {
        id: Uuid::now_v7(),
        platform: platform.to_string(),
        link: format!("https://{}.com/my_profile", platform),
    };
    Json(social_link)
}
