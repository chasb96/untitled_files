use ::axum::{routing::get, Router};
use log_layer::LogLayer;
use routes::FilesRouter;

mod axum;
mod repository;
mod web;
mod configuration;
mod health;
mod routes;
mod persist;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .register_files_routes()
        .register_files_layers()
        .layer(LogLayer::new())
}