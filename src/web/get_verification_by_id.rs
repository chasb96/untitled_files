use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, http::StatusCode, Json};
use or_status_code::{OrInternalServerError, OrNotFound};
use serde::Serialize;

use crate::axum::extractors::metadata_repository::MetadataRepositoryExtractor;
use crate::axum::extractors::verification_repository::VerificationRepositoryExtractor;
use crate::repository::verification::{VerificationRepository, VerificationState};
use crate::repository::metadata::MetadataRepository;

#[derive(Serialize)]
pub enum GetVerificationByIdResponse {
    #[serde(rename = "u")]
    Unknown,
    #[serde(rename = "p")]
    Pending,
    #[serde(rename = "a")]
    Accepted,
    #[serde(rename = "r")]
    Rejected,
    #[serde(rename = "e")]
    Error,
}

impl From<VerificationState> for GetVerificationByIdResponse {
    fn from(state: VerificationState) -> Self {
        match state {
            VerificationState::Unknown => GetVerificationByIdResponse::Unknown,
            VerificationState::Pending => GetVerificationByIdResponse::Pending,
            VerificationState::Accepted => GetVerificationByIdResponse::Accepted,
            VerificationState::Rejected => GetVerificationByIdResponse::Rejected,
            VerificationState::Error => GetVerificationByIdResponse::Error,
        }
    }
}

pub async fn get_verification_by_id(
    Authenticate(user):  Authenticate<ClaimsUser>,
    metadata_repository: MetadataRepositoryExtractor,
    verification_repository: VerificationRepositoryExtractor,
    Path(file_id): Path<String>
) -> Result<Json<GetVerificationByIdResponse>, StatusCode> {
    let file_metadata = metadata_repository
        .get_by_id(&file_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if file_metadata.user_id != user.id {
        return Err(StatusCode::NOT_FOUND);
    }

    let verification = verification_repository
        .get_by_file_id(&file_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    Ok(Json(verification.into()))
}