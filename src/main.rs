#[macro_use]
extern crate rocket;

mod routes;

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
            "/preferences",
            routes![
                routes::preferences::get_preference,
                routes::preferences::get_preferences,
            ],
        )
        .mount(
            "/users",
            routes![routes::users::get_all_users, routes::users::get_my_user,],
        )
}
