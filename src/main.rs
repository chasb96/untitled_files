use files::{self, router};

use std::{env, error::Error};
use axum::serve;
use log::info;
use tokio::net::TcpListener;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    let port = env::var("PORT").unwrap_or("80".to_string());

    let address = format!("0.0.0.0:{}", port);

    env_logger::init();

    info!("Binding to {}", address);
    let listener = TcpListener::bind(address).await?;

    info!("Serving traffic");
    serve(listener, router()).await?;

    Ok(())
}