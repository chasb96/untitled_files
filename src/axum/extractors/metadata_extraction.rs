use std::ops::Deref;

use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::metadata_extraction::MetadataExtractionChannelProducer;

pub struct MetadataExtractionExtractor(MetadataExtractionChannelProducer);

impl Deref for MetadataExtractionExtractor {
    type Target = MetadataExtractionChannelProducer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MetadataExtractionExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for MetadataExtractionExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(Default::default())
    }
}