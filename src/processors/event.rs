use rust_decimal::Decimal;
use tokio::sync::oneshot;

use crate::{
    errors::app_error::AppError,
    models::{api::TransactionResponse, domain::ClientId},
};

pub enum PaymentEvent {
    CreditRequested {
        client_id: ClientId,
        amount: Decimal,

        // One-shot channel used to send the processing result
        // back to the HTTP handler that originated the request.
        response_tx: oneshot::Sender<Result<TransactionResponse, AppError>>,
    },
    DebitRequested {
        client_id: ClientId,
        amount: Decimal,

        // Same request-response bridge used for debit operations.
        response_tx: oneshot::Sender<Result<TransactionResponse, AppError>>,
    },
}
