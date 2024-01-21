use crate::models::side_projects::SideProject;
use axum::{extract::Path, routing::get, Json, Router};
use std::vec;
use uuid::Uuid;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_all_side_projects))
        .route("/:project_id", get(get_side_project_by_id))
}

async fn get_all_side_projects() -> Json<Vec<SideProject>> {
    let side_projects: Vec<SideProject> = vec![SideProject {
        id: Uuid::now_v7(),
        title: "rusty-resume".to_string(),
        tagline: "This API".to_string(),
        link: "http://127.0.0.1:8000".to_string(),
    }];
    Json(side_projects)
}

async fn get_side_project_by_id(Path(project_id): Path<Uuid>) -> Json<SideProject> {
    let side_project: SideProject = SideProject {
        id: project_id,
        title: "rusty-resume".to_string(),
        tagline: "This API".to_string(),
        link: "http://127.0.0.1:8000".to_string(),
    };
    Json(side_project)
}
