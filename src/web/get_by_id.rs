use std::sync::OnceLock;

use axum::{extract::Path, http::{header::{CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE}, HeaderMap, HeaderValue, StatusCode}};
use axum_extra::body::AsyncReadBody;
use or_status_code::{OrInternalServerError, OrNotFound};

use crate::{axum::extractors::metadata_repository::MetadataRepositoryExtractor, persist::PersistorOption};
use crate::persist::Persistor;
use crate::repository::metadata::MetadataRepository;

static DRIVE: OnceLock<PersistorOption> = OnceLock::new();

pub async fn get_by_id(
    metadata_repository: MetadataRepositoryExtractor,
    Path(id): Path<String>
) -> Result<(HeaderMap, AsyncReadBody), StatusCode> {
    let metadata = metadata_repository
        .get_by_id(&id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    let content = DRIVE
        .get_or_init(PersistorOption::default)
        .read(metadata.key)
        .await
        .or_internal_server_error()?;
    
    let content_type = HeaderValue::from_str(&metadata.mime).or_internal_server_error()?;
    let content_disposition = HeaderValue::from_str(&format!("attachment; filename=\"{}\"", metadata.name)).or_internal_server_error()?;
    let content_length = HeaderValue::from_str(&metadata.size.to_string()).or_internal_server_error()?;

    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, content_type);
    headers.insert(CONTENT_DISPOSITION, content_disposition);
    headers.insert(CONTENT_LENGTH, content_length);

    Ok((headers, AsyncReadBody::new(content)))
}