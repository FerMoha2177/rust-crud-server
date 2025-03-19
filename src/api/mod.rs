// api/mod.rss 
use axum::{Json, response::IntoResponse};
use serde_json;

pub mod router;

// Re-export important items for easier access
pub use router::create_router;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Working fine, thanks!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
    
}