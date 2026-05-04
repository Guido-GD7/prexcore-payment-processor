use crate::{
    errors::app_error::AppError,
    models::api::{TransactionRequest, TransactionResponse},
    state::app_state::AppContext,
};
use rust_decimal::Decimal;

pub async fn apply_credit(
    context: &AppContext,
    request: TransactionRequest,
) -> Result<TransactionResponse, AppError> {
    validate_amount(request.amount)?;

    // Acquire a write lock because credit operations mutate the shared in-memory state.
    let mut store = context.store.write().await;

    let client = store
        .clients
        .get_mut(&request.client_id)
        .ok_or(AppError::ClientNotFound)?;

    client.balance += request.amount;

    Ok(TransactionResponse {
        balance: client.balance,
    })
}

pub async fn apply_debit(
    context: &AppContext,
    request: TransactionRequest,
) -> Result<TransactionResponse, AppError> {
    validate_amount(request.amount)?;

    // Debit operations require exclusive access to ensure the balance check
    // and balance update happen atomically within the same lock scope.
    let mut store = context.store.write().await;

    let client = store
        .clients
        .get_mut(&request.client_id)
        .ok_or(AppError::ClientNotFound)?;

    if client.balance <= context.app_config.max_negative_balance {
        return Err(AppError::OverdraftLimitExceeded);
    }

    let new_balance = client.balance - request.amount;

    if new_balance < context.app_config.max_negative_balance {
        return Err(AppError::OverdraftLimitExceeded);
    }

    client.balance = new_balance;

    Ok(TransactionResponse {
        balance: client.balance,
    })
}

fn validate_amount(amount: Decimal) -> Result<(), AppError> {
    if amount <= Decimal::ZERO {
        return Err(AppError::InvalidAmount);
    }

    Ok(())
}
