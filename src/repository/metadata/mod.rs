mod postgres;
mod redis;

use prost::Message;
use sqlx::Row;
use sqlx::postgres::PgRow;
use super::{error::QueryError, postgres::PostgresDatabase};

#[derive(Message)]
pub struct Metadata {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub key: String,
    #[prost(string, tag = "3")]
    pub user_id: String,
    #[prost(string, tag = "4")]
    pub name: String,
    #[prost(string, tag = "5")]
    pub mime: String,
}

impl From<PgRow> for Metadata {
    fn from(row: PgRow) -> Self {
        Metadata {
            id: row.get("id"),
            key: row.get("key"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            mime: row.get("mime")
        }
    }
}

pub trait MetadataRepository {
    async fn create(&self, id: &str, key: &str, user_id: &str, name: &str, mime: &str) -> Result<String, QueryError>;

    async fn list(&self, keys: Vec<String>) -> Result<Vec<Metadata>, QueryError>;

    async fn get_by_id(&self, id: &str) -> Result<Option<Metadata>, QueryError>;
}

pub enum MetadataRepositoryOption {
    Postgres(PostgresDatabase)
}

impl MetadataRepository for MetadataRepositoryOption {
    async fn create(&self, id: &str, key: &str, user_id: &str, name: &str, mime: &str) -> Result<String, QueryError> {
        match self {
            Self::Postgres(pg) => pg.create(id, key, user_id, name, mime).await
        }
    }
    
    async fn list(&self, keys: Vec<String>) -> Result<Vec<Metadata>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.list(keys).await
        }
    }
    
    async fn get_by_id(&self, id: &str) -> Result<Option<Metadata>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_id(id).await
        }
    }
}

impl Default for MetadataRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}