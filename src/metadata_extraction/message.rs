use crate::stl_verification::{Message as StlVerificationMessage, StlVerificationChannelProducer};
use crate::repository::metadata::{MetadataRepository, NewMetadata};
use crate::format_verification::{FormatVerificationChannelProducer, Message as FormatVerificationMessage};
use crate::file_format::FileFormat;

use super::error::MetadataExtractionError;

pub struct Message {
    pub file_id: String,
    pub key: String,
    pub user_id: String,
    pub name: String,
    pub mime: FileFormat,
    pub size: usize,
}

impl Message {
    pub async fn handle(
        self,
        metadata_repository: &impl MetadataRepository,
        format_verification_channel: &FormatVerificationChannelProducer,
        stl_verification_channel: &StlVerificationChannelProducer,
    ) -> Result<(), MetadataExtractionError> {
        let metadata = NewMetadata {
            id: &self.file_id,
            key: &self.key,
            user_id: &self.user_id,
            name: &self.name,
            mime: &self.mime.media_type(),
            size: self.size as i64,
        };

        metadata_repository
            .create(metadata)
            .await?;

        if self.mime == FileFormat::StereolithographyBinary {
            stl_verification_channel
                .send(StlVerificationMessage {
                    file_id: self.file_id,
                    key: self.key
                })
                .await?;
        } else {
            format_verification_channel
                .send(FormatVerificationMessage {
                    file_id: self.file_id,
                    key: self.key,
                    extension: self.mime
                        .extension()
                        .to_string(),
                })
                .await?;
        }

        Ok(())
    }
}