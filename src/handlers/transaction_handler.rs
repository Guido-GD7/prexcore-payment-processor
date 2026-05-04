use actix_web::web;
use tokio::sync::oneshot;

use crate::{
    errors::app_error::AppError,
    models::api::{TransactionRequest, TransactionResponse},
    processors::event::PaymentEvent,
    state::app_state::AppState,
};

pub async fn process_credit_transaction_worker(
    state: web::Data<AppState>,
    payload: web::Json<TransactionRequest>,
) -> Result<web::Json<TransactionResponse>, AppError> {
    let request = payload.into_inner();

    let (response_tx, response_rx) = oneshot::channel::<Result<TransactionResponse, AppError>>();

    let event = PaymentEvent::CreditRequested {
        client_id: request.client_id,
        amount: request.amount,
        response_tx,
    };

    state
        .transaction_dispatcher
        .dispatch(request.client_id, event)
        .await?;

    let result = response_rx.await.map_err(|_| AppError::InternalError)??;

    Ok(web::Json(result))
}

pub async fn process_debit_transaction_worker(
    state: web::Data<AppState>,
    payload: web::Json<TransactionRequest>,
) -> Result<web::Json<TransactionResponse>, AppError> {
    let request = payload.into_inner();

    let (response_tx, response_rx) = oneshot::channel::<Result<TransactionResponse, AppError>>();

    let event = PaymentEvent::DebitRequested {
        client_id: request.client_id,
        amount: request.amount,
        response_tx,
    };

    state
        .transaction_dispatcher
        .dispatch(request.client_id, event)
        .await?;

    let result = response_rx.await.map_err(|_| AppError::InternalError)??;

    Ok(web::Json(result))
}
