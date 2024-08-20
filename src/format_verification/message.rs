use crate::{file_format::FileFormat, persist::Persistor, repository::verification::{VerificationRepository, VerificationState}};

use super::error::FormatVerificationError;

pub struct Message {
    pub file_id: String,
    pub key: String,
    pub extension: String,
}

impl Message {
    pub async fn handle(
        self, 
        verification_repository: &impl VerificationRepository,
        persistor: &impl Persistor,
    ) -> Result<(), FormatVerificationError> {
        let mut file = persistor
            .read(self.key.clone())
            .await?;

        let extension_format = match FileFormat::from_extension(&self.extension).await {
            Some(FileFormat::Markdown) => return Ok(verification_repository.upsert(&self.file_id, VerificationState::Accepted).await?),
            Some(FileFormat::PlainText) => return Ok(verification_repository.upsert(&self.file_id, VerificationState::Accepted).await?),
            Some(extension_format) => extension_format,
            None => return Ok(verification_repository.upsert(&self.file_id, VerificationState::Rejected).await?),
        };

        let bytes_format = match FileFormat::from_read_magic_bytes(&mut file).await {
            Ok(Some(bytes_format)) => bytes_format,
            Ok(None) => return Ok(verification_repository.upsert(&self.file_id, VerificationState::Rejected).await?),
            Err(_) => return Ok(verification_repository.upsert(&self.file_id, VerificationState::Error).await?),
        };

        if extension_format != bytes_format {
            return Ok(verification_repository.upsert(&self.file_id, VerificationState::Rejected).await?);
        }

        Ok(verification_repository.upsert(&self.file_id, VerificationState::Accepted).await?)
    }
}