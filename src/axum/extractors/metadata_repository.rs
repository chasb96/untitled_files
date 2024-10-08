use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::repository::metadata::MetadataRepositoryOption;

pub struct MetadataRepositoryExtractor(&'static MetadataRepositoryOption);

impl Deref for MetadataRepositoryExtractor {
    type Target = MetadataRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MetadataRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for MetadataRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(Default::default())
    }
}