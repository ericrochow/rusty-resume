use axum;
use axum::{
    // http::StatusCode,
    // response::{IntoResponse, Response},
    routing::get,
    // Json,
    Router,
};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

mod crud;
mod models;
mod routes;

async fn ping() -> &'static str {
    "pong"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        // .compact()
        .json()
        .init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/ping", get(ping))
        .nest("/basic_info", routes::basic_info::create_router())
        .nest("/certifications", routes::certifications::create_router())
        .nest("/competencies", routes::competencies::create_router())
        .nest("/education", routes::education::create_router())
        .nest("/experience", routes::experience::create_router())
        .nest("/interests", routes::interests::create_router())
        .nest("/preferences", routes::preferences::create_router())
        .nest("/side_projects", routes::side_projects::create_router())
        .nest("/skills", routes::skills::create_router())
        .nest("/social_links", routes::social_links::create_router())
        .nest("/users", routes::users::create_router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
