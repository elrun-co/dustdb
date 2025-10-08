use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;

#[derive(Clone)]
pub struct PubSub {
    subs: Arc<Mutex<HashMap<String, Vec<tokio::sync::mpsc::UnboundedSender<String>>>>>,
}

impl PubSub {
    pub fn new() -> Self {
        Self {
            subs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn subscribe(&self, channel: &str) -> tokio::sync::mpsc::UnboundedReceiver<String> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        self.subs
            .lock()
            .entry(channel.to_string())
            .or_default()
            .push(tx);
        rx
    }

    pub fn publish(&self, channel: &str, msg: &str) {
        if let Some(list) = self.subs.lock().get_mut(channel) {
            list.retain(|tx| tx.send(msg.to_string()).is_ok());
        }
    }
}

