use std::{error::Error, fmt::Display};

use async_channel::SendError;

use crate::{persist::error::ReadError, repository::error::QueryError};
use crate::reference_counting::Message as ReferenceCountingMessage;

#[derive(Debug)]
pub enum FormatVerificationError {
    ReadError(ReadError),
    QueryError(QueryError),
    ReferenceCountingChannelError(SendError<ReferenceCountingMessage>)
}

impl Error for FormatVerificationError { }

impl Display for FormatVerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadError(e) => write!(f, "Error reading file: {}", e),
            Self::QueryError(e) => write!(f, "Error querying repository: {}", e),
            Self::ReferenceCountingChannelError(e) => write!(f, "Error sending message to reference counting channel: {}", e),
        }
    }
}

impl From<ReadError> for FormatVerificationError {
    fn from(value: ReadError) -> Self {
        FormatVerificationError::ReadError(value)
    }
}

impl From<QueryError> for FormatVerificationError {
    fn from(value: QueryError) -> Self {
        FormatVerificationError::QueryError(value)
    }
}

impl From<SendError<ReferenceCountingMessage>> for FormatVerificationError {
    fn from(value: SendError<ReferenceCountingMessage>) -> Self {
        FormatVerificationError::ReferenceCountingChannelError(value)
    }
}