use std::{env, sync::OnceLock};

use deadpool::managed::Pool;
use log_unwrap::LogUnwrap;
use redis::Client;

use super::deadpool::RedisConnectionManager;

static REDIS_CLIENT: OnceLock<Pool<RedisConnectionManager>> = OnceLock::new();

pub struct RedisCache {
    pub connection_pool: &'static Pool<RedisConnectionManager>,
}

impl Default for RedisCache {
    fn default() -> Self {
        Self {
            connection_pool: REDIS_CLIENT
                .get_or_init(|| {
                    let base_url = env::var("REDIS_BASE_URL")
                        .unwrap_or("redis://redis-cache:6379".to_string())
                        .trim_end_matches('/')
                        .to_string();

                    let manager = RedisConnectionManager {
                        client: Client::open(base_url).log_unwrap(),
                    };

                    Pool::builder(manager)
                        .build()
                        .log_unwrap()
                })
        }
    }
}