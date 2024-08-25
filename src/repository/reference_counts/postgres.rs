use crate::repository::{error::QueryError, postgres::PostgresDatabase};

use super::{NewReferenceCount, ReferenceCountRepository};

impl ReferenceCountRepository for PostgresDatabase {
    async fn create<'a>(&self, reference_count: NewReferenceCount<'a>) -> Result<(), QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO reference_counts (id, count, expiry)
            VALUES ($1, 0, $2)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(reference_count.file_id)
            .bind(reference_count.expiry)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(Into::into)
    }

    async fn increment(&self, id: &str) -> Result<(), QueryError> {
        const INCREMENT_QUERY: &'static str = r#"
            UPDATE reference_counts
            SET count = count + 1
            WHERE id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INCREMENT_QUERY)
            .bind(id)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(Into::into)
    }

    async fn decrement(&self, id: &str) -> Result<(), QueryError> {
        const DECREMENT_QUERY: &'static str = r#"
            UPDATE reference_counts
            SET count = count - 1
            WHERE id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(DECREMENT_QUERY)
            .bind(id)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(Into::into)
    }
}