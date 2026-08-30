use crate::events::DomainEvent;
use std::sync::{Arc, Mutex};

/// Trait implemented by event listeners/handlers in the domain or application layer.
pub trait DomainEventHandler: Send + Sync {
    fn handle(&self, event: &DomainEvent);
}

pub type EventHandlerRef = Arc<dyn DomainEventHandler>;

/// Thread-safe in-memory pub/sub Domain Event Bus.
#[derive(Clone, Default)]
pub struct DomainEventBus {
    handlers: Arc<Mutex<Vec<EventHandlerRef>>>,
}

impl DomainEventBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register a new event handler to listen for domain events.
    pub fn subscribe(&self, handler: EventHandlerRef) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(handler);
    }

    /// Publish a single domain event to all subscribed handlers.
    pub fn publish(&self, event: &DomainEvent) {
        let handlers = self.handlers.lock().unwrap().clone();
        for handler in handlers {
            handler.handle(event);
        }
    }

    /// Publish a batch of domain events.
    pub fn publish_all(&self, events: &[DomainEvent]) {
        for event in events {
            self.publish(event);
        }
    }

    /// Return current total number of registered subscribers.
    pub fn handler_count(&self) -> usize {
        self.handlers.lock().unwrap().len()
    }
}
