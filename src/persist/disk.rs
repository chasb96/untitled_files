use std::{fs::File, io::{Read, Write}};
use bytes::Bytes;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use crate::configuration::Configuration;

use super::{error::{PersistError, ReadError}, Persistor};

pub struct DiskDrive {
    root: &'static String,
}

impl Persistor for DiskDrive {
    async fn persist(&self, key: &str, bytes: Bytes) -> Result<(), PersistError> {
        let file = File::create(format!("{}/{}", &self.root, key))?;
        let mut gzip_compressor = GzEncoder::new(file, Compression::default());

        gzip_compressor.write_all(&bytes)?;

        gzip_compressor.finish()?;

        Ok(())
    }
    
    async fn read(&self, key: &str) -> Result<Bytes, ReadError> {
        let file = File::open(format!("{}/{}", &self.root, key))?;
        let mut gzip_decompressor = GzDecoder::new(file);

        let mut buf = Vec::new();

        gzip_decompressor.read_to_end(&mut buf)?;

        Ok(Bytes::from(buf))
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