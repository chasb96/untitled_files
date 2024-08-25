use std::{error::Error, fmt::Display, io};

use async_channel::SendError;

use crate::{persist::error::ReadError, repository::error::QueryError};
use crate::reference_counting::Message as ReferenceCountingMessage;

#[derive(Debug)]
pub enum StlVerificationError {
    ReadError(ReadError),
    QueryError(QueryError),
    ReferenceCountingChannelError(SendError<ReferenceCountingMessage>)
}

impl Error for StlVerificationError { }

impl Display for StlVerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadError(e) => write!(f, "Error reading file: {}", e),
            Self::QueryError(e) => write!(f, "Error querying repository: {}", e),
            Self::ReferenceCountingChannelError(e) => write!(f, "Error sending message to reference counting channel: {}", e),
        }
    }
}

impl From<ReadError> for StlVerificationError {
    fn from(value: ReadError) -> Self {
        Self::ReadError(value)
    }
}

impl From<io::Error> for StlVerificationError {
    fn from(value: io::Error) -> Self {
        Self::ReadError(ReadError::IO(value))
    }
}

impl From<QueryError> for StlVerificationError {
    fn from(value: QueryError) -> Self {
        Self::QueryError(value)
    }
}

impl From<SendError<ReferenceCountingMessage>> for StlVerificationError {
    fn from(value: SendError<ReferenceCountingMessage>) -> Self {
        Self::ReferenceCountingChannelError(value)
    }
}