use crate::models::certifications::Certification;
use rocket::serde::json::Json;
use std::vec;
use uuid::Uuid;

#[get("/")]
pub fn get_certifications() -> Json<Vec<Certification>> {
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

#[get("/<certification>")]
pub fn get_certification(certification: String) -> Json<Certification> {
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
