use axum::{extract::Path, http::StatusCode};
use or_status_code::OrInternalServerError;

use crate::axum::extractors::reference_count_repository::ReferenceCountRepositoryExtractor;
use crate::repository::reference_counts::ReferenceCountRepository;

pub async fn increment_reference_count(
    reference_count_repository: ReferenceCountRepositoryExtractor,
    Path(file_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    reference_count_repository
        .increment(&file_id)
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::OK)
}

pub async fn decrement_reference_count(
    reference_count_repository: ReferenceCountRepositoryExtractor,
    Path(file_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    reference_count_repository
        .decrement(&file_id)
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::OK)
}