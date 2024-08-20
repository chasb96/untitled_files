use std::path::PathBuf;
use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Multipart, http::StatusCode, Json};
use futures::TryStreamExt;
use or_status_code::{OrInternalServerError, OrBadRequest};
use rand::distributions::{Alphanumeric, DistString};
use serde::Serialize;

use crate::repository::metadata::NewMetadata;
use crate::file_format::FileFormat;
use crate::axum::extractors::stl_verification::StlVerificationExtractor;
use crate::axum::extractors::persistor::PersistorExtractor;
use crate::axum::extractors::metadata_repository::MetadataRepositoryExtractor;
use crate::axum::extractors::format_verification::FormatVerificationExtractor;
use crate::persist::Persistor;
use crate::repository::metadata::MetadataRepository;
use crate::stl_verification::Message as StlVerificationMessage;
use crate::format_verification::Message as FormatVerificationMessage;

#[derive(Serialize)]
pub struct CreateFileResponse {
    #[serde(rename = "i")]
    pub id: String,
}

pub async fn create_file(
    Authenticate(user): Authenticate<ClaimsUser>,
    format_verification: FormatVerificationExtractor,
    stl_verification: StlVerificationExtractor,
    persistor: PersistorExtractor,
    metadata_repository: MetadataRepositoryExtractor,
    mut request: Multipart
) -> Result<Json<CreateFileResponse>, StatusCode> {
    let field = request
        .next_field()
        .await
        .or_internal_server_error()?
        .or_bad_request()?;

    let path = field
        .name()
        .map(PathBuf::from)
        .or_bad_request()?;

    let name = path
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .or_bad_request()?;

    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .or_bad_request()?;

    let mime = FileFormat::from_extension(extension)
        .await
        .or_bad_request()?;
    
    let bytes = field.into_stream();

    let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let key = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let size = persistor
        .write(&key, bytes)
        .await
        .or_internal_server_error()?;

    let metadata = NewMetadata {
        id: &id,
        key: &key,
        user_id: &user.id,
        name: &name,
        mime: &mime.media_type(),
        size: size as i64,
    };

    metadata_repository
        .create(metadata)
        .await
        .or_internal_server_error()?;

    if mime == FileFormat::StereolithographyBinary {
        stl_verification
            .send(StlVerificationMessage {
                file_id: id.clone(),
                key: key.clone()
            })
            .await
            .or_internal_server_error()?;
    } else {
        format_verification
            .send(FormatVerificationMessage {
                file_id: id.clone(),
                key: key.clone(),
                extension: extension.to_string()
            })
            .await
            .or_internal_server_error()?;
    }

    Ok(Json(
        CreateFileResponse {
            id
        }
    ))
}