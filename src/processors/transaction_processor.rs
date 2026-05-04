use async_trait::async_trait;

use crate::{
    models::api::TransactionRequest,
    processors::{event::PaymentEvent, event_handler::EventHandler},
    services::transaction_service,
    state::app_state::AppContext,
};

pub struct TransactionProcessor {
    // TransactionProcessor is a worker component responsible for handling
    // transaction-related events. It decouples HTTP request handling from
    // business logic execution via an event-driven pipeline.
    context: AppContext,
}

impl TransactionProcessor {
    pub fn new(context: AppContext) -> Self {
        Self { context }
    }
}

#[async_trait]
impl EventHandler<PaymentEvent> for TransactionProcessor {
    async fn handle(&self, event: PaymentEvent) {
        // The processor acts as a worker that consumes events emitted by the dispatcher.
        // Each event is translated into a service call and processed asynchronously.
        match event {
            PaymentEvent::CreditRequested {
                client_id,
                amount,
                response_tx,
            } => {
                let request = TransactionRequest { client_id, amount };

                // Execute business logic through the service layer
                let result = transaction_service::apply_credit(&self.context, request).await;

                // Send the result back to the caller via oneshot channel.
                // Ignoring send errors since the receiver may have been dropped.
                let _ = response_tx.send(result);
            }

            PaymentEvent::DebitRequested {
                client_id,
                amount,
                response_tx,
            } => {
                let request = TransactionRequest { client_id, amount };
                let result = transaction_service::apply_debit(&self.context, request).await;
                let _ = response_tx.send(result);
            }
        }
    }
}
