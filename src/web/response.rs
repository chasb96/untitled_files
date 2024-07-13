use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct CreateFileResponse {
    pub ids: HashMap<String, String>,
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