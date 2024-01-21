use crate::models::social_links::SocialLink;
use axum::{extract::Path, routing::get, Json, Router};
use std::vec;
use uuid::Uuid;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_all_social_links))
        .route("/:platform", get(get_social_link_by_platform))
}

async fn get_all_social_links() -> Json<Vec<SocialLink>> {
    let social_links: Vec<SocialLink> = vec![SocialLink {
        id: Uuid::now_v7(),
        platform: "linkedin".to_string(),
        link: "https://linkedin.com/in/my_user".to_string(),
    }];
    Json(social_links)
}

async fn get_social_link_by_platform(Path(platform): Path<String>) -> Json<SocialLink> {
    let social_link: SocialLink = SocialLink {
        id: Uuid::now_v7(),
        platform: platform.to_string(),
        link: format!("https://{}.com/my_profile", platform),
    };
    Json(social_link)
}
