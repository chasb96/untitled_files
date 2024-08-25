use std::sync::OnceLock;

use log::error;
use async_channel::{Receiver, Sender};
use tokio::{spawn, task::JoinHandle};

use crate::{format_verification::FormatVerificationChannelProducer, repository::metadata::MetadataRepositoryOption, stl_verification::StlVerificationChannelProducer};

use super::Message;

static METADATA_EXTRACTION_CHANNEL: OnceLock<MetadataExtractionChannel> = OnceLock::new();

pub struct MetadataExtractionChannel {
    #[allow(dead_code)]
    consumer: MetadataExtractionChannelConsumer,
    producer: MetadataExtractionChannelProducer,
}

impl MetadataExtractionChannel {
    pub fn new() -> Self {
        let (sender, receiver) = async_channel::unbounded();

        Self {
            consumer: MetadataExtractionChannelConsumer::new(receiver),
            producer: MetadataExtractionChannelProducer::new(sender),
        }
    }

    pub fn producer(&self) -> MetadataExtractionChannelProducer {
        self.producer.clone()
    }
}

#[allow(dead_code)]
pub struct MetadataExtractionChannelConsumer(JoinHandle<()>);

impl MetadataExtractionChannelConsumer {
    pub fn new(receiver: Receiver<Message>) -> Self {
        Self(spawn(async move {
            let metadata_repository = MetadataRepositoryOption::default();
            let format_verification_producer = FormatVerificationChannelProducer::default();
            let stl_verification_producer = StlVerificationChannelProducer::default();

            loop {
                match receiver.recv().await {
                    Ok(message) => if let Err(e) = message.handle(
                        &metadata_repository, 
                        &format_verification_producer, 
                        &stl_verification_producer
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
pub struct MetadataExtractionChannelProducer(Sender<Message>);

impl MetadataExtractionChannelProducer {
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

impl Default for MetadataExtractionChannelProducer {
    fn default() -> Self {
        let metadata_extraction_channel = METADATA_EXTRACTION_CHANNEL.get_or_init(MetadataExtractionChannel::new);
        
        metadata_extraction_channel.producer()
    }
}