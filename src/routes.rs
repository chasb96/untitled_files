use axum::{extract::DefaultBodyLimit, routing::{delete, get, post}, Router};
use super::web::*;

pub trait FilesRouter {
    fn register_files_routes(self) -> Self;

    fn register_files_layers(self) -> Self;
}

impl FilesRouter for Router {
    fn register_files_routes(self) -> Self {
        self.route("/files", post(create_file))
            .route("/files", get(list_metadata))
            .route("/files/:id", get(get_by_id))
            .route("/files/:id/verification", get(get_verification_by_id))
            .route("/reference_counts/:file_id", get(increment_reference_count))
            .route("/reference_counts/:file_id", delete(decrement_reference_count))
    }
    
    fn register_files_layers(self) -> Self {
        self.layer(DefaultBodyLimit::max(67108864))
    }
}