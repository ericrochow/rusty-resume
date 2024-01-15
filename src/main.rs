#[macro_use]
extern crate rocket;

mod crud;
mod models;
mod routes;

pub use models::basic_info::BasicInfo;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ping,])
        .mount(
            "/basic_info",
            routes![
                routes::basic_info::get_basic_info,
                routes::basic_info::get_basic_info_fact,
            ],
        )
        .mount(
            "/certifications",
            routes![
                routes::certifications::get_certification,
                routes::certifications::get_certifications,
            ],
        )
        .mount(
            "/competencies",
            routes![routes::competencies::get_competencies,],
        )
        .mount(
            "/education",
            routes![
                routes::education::get_education_history,
                routes::education::get_education_item,
            ],
        )
        .mount(
            "/experience",
            routes![routes::experience::get_experience_history,],
        )
        .mount(
            "/interests",
            routes![
                routes::interests::get_all_interests,
                routes::interests::get_by_interests_by_type,
            ],
        )
        .mount(
            "/preferences",
            routes![
                routes::preferences::get_preference,
                routes::preferences::get_preferences,
            ],
        )
        .mount(
            "/side_projects",
            routes![
                routes::side_projects::get_all_side_projects,
                routes::side_projects::get_side_project_by_id,
            ],
        )
        .mount("/skills", routes![routes::skills::get_all_skills])
        .mount(
            "/social_links",
            routes![
                routes::social_links::get_all_social_links,
                routes::social_links::get_social_link_by_platform
            ],
        )
        .mount(
            "/users",
            routes![routes::users::get_all_users, routes::users::get_my_user,],
        )
}
