use sqlx::Row;
use sqlx::postgres::PgRow;
use crate::repository::{error::QueryError, postgres::PostgresDatabase};
use super::{Metadata, MetadataRepository, NewMetadata};

impl MetadataRepository for PostgresDatabase {
    async fn create<'a>(&self, metadata: NewMetadata<'a>) -> Result<String, QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO metadata (id, key, user_id, name, mime, size)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(metadata.id)
            .bind(metadata.key)
            .bind(metadata.user_id)
            .bind(metadata.name)
            .bind(metadata.mime)
            .bind(metadata.size)
            .map(|row: PgRow| row.get("id"))
            .fetch_one(conn.as_mut())
            .await
            .map_err(Into::into)
    }
    
    async fn list(&self, ids: Vec<String>) -> Result<Vec<Metadata>, QueryError> {
        const LIST_QUERY: &'static str = r#"
            SELECT
                id,
                key,
                user_id,
                name,
                mime,
                size
            FROM
                metadata
            WHERE
                id IN (
                    SELECT id
                    FROM UNNEST($1) AS ids(id)
                )
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(LIST_QUERY)
            .bind(ids)
            .map(Into::into)
            .fetch_all(conn.as_mut())
            .await
            .map_err(Into::into)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Metadata>, QueryError> {
        const GET_BY_ID_QUERY: &'static str = r#"
            SELECT
                id,
                key,
                user_id,
                name,
                mime,
                size
            FROM
                metadata
            WHERE
                id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(GET_BY_ID_QUERY)
            .bind(id)
            .map(Into::into)
            .fetch_optional(conn.as_mut())
            .await
            .map_err(Into::into)
    }
}