use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::repository::{error::QueryError, postgres::PostgresDatabase};

use super::{VerificationRepository, VerificationState};

impl VerificationRepository for PostgresDatabase {
    async fn upsert(&self, file_id: &str, state: VerificationState) -> Result<(), QueryError> {
        const UPSERT_QUERY: &'static str = r#"
            INSERT INTO verification_states (file_id, state)
            VALUES ($1, $2)
            ON CONFLICT (file_id)
            DO UPDATE SET state = $2
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(UPSERT_QUERY)
            .bind(file_id)
            .bind(state as i16)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(Into::into)
    }

    async fn get_by_file_id(&self, file_id: &str) -> Result<Option<VerificationState>, QueryError> {
        const GET_BY_FILE_ID_QUERY: &'static str = r#"
            SELECT state
            FROM verification_states
            WHERE file_id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(GET_BY_FILE_ID_QUERY)
            .bind(file_id)
            .map(|row: PgRow| row.get("state"))
            .map(|state: i16| state.into())
            .fetch_optional(conn.as_mut())
            .await
            .map_err(Into::into)
    }
}