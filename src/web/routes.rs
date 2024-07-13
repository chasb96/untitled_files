use std::{collections::HashMap, io::Cursor, path::PathBuf};
use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::{Multipart, Path, Request}, http::{header::{CONTENT_DISPOSITION, CONTENT_TYPE}, HeaderMap, HeaderValue, StatusCode}, response::IntoResponse, Json, RequestExt};
use bytes::Bytes;
use file_format::FileFormat;
use or_status_code::{OrInternalServerError, OrNotFound, OrBadRequest};
use rand::distributions::{Alphanumeric, DistString};

use super::{request::ListMetadataRequest, response::{CreateFileResponse, ListMetadataResponse}};

use crate::{axum::extractors::{metadata_repository::MetadataRepositoryExtractor, persistor::PersistorExtractor}, repository::metadata::NewMetadata, web::response::MetadataResponse};
use crate::persist::Persistor;
use crate::repository::metadata::MetadataRepository;

pub async fn post_files(
    authenticate_extractor: Authenticate<ClaimsUser>,
    persistor: PersistorExtractor,
    metadata_repository: MetadataRepositoryExtractor,
    request: Request
) -> impl IntoResponse {
    let content_type = request
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|content_type| content_type.to_str().ok())
        .or_bad_request()?;

    if content_type == "application/json" {
        let json = request
            .extract()
            .await
            .or_internal_server_error()?;
    
        list_metadata(metadata_repository, json)
            .await
            .map(|json| json.into_response())
    } else if content_type.starts_with("multipart/form-data") {
        let multipart = request
            .extract()
            .await
            .or_internal_server_error()?;

        create_file(authenticate_extractor, persistor, metadata_repository, multipart)
            .await
            .map(|json| json.into_response())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn create_file(
    Authenticate(user): Authenticate<ClaimsUser>,
    persistor: PersistorExtractor,
    metadata_repository: MetadataRepositoryExtractor,
    mut request: Multipart
) -> Result<Json<CreateFileResponse>, StatusCode> {
    const UPLOAD_CAP: usize = 16;
    const FILE_FORMAT_WHITELIST: &[FileFormat] = &[
        FileFormat::StereolithographyAscii,
        FileFormat::StereolithographyBinary,
        FileFormat::PortableNetworkGraphics,
        FileFormat::PlainText,
        FileFormat::PortableDocumentFormat,
        FileFormat::JointPhotographicExpertsGroup,
    ];

    let mut ids = HashMap::new();

    while let Some(field) = request.next_field().await.or_internal_server_error()? {
        let name = field.name()
            .map(|name| PathBuf::from(name))
            .or_bad_request()?
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .or_bad_request()?
            .to_string();
        
        let bytes = field.bytes().await.or_bad_request()?;
    
        let mut file_format = FileFormat::from_bytes(&bytes);

        if !FILE_FORMAT_WHITELIST.contains(&file_format) {
            // STLB does not have reliable magic bytes, try reading the file if
            //  it is stated to be an STLB file
            let extension = PathBuf::from(&name)
                .extension()
                .or_bad_request()?
                .to_ascii_uppercase();

            if extension == "STL" && stl_io::read_stl(&mut Cursor::new(&bytes)).is_ok() {
                file_format = FileFormat::StereolithographyBinary;
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }

        let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let key = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        persistor
            .persist(&key, bytes)
            .await
            .or_internal_server_error()?;

        let metadata = NewMetadata {
            id: &id,
            key: &key,
            user_id: &user.id,
            name: &name,
            mime: file_format.media_type(),
        };

        metadata_repository
            .create(metadata)
            .await
            .or_internal_server_error()?;

        ids.insert(name, id.to_string());

        if ids.len() >= UPLOAD_CAP {
            break;
        }
    }

    Ok(Json(
        CreateFileResponse {
            ids
        }
    ))
}

pub async fn list_metadata(
    metadata_repository: MetadataRepositoryExtractor,
    Json(request): Json<ListMetadataRequest>
) -> Result<Json<ListMetadataResponse>, StatusCode> {
    const LIST_CAP: usize = 256;

    if request.ids.len() > LIST_CAP {
        return Err(StatusCode::BAD_REQUEST);
    }

    let metadata = metadata_repository
        .list(request.ids)
        .await
        .or_internal_server_error()?;

    Ok(Json(
        ListMetadataResponse {
            files: metadata
                .iter()
                .map(|metadata| MetadataResponse {
                    id: metadata.id.to_owned(),
                    name: metadata.name.to_owned(),
                    user_id: metadata.user_id
                        .to_string()
                        .clone(),
                })
                .collect()
        }
    ))
}

pub async fn get_by_id(
    persistor: PersistorExtractor,
    metadata_repository: MetadataRepositoryExtractor,
    Path(id): Path<String>
) -> Result<(HeaderMap, Bytes), StatusCode> {
    let metadata = metadata_repository
        .get_by_id(&id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    let content = persistor
        .read(&metadata.key)
        .await
        .or_internal_server_error()?;
    
    let content_type = HeaderValue::from_str(&metadata.mime).or_internal_server_error()?;
    let content_disposition = HeaderValue::from_str(&format!("attachment; filename=\"{}\"", metadata.name)).or_internal_server_error()?;

    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, content_type);
    headers.insert(CONTENT_DISPOSITION, content_disposition);

    Ok((headers, content))
}