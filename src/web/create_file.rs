use std::path::PathBuf;
use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Multipart, http::StatusCode, Json};
use futures::TryStreamExt;
use or_status_code::{OrInternalServerError, OrBadRequest};
use rand::distributions::{Alphanumeric, DistString};
use serde::Serialize;

use crate::{axum::extractors::{metadata_repository::MetadataRepositoryExtractor, persistor::PersistorExtractor}, repository::metadata::NewMetadata, FILE_FORMAT_WHITELIST};
use crate::persist::Persistor;
use crate::repository::metadata::MetadataRepository;

#[derive(Serialize)]
pub struct CreateFileResponse {
    #[serde(rename = "i")]
    pub id: String,
}

pub async fn create_file(
    Authenticate(user): Authenticate<ClaimsUser>,
    persistor: PersistorExtractor,
    metadata_repository: MetadataRepositoryExtractor,
    mut request: Multipart
) -> Result<Json<CreateFileResponse>, StatusCode> {
    let field = request
        .next_field()
        .await
        .or_internal_server_error()?
        .or_bad_request()?;

    let name = field.name()
        .map(PathBuf::from)
        .or_bad_request()?
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .or_bad_request()?
        .to_string();
    
    let bytes = field.into_stream();

    let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let key = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let size = persistor
        .write(&key, bytes)
        .await
        .or_internal_server_error()?;

    let mime = persistor
        .mime(&key)
        .await
        .or_internal_server_error()?
        .and_then(|mime| if FILE_FORMAT_WHITELIST.contains(&mime) { Some(mime) } else { None });

    if mime.is_none() {
        persistor
            .delete(&key)
            .await
            .or_internal_server_error()?;

        return Err(StatusCode::BAD_REQUEST);
    }

    let mime = mime.unwrap();

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

    Ok(Json(
        CreateFileResponse {
            id
        }
    ))
}