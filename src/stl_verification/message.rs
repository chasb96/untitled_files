use crate::{persist::{Persistor, PersistorOption}, repository::verification::{VerificationRepository, VerificationState}};

use super::{error::StlVerificationError, stl};

pub struct Message {
    pub file_id: String,
    pub key: String,
}

impl Message {
    pub async fn handle(self, verification_repository: &impl VerificationRepository) -> Result<(), StlVerificationError> {
        let mut file = <&'static PersistorOption>::default()
            .read(self.key.clone())
            .await?;

        match stl::verify(&mut file).await {
            Ok(true) => Ok(verification_repository.upsert(&self.file_id, VerificationState::Accepted).await?),
            Ok(false) => Ok(verification_repository.upsert(&self.file_id, VerificationState::Rejected).await?),
            Err(_) => Ok(verification_repository.upsert(&self.file_id, VerificationState::Error).await?),
        }
    }
}