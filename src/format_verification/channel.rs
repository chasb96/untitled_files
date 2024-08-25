use std::sync::OnceLock;
use log::error;
use async_channel::{Receiver, SendError, Sender};
use tokio::{spawn, task::JoinHandle};

use crate::{persist::PersistorOption, reference_counting::ReferenceCountingChannelProducer, repository::verification::VerificationRepositoryOption};

use super::message::Message;

static FORMAT_VERIFICATION_CHANNEL: OnceLock<FormatVerificationChannel> = OnceLock::new();

pub struct FormatVerificationChannel {
    #[allow(dead_code)]
    consumer: FormatVerificationChannelConsumer,
    producer: FormatVerificationChannelProducer,
}

impl FormatVerificationChannel {
    pub fn new() -> Self {
        let (sender, receiver) = async_channel::unbounded();

        Self {
            consumer: FormatVerificationChannelConsumer::new(receiver),
            producer: FormatVerificationChannelProducer::new(sender),
        }
    }

    pub fn producer(&self) -> FormatVerificationChannelProducer {
        self.producer.clone()
    }
}

#[allow(dead_code)]
pub struct FormatVerificationChannelConsumer(JoinHandle<()>);

impl FormatVerificationChannelConsumer {
    pub fn new(receiver: Receiver<Message>) -> Self {
        Self(spawn(async move {
            let verification_repository = VerificationRepositoryOption::default();
            let persistor = PersistorOption::default();
            let reference_counting_channel = ReferenceCountingChannelProducer::default();

            loop {
                match receiver.recv().await {
                    Ok(message) => if let Err(e) = message.handle(
                        &verification_repository, 
                        &persistor,
                        &reference_counting_channel,
                    ).await {
                        error!("Error handling message: {:?}", e);
                    },
                    Err(e) => error!("Error receiving message: {:?}", e),
                }
            }
        }))
    }
}

#[derive(Clone)]
pub struct FormatVerificationChannelProducer(Sender<Message>);

impl FormatVerificationChannelProducer {
    pub fn new(sender: Sender<Message>) -> Self {
        Self(sender)
    }

    pub async fn send(&self, message: impl Into<Message>) -> Result<(), SendError<Message>> {
        self.0
            .send(message.into())
            .await
    }
}

impl Default for FormatVerificationChannelProducer {
    fn default() -> Self {
        let format_verification_channel = FORMAT_VERIFICATION_CHANNEL.get_or_init(FormatVerificationChannel::new);

        format_verification_channel.producer()
    }
}