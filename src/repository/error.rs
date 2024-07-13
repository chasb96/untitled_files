use std::{error::Error, fmt::Display};
use deadpool::managed::PoolError;
use prost::DecodeError;
use redis::RedisError;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum QueryError {
    Sqlx(SqlxError),
    PostgresPool(PoolError<SqlxError>),
    RedisPool(PoolError<RedisError>),
    ProtobufDecode(DecodeError),
    Redis(RedisError),
}

impl Error for QueryError { }

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::Sqlx(e) => write!(f, "Error running query: {}", e),
            QueryError::PostgresPool(e) => write!(f, "Error obtaining connection from postgres pool: {}", e),
            QueryError::RedisPool(e) => write!(f, "Error obtaining connection from redis pool: {}", e),
            QueryError::ProtobufDecode(e) => write!(f, "Error decoding protobuf: {}", e),
            QueryError::Redis(e) => write!(f, "Error accessing cache: {}", e),
        }
    }
}

impl From<PoolError<SqlxError>> for QueryError {
    fn from(value: PoolError<SqlxError>) -> Self {
        QueryError::PostgresPool(value)
    }
}

impl From<PoolError<RedisError>> for QueryError {
    fn from(value: PoolError<RedisError>) -> Self {
        QueryError::RedisPool(value)
    }
}

impl From<SqlxError> for QueryError {
    fn from(value: SqlxError) -> Self {
        QueryError::Sqlx(value)
    }
}

impl From<DecodeError> for QueryError {
    fn from(value: DecodeError) -> Self {
        QueryError::ProtobufDecode(value)
    }
}

impl From<RedisError> for QueryError {
    fn from(value: RedisError) -> Self {
        QueryError::Redis(value)
    }
}