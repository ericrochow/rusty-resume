#[macro_use]
extern crate rocket;

mod routes;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            ping,
            routes::basic_info::get_basic_info,
            routes::users::get_all_users,
            routes::users::get_my_user,
        ],
    )
}
