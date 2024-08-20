mod postgres;

use std::sync::OnceLock;

use super::{error::QueryError, postgres::PostgresDatabase};

static VERIFICATION_REPOSITORY: OnceLock<VerificationRepositoryOption> = OnceLock::new();

#[repr(i16)]
pub enum VerificationState {
    Unknown = 0,
    Pending = 1,
    Accepted = 2,
    Rejected = 3,
    Error = 4,
}

impl From<i16> for VerificationState {
    fn from(state: i16) -> Self {
        match state {
            1 => Self::Pending,
            2 => Self::Accepted,
            3 => Self::Rejected,
            4 => Self::Error,
            _ => Self::Unknown,
        }
    }
}

pub trait VerificationRepository {
    async fn upsert(&self, file_id: &str, state: VerificationState) -> Result<(), QueryError>;

    async fn get_by_file_id(&self, file_id: &str) -> Result<Option<VerificationState>, QueryError>;
}

pub enum VerificationRepositoryOption {
    Postgres(PostgresDatabase),
}

impl VerificationRepository for VerificationRepositoryOption {
    async fn upsert(&self, file_id: &str, state: VerificationState) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.upsert(file_id, state).await,
        }
    }

    async fn get_by_file_id(&self, file_id: &str) -> Result<Option<VerificationState>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_file_id(file_id).await,
        }
    }
}

impl Default for VerificationRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}

impl Default for &'static VerificationRepositoryOption {
    fn default() -> Self {
        VERIFICATION_REPOSITORY.get_or_init(Default::default)
    }
}