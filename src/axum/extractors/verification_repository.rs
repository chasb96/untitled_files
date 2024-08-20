use std::ops::Deref;

use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::repository::verification::VerificationRepositoryOption;

pub struct VerificationRepositoryExtractor(&'static VerificationRepositoryOption);

impl Deref for VerificationRepositoryExtractor {
    type Target = VerificationRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for VerificationRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for VerificationRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(Default::default())
    }
}