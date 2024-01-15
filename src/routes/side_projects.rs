use crate::models::side_projects::SideProject;
use rocket::serde::json::Json;
use std::vec;
use uuid::Uuid;

#[get("/")]
pub fn get_all_side_projects() -> Json<Vec<SideProject>> {
    let side_projects: Vec<SideProject> = vec![SideProject {
        id: Uuid::now_v7(),
        title: "rusty-resume".to_string(),
        tagline: "This API".to_string(),
        link: "http://127.0.0.1:8000".to_string(),
    }];
    Json(side_projects)
}

#[get("/<project_id>")]
pub fn get_side_project_by_id(project_id: Uuid) -> Json<SideProject> {
    let side_project: SideProject = SideProject {
        id: project_id,
        title: "rusty-resume".to_string(),
        tagline: "This API".to_string(),
        link: "http://127.0.0.1:8000".to_string(),
    };
    Json(side_project)
}
