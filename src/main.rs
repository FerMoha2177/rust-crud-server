use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};

mod api;
mod application;
mod domain;  
mod data;  // Add this line to declare the data module

use tower_http::cors::CorsLayer;
use api::create_router;
use data::db_context::surreal_context::connect_db;


//#[tokio::main] is a procedural macro that transforms your async fn main() 
//unction into a regular fn main() that sets up the Tokio runtime 
//and executes your async code within it.
#[tokio::main]
async fn main(){
    connect_db().await.unwrap();
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}