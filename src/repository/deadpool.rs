use deadpool::managed::{Manager, Metrics, RecycleError, RecycleResult};
use redis::{aio::MultiplexedConnection, Client, RedisError};
use sqlx::{PgConnection, Error, Connection};

pub struct PostgresConnectionManager {
    pub connection_string: String
}

impl Manager for PostgresConnectionManager {
    type Type = PgConnection;
    type Error = Error;
    
    async fn create(&self) -> Result<PgConnection, Self::Error> {
        PgConnection::connect(&self.connection_string).await
    }
    
    async fn recycle(&self, conn: &mut PgConnection, _: &Metrics) -> RecycleResult<Self::Error> {
        conn.ping()
            .await
            .map_err(|e| RecycleError::message(format!("Failed to ping postgres: {}", e)))
    }
}

pub struct RedisConnectionManager {
    pub client: Client,
}

impl Manager for RedisConnectionManager {
    type Type = MultiplexedConnection;
    type Error = RedisError;
    
    async fn create(&self) -> Result<MultiplexedConnection, Self::Error> {
        self.client
            .get_multiplexed_async_connection()
            .await
    }
    
    async fn recycle(&self, conn: &mut MultiplexedConnection, _: &Metrics) -> RecycleResult<Self::Error> {
        redis::cmd("PING")
            .query_async(conn)
            .await
            .map_err(|_| RecycleError::message("Failed to ping redis"))
    }
}