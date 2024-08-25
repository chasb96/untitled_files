use chrono::{Days, Utc};

use crate::repository::reference_counts::{NewReferenceCount, ReferenceCountRepository};

use super::error::ReferenceCountingError;

pub struct Message {
    pub file_id: String,
}

impl Message {
    pub async fn handle(
        self,
        reference_count_repository: &impl ReferenceCountRepository,
    ) -> Result<(), ReferenceCountingError> {
        reference_count_repository
            .create(NewReferenceCount {
                file_id: &self.file_id,
                expiry: Utc::now()
                    .checked_add_days(Days::new(1))
                    .unwrap()
                    .timestamp()
            })
            .await?;

        Ok(())
    }
}