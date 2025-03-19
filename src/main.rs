// use std::net::SocketAddr;
// use axum::{Router, routing::get};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;


// mod api;
// mod arbitrage;
// mod blockchain;
// mod models;

// #[tokio::main]
// async fn main() {
//     // Initialize logging
//     tracing_subscriber::fmt::init();
    
//     // Load environment variables
//     dotenv::dotenv().ok();
    
//     // Build our application with routes
//     let app = Router::new()
//         .route("/health", get(health_check));
    
//     // Run the server
//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     tracing::info!("listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// async fn health_check() -> &'static str {
//     "Arbitrage server is running"
// }


//temp main test


mod api;
// Other modules can be uncommented when you need them
// mod arbitrage;
// mod blockchain;
// mod models;

use api::create_router;

//#[tokio::main] is a procedural macro that transforms your async fn main() 
//unction into a regular fn main() that sets up the Tokio runtime 
//and executes your async code within it.
#[tokio::main]
async fn main(){
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