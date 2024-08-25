mod postgres;

use std::sync::OnceLock;

use super::{error::QueryError, postgres::PostgresDatabase};

static REFERENCE_COUNT_REPOSITORY: OnceLock<ReferenceCountRepositoryOption> = OnceLock::new();

pub struct NewReferenceCount<'a> {
    pub file_id: &'a str,
    pub expiry: i64,
}

pub trait ReferenceCountRepository {
    async fn create<'a>(&self, reference_count: NewReferenceCount<'a>) -> Result<(), QueryError>;

    async fn increment(&self, file_id: &str) -> Result<(), QueryError>;

    async fn decrement(&self, file_id: &str) -> Result<(), QueryError>;
}

#[allow(dead_code)]
pub enum ReferenceCountRepositoryOption {
    Postgres(PostgresDatabase),
}

impl ReferenceCountRepository for ReferenceCountRepositoryOption {
    async fn create<'a>(&self, reference_count: NewReferenceCount<'a>) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.create(reference_count).await,
        }
    }

    async fn increment(&self, file_id: &str) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.increment(file_id).await,
        }
    }

    async fn decrement(&self, file_id: &str) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.decrement(file_id).await,
        }
    }
}

impl Default for ReferenceCountRepositoryOption {
    fn default() -> Self {
        Self::Postgres(Default::default())
    }
}

impl Default for &'static ReferenceCountRepositoryOption {
    fn default() -> Self {
        REFERENCE_COUNT_REPOSITORY.get_or_init(|| ReferenceCountRepositoryOption::default())
    }
}