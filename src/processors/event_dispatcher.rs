use crate::{
    errors::app_error::AppError,
    models::domain::ClientId,
    processors::{event::PaymentEvent, event_handler::EventHandler},
};
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct EventDispatcher {
    senders: Vec<mpsc::Sender<PaymentEvent>>,
}

impl EventDispatcher {
    pub fn new(worker_count: usize, handler: Arc<dyn EventHandler<PaymentEvent>>) -> Self {
        let mut senders = Vec::new();

        for worker_id in 0..worker_count {
            // Each worker owns an independent channel.
            // The bounded capacity provides basic backpressure under high load.
            let (tx, mut rx) = mpsc::channel::<PaymentEvent>(100);
            let handler = handler.clone();

            tokio::spawn(async move {
                while let Some(event) = rx.recv().await {
                    println!("Worker {} received event", worker_id);
                    handler.handle(event).await;
                }
            });

            senders.push(tx);
        }

        Self { senders }
    }

    pub async fn dispatch(&self, client_id: ClientId, event: PaymentEvent) -> Result<(), AppError> {
        if self.senders.is_empty() {
            return Err(AppError::InternalError);
        }

        // Route events by client_id so all events for the same client
        // are handled by the same worker, preserving per-client ordering.
        let worker_index = client_id as usize % self.senders.len();

        self.senders
            .get(worker_index)
            .ok_or(AppError::InternalError)?
            .send(event)
            .await
            .map_err(|_| AppError::InternalError)
    }
}
