use crate::{
    errors::app_error::AppError,
    models::{
        api::{ClientBalanceResponse, StoreBalancesResponse},
        domain::{Client, ClientId},
    },
    state::app_state::AppContext,
};
use rust_decimal::Decimal;

pub async fn client_balance(
    context: &AppContext,
    user_id: ClientId,
) -> Result<ClientBalanceResponse, AppError> {
    // Read lock is enough because this operation only retrieves client data.
    let store = context.store.read().await;

    let client = store
        .clients
        .get(&user_id)
        .ok_or(AppError::ClientNotFound)?;

    Ok(ClientBalanceResponse {
        client_name: client.client_name.clone(),
        birth_date: client.birth_date.clone(),
        document_number: client.document_number.clone(),
        country: client.country.clone(),
        balance: client.balance,
    })
}

pub async fn store_balances(context: &AppContext) -> Result<StoreBalancesResponse, AppError> {
    // Write lock is required because this operation updates the file counter
    // and resets balances after persistence.
    let mut store = context.store.write().await;

    // Check if there are any clients to persist.
    // If not, return an error.
    if store.clients.is_empty() {
        return Err(AppError::StorageError(
            "Cannot store balances because there are no clients".to_string(),
        ));
    }

    store.file_counter += 1;

    // Create a snapshot before writing to disk.
    let clients_snapshot: Vec<Client> = store.clients.values().cloned().collect();
    let file_counter = store.file_counter;

    let file_name = context
        .storage
        .store_balances(clients_snapshot, file_counter)
        .await?;

    // Challenge-specific behavior: once balances are persisted,
    // in-memory balances are reset to zero.
    for client in store.clients.values_mut() {
        client.balance = Decimal::ZERO;
    }

    Ok(StoreBalancesResponse {
        file_name,
        stored_clients: store.clients.len() as u64,
    })
}
