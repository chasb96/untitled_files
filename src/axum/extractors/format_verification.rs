use std::ops::Deref;

use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::format_verification::FormatVerificationChannelProducer;

pub struct FormatVerificationExtractor(FormatVerificationChannelProducer);

impl Deref for FormatVerificationExtractor {
    type Target = FormatVerificationChannelProducer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for FormatVerificationExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for FormatVerificationExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(Default::default())
    }
}