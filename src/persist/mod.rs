mod error;
mod disk;

use axum::body::Bytes;
use self::{disk::DiskDrive, error::{PersistError, ReadError}};

pub trait Persistor {
    async fn persist(&self, key: &str, bytes: Bytes) -> Result<(), PersistError>;

    async fn read(&self, key: &str) -> Result<Bytes, ReadError>;
}

pub enum PersistorOption {
    Disk(DiskDrive)
}

impl Persistor for PersistorOption {
    async fn persist(&self, key: &str, bytes: Bytes) -> Result<(), PersistError> {
        match self {
            Self::Disk(d) => d.persist(key, bytes).await
        }
    }
    
    async fn read(&self, key: &str) -> Result<Bytes, ReadError> {
        match self {
            Self::Disk(d) => d.read(key).await,
        }
    }
}

impl Default for PersistorOption {
    fn default() -> Self {
        PersistorOption::Disk(Default::default())
    }
}