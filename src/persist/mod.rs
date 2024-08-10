mod error;
mod disk;

use axum::body::Bytes;
use error::DeleteError;
use file_format::FileFormat;
use futures::Stream;
use tokio::io::AsyncRead;

use self::{disk::DiskDrive, error::{WriteError, ReadError}};

pub trait Persistor {
    async fn write<T, S>(&self, key: &str, stream: T) -> Result<usize, WriteError<S>>
    where
        T: Stream<Item = Result<Bytes, S>> + Unpin,
        WriteError<S>: From<S>;

    async fn read(&self, key: String) -> Result<impl AsyncRead, ReadError>;

    async fn mime(&self, key: &str) -> Result<Option<FileFormat>, ReadError>;

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
    
    async fn read(&self, key: String) -> Result<impl AsyncRead, ReadError> {
        match self {
            Self::Disk(d) => d.read(key).await,
        }
    }

    async fn mime(&self, key: &str) -> Result<Option<FileFormat>, ReadError> {
        match self {
            Self::Disk(d) => d.mime(key).await
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