mod api_docs;
mod api_errors;
mod api_responses;
mod db;
mod handlers;
mod models;
mod repository;

use std::net::SocketAddr;

use axum::{
    http::HeaderValue,
    routing::{get, post},
    Extension, Router,
};
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
// use utoipa_swagger_ui::SwaggerUi;

use crate::{api_docs::ApiDoc, db::create_pool};

#[tokio::main]
async fn main() {
    // let cors_layer = CorsLayer::permissive();

    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().expect("Cannot parse"))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    // .allow_credentials(true);
    let pool = create_pool().await.expect("DB pool not created");
    let router = Router::new()
        .route("/api", get(|| async { "Hello, World!" }))
        .route("/api/json", get(|| async { "{\"key\":\"value\"}" }))
        .route("/api/users", get(handlers::users::all_users))
        .route("/api/users", post(handlers::users::create_user))
        .route("/api/users/{id}", get(handlers::users::get_by_id))
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .layer(Extension(pool));
    //

    let addr = SocketAddr::from(([0, 0, 0, 0], 5005));
    let tcp = TcpListener::bind(&addr)
        .await
        .expect("failed to bind to socket");
    println!("Server started!");
    axum::serve(tcp, router).await.expect("Error start server");
}
