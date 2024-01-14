use rocket::serde::{json::Json, Serialize};
use std::vec;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Certification {
    id: Uuid,
    cert: String,
    full_name: String,
    time: String,
    valid: bool,
    progress: u8,
}

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
