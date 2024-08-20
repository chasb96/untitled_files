use std::sync::OnceLock;

use log::error;
use async_channel::{Receiver, Sender};
use tokio::{spawn, task::JoinHandle};

use crate::repository::verification::VerificationRepositoryOption;

use super::message::Message;

static STL_VERIFICATION_CHANNEL: OnceLock<StlVerificationChannel> = OnceLock::new();

pub struct StlVerificationChannel {
    #[allow(dead_code)]
    consumer: StlVerificationChannelConsumer,
    producer: StlVerificationChannelProducer,
}

impl StlVerificationChannel {
    pub fn new() -> Self {
        let (sender, receiver) = async_channel::unbounded();

        Self {
            consumer: StlVerificationChannelConsumer::new(receiver),
            producer: StlVerificationChannelProducer::new(sender),
        }
    }

    pub fn producer(&self) -> StlVerificationChannelProducer {
        self.producer.clone()
    }
}

#[allow(dead_code)]
pub struct StlVerificationChannelConsumer(JoinHandle<()>);

impl StlVerificationChannelConsumer {
    pub fn new(receiver: Receiver<Message>) -> Self {
        Self(spawn(async move {
            let verification_repository = VerificationRepositoryOption::default();

            loop {
                match receiver.recv().await {
                    Ok(message) => if let Err(e) = message.handle(&verification_repository).await {
                        error!("Error handling message: {:?}", e);
                    },
                    Err(e) => error!("Error receiving message: {:?}", e),
                }
            }
        }))
    }
}

#[derive(Clone)]
pub struct StlVerificationChannelProducer(Sender<Message>);

impl StlVerificationChannelProducer {
    pub fn new(sender: Sender<Message>) -> Self {
        Self(sender)
    }

    pub async fn send(&self, message: impl Into<Message>) -> Result<(), ()> {
        self.0
            .send(message.into())
            .await
            .map_err(|_| ())
    }
}

impl Default for StlVerificationChannelProducer {
    fn default() -> Self {
        let stl_verification_channel = STL_VERIFICATION_CHANNEL.get_or_init(StlVerificationChannel::new);

        stl_verification_channel.producer()
    }
}