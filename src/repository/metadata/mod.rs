mod postgres;
mod redis;

use prost::Message;
use redis::MetadataCachingRepository;
use sqlx::Row;
use sqlx::postgres::PgRow;
use super::{error::QueryError, postgres::PostgresDatabase};

pub struct NewMetadata<'a> {
    pub id: &'a str,
    pub key: &'a str,
    pub user_id: &'a str,
    pub name: &'a str,
    pub mime: &'a str,
}

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
    async fn create<'a>(&self, metadata: NewMetadata<'a>) -> Result<String, QueryError>;

    async fn list(&self, keys: Vec<String>) -> Result<Vec<Metadata>, QueryError>;

    async fn get_by_id(&self, id: &str) -> Result<Option<Metadata>, QueryError>;
}

#[allow(dead_code)]
pub enum MetadataRepositoryOption {
    Postgres(PostgresDatabase),
    CachedPostgres(MetadataCachingRepository<PostgresDatabase>),
}

impl MetadataRepository for MetadataRepositoryOption {
    async fn create<'a>(&self, metadata: NewMetadata<'a>) -> Result<String, QueryError> {
        match self {
            Self::Postgres(pg) => pg.create(metadata).await,
            Self::CachedPostgres(cached_pg) => cached_pg.create(metadata).await,
        }
    }
    
    async fn list(&self, keys: Vec<String>) -> Result<Vec<Metadata>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.list(keys).await,
            Self::CachedPostgres(cached_pg) => cached_pg.list(keys).await,
        }
    }
    
    async fn get_by_id(&self, id: &str) -> Result<Option<Metadata>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_id(id).await,
            Self::CachedPostgres(cached_pg) => cached_pg.get_by_id(id).await,
        }
    }
}

impl Default for MetadataRepositoryOption {
    fn default() -> Self {
        Self::CachedPostgres(Default::default())
    }
}