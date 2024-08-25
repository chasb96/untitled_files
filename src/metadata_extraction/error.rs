use std::{error::Error, fmt::Display};

use async_channel::SendError;

use crate::stl_verification::Message as StlVerificationMessage;
use crate::repository::error::QueryError;
use crate::format_verification::Message as FormatVerificationMessage;

#[derive(Debug)]
pub enum MetadataExtractionError {
    QueryError(QueryError),
    FormatVerificationChannelError(SendError<FormatVerificationMessage>),
    StlVerificationChannelError(SendError<StlVerificationMessage>),
}

impl Error for MetadataExtractionError { }

impl Display for MetadataExtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryError(e) => write!(f, "Error querying repository: {}", e),
            Self::FormatVerificationChannelError(e) => write!(f, "Error sending message to format verification channel: {}", e),
            Self::StlVerificationChannelError(e) => write!(f, "Error sending message to stl verification channel: {}", e),
        }
    }
}

impl From<QueryError> for MetadataExtractionError {
    fn from(value: QueryError) -> Self {
        Self::QueryError(value)
    }
}

impl From<SendError<FormatVerificationMessage>> for MetadataExtractionError {
    fn from(value: SendError<FormatVerificationMessage>) -> Self {
        Self::FormatVerificationChannelError(value)
    }
}

impl From<SendError<StlVerificationMessage>> for MetadataExtractionError {
    fn from(value: SendError<StlVerificationMessage>) -> Self {
        Self::StlVerificationChannelError(value)
    }
}