// api/router.rs

use axum::{
    routing::{get, post},
    Router,
};

use super::health_checker_handler;
use crate::application::commands::create_todo_command;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/todos", post(create_todo_command))

}
