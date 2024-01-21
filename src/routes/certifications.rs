use crate::models::certifications::Certification;
use axum::{extract::Path, routing::get, Json, Router};
use std::vec;
use uuid::Uuid;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_certifications))
        .route("/:certification", get(get_certification))
}

async fn get_certifications() -> Json<Vec<Certification>> {
    let certifications: Vec<Certification> = vec![Certification {
        id: Uuid::now_v7(),
        cert: "ccna".to_string(),
        full_name: "Cisco Certified Network Administrator".to_string(),
        time: "2011 - Present".to_string(),
        valid: true,
        progress: 100,
    }];
    Json(certifications)
}

async fn get_certification(Path(certification): Path<String>) -> Json<Certification> {
    let certification: Certification = Certification {
        id: Uuid::now_v7(),
        cert: certification.to_string(),
        full_name: "Cisco Certified Network Administrator".to_string(),
        time: "2011 - Present".to_string(),
        valid: true,
        progress: 100,
    };
    Json(certification)
}
