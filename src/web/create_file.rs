use std::path::PathBuf;
use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Multipart, http::StatusCode, Json};
use futures::TryStreamExt;
use or_status_code::{OrInternalServerError, OrBadRequest};
use rand::distributions::{Alphanumeric, DistString};
use serde::Serialize;

use crate::axum::extractors::metadata_extraction::MetadataExtractionExtractor;
use crate::file_format::FileFormat;
use crate::axum::extractors::persistor::PersistorExtractor;
use crate::persist::Persistor;
use crate::metadata_extraction::Message as MetadataExtractionMessage;

#[derive(Serialize)]
pub struct CreateFileResponse {
    #[serde(rename = "i")]
    pub id: String,
}

pub async fn create_file(
    Authenticate(user): Authenticate<ClaimsUser>,
    persistor: PersistorExtractor,
    metadata_extraction: MetadataExtractionExtractor,
    mut request: Multipart
) -> Result<(StatusCode, Json<CreateFileResponse>), StatusCode> {
    let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let key = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let field = request
        .next_field()
        .await
        .or_internal_server_error()?
        .or_bad_request()?;

    let path = field
        .name()
        .map(PathBuf::from)
        .or_bad_request()?;

    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .or_bad_request()?;

    let size = persistor
        .write(&key, field.into_stream())
        .await
        .or_internal_server_error()?;

    let metadata_extraction_message = MetadataExtractionMessage {
        file_id: id.clone(),
        key: key.clone(),
        user_id: user.id,
        name: path
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .or_bad_request()?
            .to_string(),
        mime: FileFormat::from_extension(extension)
            .await
            .or_bad_request()?,
        size,
    };

    metadata_extraction
        .send(metadata_extraction_message)
        .await
        .or_internal_server_error()?;

    Ok((StatusCode::ACCEPTED, Json(
        CreateFileResponse {
            id
        }
    )))
}