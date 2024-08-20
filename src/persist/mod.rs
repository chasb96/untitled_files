mod disk;
pub mod error;

use std::sync::OnceLock;

use axum::body::Bytes;
use error::DeleteError;
use futures::Stream;
use tokio::io::AsyncRead;

use self::{disk::DiskDrive, error::{WriteError, ReadError}};

static DRIVE: OnceLock<PersistorOption> = OnceLock::new();

pub trait Persistor {
    async fn write<T, S>(&self, key: &str, stream: T) -> Result<usize, WriteError<S>>
    where
        T: Stream<Item = Result<Bytes, S>> + Unpin,
        WriteError<S>: From<S>;

    async fn read(&self, key: String) -> Result<impl AsyncRead + Unpin, ReadError>;

    async fn delete(&self, key: &str) -> Result<(), DeleteError>;
}

pub enum PersistorOption {
    Disk(DiskDrive)
}

impl Persistor for PersistorOption {
    async fn write<T, S>(&self, key: &str, stream: T) -> Result<usize, WriteError<S>> 
    where
        T: Stream<Item = Result<Bytes, S>> + Unpin,
        WriteError<S>: From<S>,
    {
        match self {
            Self::Disk(d) => d.write(key, stream).await
        }
    }
    
    async fn read(&self, key: String) -> Result<impl AsyncRead + Unpin, ReadError> {
        match self {
            Self::Disk(d) => d.read(key).await,
        }
    }

    async fn delete(&self, key: &str) -> Result<(), DeleteError> {
        match self {
            Self::Disk(d) => d.delete(key).await
        }
    }
}

impl Default for PersistorOption {
    fn default() -> Self {
        PersistorOption::Disk(Default::default())
    }
}

impl Default for &'static PersistorOption {
    fn default() -> Self {
        DRIVE.get_or_init(PersistorOption::default)
    }
}