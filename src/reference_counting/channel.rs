use std::sync::OnceLock;

use log::error;
use async_channel::{Receiver, SendError, Sender};
use tokio::{spawn, task::JoinHandle};

use crate::repository::reference_counts::ReferenceCountRepositoryOption;

use super::Message;

static REFERENCE_COUNTING_CHANNEL: OnceLock<ReferenceCountingChannel> = OnceLock::new();

pub struct ReferenceCountingChannel {
    #[allow(dead_code)]
    consumer: ReferenceCountingChannelConsumer,
    producer: ReferenceCountingChannelProducer,
}

impl ReferenceCountingChannel {
    pub fn new() -> Self {
        let (sender, receiver) = async_channel::unbounded();

        Self {
            consumer: ReferenceCountingChannelConsumer::new(receiver),
            producer: ReferenceCountingChannelProducer::new(sender),
        }
    }

    pub fn producer(&self) -> ReferenceCountingChannelProducer {
        self.producer.clone()
    }
}

#[allow(dead_code)]
pub struct ReferenceCountingChannelConsumer(JoinHandle<()>);

impl ReferenceCountingChannelConsumer {
    pub fn new(receiver: Receiver<Message>) -> Self {
        Self(spawn(async move {
            let reference_count_repository = ReferenceCountRepositoryOption::default();

            loop {
                match receiver.recv().await {
                    Ok(message) => if let Err(e) = message.handle(&reference_count_repository).await {
                        error!("Error handling message: {:?}", e);
                    },
                    Err(e) => error!("Error receiving message: {:?}", e),
                }
            }
        }))
    }
}

#[derive(Clone)]
pub struct ReferenceCountingChannelProducer(Sender<Message>);

impl ReferenceCountingChannelProducer {
    pub fn new(sender: Sender<Message>) -> Self {
        Self(sender)
    }

    pub async fn send(&self, message: Message) -> Result<(), SendError<Message>> {
        self.0
            .send(message)
            .await
    }
}

impl Default for ReferenceCountingChannelProducer {
    fn default() -> Self {
        REFERENCE_COUNTING_CHANNEL.get_or_init(|| ReferenceCountingChannel::new()).producer()
    }
}