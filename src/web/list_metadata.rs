use axum::{extract::Query, http::StatusCode, Json};
use or_status_code::OrInternalServerError;


use crate::axum::extractors::metadata_repository::MetadataRepositoryExtractor;
use crate::repository::metadata::MetadataRepository;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ListMetadataQuery {
    #[serde(rename = "i")]
    pub ids: Vec<String>,
}

#[derive(Serialize)]
pub struct MetadataResponse {
    pub id: String,
    pub name: String,
    pub user_id: String,
}

#[derive(Serialize)]
pub struct ListMetadataResponse {
    pub files: Vec<MetadataResponse>,
}

pub async fn list_metadata(
    metadata_repository: MetadataRepositoryExtractor,
    Query(request): Query<ListMetadataQuery>,
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
                .into_iter()
                .map(|metadata| MetadataResponse {
                    id: metadata.id,
                    name: metadata.name,
                    user_id: metadata.user_id,
                })
                .collect()
        }
    ))
}