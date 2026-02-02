//! Message bus for inter-service communication.

use tokio::sync::broadcast;

/// A message on the bus.
#[derive(Debug, Clone)]
pub struct Message {
    /// Message topic.
    pub topic: String,
    /// Message payload (JSON or other serialized data).
    pub payload: String,
}

impl Message {
    /// Creates a new message.
    pub fn new(topic: impl Into<String>, payload: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            payload: payload.into(),
        }
    }
}

/// A subscription to messages.
pub struct Subscription {
    receiver: broadcast::Receiver<Message>,
    topic_filter: Option<String>,
}

impl Subscription {
    /// Receives the next message matching the filter.
    pub async fn recv(&mut self) -> Option<Message> {
        loop {
            match self.receiver.recv().await {
                Ok(msg) => {
                    if self.topic_filter.as_ref().map_or(true, |t| msg.topic.starts_with(t)) {
                        return Some(msg);
                    }
                }
                Err(broadcast::error::RecvError::Closed) => return None,
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
            }
        }
    }
}

/// The central message bus.
#[derive(Clone)]
pub struct MessageBus {
    sender: broadcast::Sender<Message>,
}

impl MessageBus {
    /// Creates a new message bus.
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1024);
        Self { sender }
    }

    /// Publishes a message.
    pub fn publish(&self, message: Message) {
        let _ = self.sender.send(message);
    }

    /// Subscribes to all messages.
    pub fn subscribe(&self) -> Subscription {
        Subscription {
            receiver: self.sender.subscribe(),
            topic_filter: None,
        }
    }

    /// Subscribes to messages with a topic prefix.
    pub fn subscribe_topic(&self, topic_prefix: impl Into<String>) -> Subscription {
        Subscription {
            receiver: self.sender.subscribe(),
            topic_filter: Some(topic_prefix.into()),
        }
    }
}

impl Default for MessageBus {
    fn default() -> Self {
        Self::new()
    }
}
