use crate::persist::PersistorOption;
use crate::persist::Persistor;
use crate::reference_counting::ReferenceCountingChannelProducer;
use crate::repository::verification::{VerificationRepository, VerificationState};
use crate::reference_counting::Message as ReferenceCountingMessage;

use super::{error::StlVerificationError, stl};

pub struct Message {
    pub file_id: String,
    pub key: String,
}

impl Message {
    pub async fn handle(
        self, 
        verification_repository: &impl VerificationRepository,
        reference_counting_channel: &ReferenceCountingChannelProducer,
    ) -> Result<(), StlVerificationError> {
        let mut file = <&'static PersistorOption>::default()
            .read(self.key.clone())
            .await?;

        match stl::verify(&mut file).await {
            Ok(true) => verification_repository.upsert(&self.file_id, VerificationState::Accepted).await?,
            Ok(false) => verification_repository.upsert(&self.file_id, VerificationState::Rejected).await?,
            Err(_) => return Ok(verification_repository.upsert(&self.file_id, VerificationState::Error).await?),
        }

        reference_counting_channel
            .send(ReferenceCountingMessage {
                file_id: self.file_id,
            })
            .await
            .map_err(Into::into)
    }
}