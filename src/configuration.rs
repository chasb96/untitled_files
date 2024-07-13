use std::{env, sync::OnceLock};
use log_unwrap::LogUnwrap;
use serde::Deserialize;

static CONFIGURATION: OnceLock<Configuration> = OnceLock::new();

#[derive(Deserialize)]
pub struct Configuration {
    pub database_url: String,
    pub files_path: String,
}

impl Configuration {
    pub fn configured() -> &'static Self {
        let config = CONFIGURATION
            .get_or_init(|| {
                let database_url = env::var("FILES_DATABASE_URL").log_unwrap();
                let files_path = env::var("FILES_PATH").log_unwrap();

                Configuration {
                    database_url,
                    files_path,
                }
            });

        &config
    }
}