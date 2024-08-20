use std::{error::Error, fmt::Display};

use crate::{persist::error::ReadError, repository::error::QueryError};

#[derive(Debug)]
pub enum FormatVerificationError {
    ReadError(ReadError),
    QueryError(QueryError),
}

impl Error for FormatVerificationError { }

impl Display for FormatVerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadError(e) => write!(f, "Error reading file: {}", e),
            Self::QueryError(e) => write!(f, "Error querying repository: {}", e),
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