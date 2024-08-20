use async_compression::tokio::bufread::GzipDecoder;
use async_compression::tokio::write::GzipEncoder;
use bytes::Bytes;
use futures::Stream;
use futures::StreamExt;
use tokio::fs::File;
use tokio::io::{AsyncRead, BufReader};
use tokio::io::AsyncWriteExt; 

use crate::configuration::Configuration;

use super::{error::{DeleteError, ReadError, WriteError}, Persistor};

pub struct DiskDrive {
    root: &'static String,
}

impl Persistor for DiskDrive {
    async fn write<T, S>(&self, key: &str, stream: T) -> Result<usize, WriteError<S>> 
    where
        T: Stream<Item = Result<Bytes, S>> + Unpin,
        WriteError<S>: From<S>,
    {
        let file = File::create(format!("{}/{}", &self.root, key)).await?;
        let mut gzip_compressor = GzipEncoder::new(file);
        let mut written = 0;

        let mut chunks = stream.chunks(8192);

        while let Some(chunk) = chunks.next().await {
            let chunk = chunk
                .into_iter()
                .collect::<Result<Vec<Bytes>, S>>()?
                .concat();

            written += chunk.len();

            gzip_compressor.write_all(&chunk).await?;
        }

        gzip_compressor.flush().await?;
        gzip_compressor.shutdown().await?;

        Ok(written)
    }
    
    async fn read(&self, key: String) -> Result<impl AsyncRead + Unpin, ReadError> {
        let file = BufReader::new(File::open(format!("{}/{}", &self.root, key)).await?);
        let gzip_decompressor = GzipDecoder::new(file);

        Ok(gzip_decompressor)
    }

    async fn delete(&self, key: &str) -> Result<(), DeleteError> {
        std::fs::remove_file(format!("{}/{}", &self.root, key))?;

        Ok(())
    }
}

impl Default for DiskDrive {
    fn default() -> Self {
        let configuration = Configuration::configured();

        DiskDrive {
            root: &configuration.files_path
        }
    }
}