use std::ops::Deref;

use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::stl_verification::StlVerificationChannelProducer;

pub struct StlVerificationExtractor(StlVerificationChannelProducer);

impl Deref for StlVerificationExtractor {
    type Target = StlVerificationChannelProducer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for StlVerificationExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for StlVerificationExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(Default::default())
    }
}