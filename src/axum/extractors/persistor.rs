use std::ops::Deref;

use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::persist::PersistorOption;

pub struct PersistorExtractor(PersistorOption);

impl Deref for PersistorExtractor {
    type Target = PersistorOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for PersistorExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for PersistorExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(PersistorExtractor::default())
    }
}