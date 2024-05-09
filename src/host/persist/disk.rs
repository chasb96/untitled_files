use std::{fs::File, io::{Read, Write}};
use bytes::Bytes;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use super::{error::ReadError, Persistor};

pub struct DiskDrive<'a> {
    root: &'a String,
}

impl<'a> DiskDrive<'a> {
    pub fn new(root: &'a String) -> Self {
        Self {
            root
        }
    }
}

impl<'a> Persistor for DiskDrive<'a> {
    async fn persist(&self, key: &str, bytes: Bytes) -> Result<(), super::error::PersistError> {
        let file = File::create(format!("{}/{}", self.root, key))?;
        let mut gzip_compressor = GzEncoder::new(file, Compression::default());

        gzip_compressor.write_all(&bytes)?;

        gzip_compressor.finish()?;

        Ok(())
    }
    
    async fn read(&self, key: &str) -> Result<Bytes, ReadError> {
        let file = File::open(format!("{}/{}", self.root, key))?;
        let mut gzip_decompressor = GzDecoder::new(file);

        let mut buf = Vec::new();

        gzip_decompressor.read_to_end(&mut buf)?;

        Ok(Bytes::from(buf))
    }
}