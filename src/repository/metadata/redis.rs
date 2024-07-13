use prost::Message;
use redis::AsyncCommands;

use crate::repository::{error::QueryError, redis::RedisCache};

use super::{Metadata, MetadataRepository, NewMetadata};

pub struct MetadataCachingRepository<T> {
    redis: RedisCache,
    repository: T
}

impl<T> MetadataRepository for MetadataCachingRepository<T>
where 
    T: MetadataRepository,
{
    async fn create<'a>(&self, metadata: NewMetadata<'a>) -> Result<String, QueryError> {
        self.repository.create(metadata).await
    }

    async fn list(&self, keys: Vec<String>) -> Result<Vec<Metadata>, QueryError> {
        self.repository.list(keys).await
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Metadata>, QueryError> {
        let cache_key = format!("file_metadata:{}", id);

        let mut conn = self.redis
            .connection_pool
            .get()
            .await?;

        if let Some(bytes) = conn.get(&cache_key).await? {
            let bytes: Vec<u8> = bytes;

            return Ok(Some(Metadata::decode(bytes.as_slice())?))
        }

        if let Some(user) = self.repository.get_by_id(id).await? {
            let _: () = conn.set(cache_key, user.encode_to_vec()).await?;

            return Ok(Some(user))
        }

        Ok(None)
    }
}

impl<T> Default for MetadataCachingRepository<T>
where 
    T: Default
{
    fn default() -> Self {
        Self {
            redis: RedisCache::default(),
            repository: T::default()
        }
    }
}