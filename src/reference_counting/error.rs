use std::{error::Error, fmt::Display};

use crate::repository::error::QueryError;

#[derive(Debug)]
pub enum ReferenceCountingError {
    QueryError(QueryError),
}

impl Error for ReferenceCountingError { }

impl Display for ReferenceCountingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryError(e) => write!(f, "Error querying repository: {}", e),
        }
    }
}

impl From<QueryError> for ReferenceCountingError {
    fn from(value: QueryError) -> Self {
        Self::QueryError(value)
    }
}