use crate::{
    errors::app_error::AppError,
    models::{
        api::{NewClientRequest, NewClientResponse},
        domain::Client,
    },
    state::app_state::AppContext,
};
use rust_decimal::Decimal;

pub async fn create_client(
    context: &AppContext,
    request: NewClientRequest,
) -> Result<NewClientResponse, AppError> {
    validate_client_request(&request)?;

    // Write access is required because client creation mutates both
    // the primary client store and the document-number index.
    let mut store = context.store.write().await;

    // The document index enforces uniqueness without scanning all clients.
    if store.document_index.contains_key(&request.document_number) {
        return Err(AppError::DuplicateDocumentNumber);
    }

    let client_id = store.next_id();

    let client = Client {
        id: client_id,
        client_name: request.client_name,
        birth_date: request.birth_date,
        document_number: request.document_number.clone(),
        country: request.country,
        balance: Decimal::ZERO,
    };

    // Keep the secondary index and the primary store in sync.
    store
        .document_index
        .insert(client.document_number.clone(), client_id);

    store.clients.insert(client_id, client);

    Ok(NewClientResponse { client_id })
}

fn validate_client_request(request: &NewClientRequest) -> Result<(), AppError> {
    if request.client_name.trim().is_empty() {
        return Err(AppError::InvalidClientData(
            "client_name cannot be empty".to_string(),
        ));
    }

    if request.birth_date.trim().is_empty() {
        return Err(AppError::InvalidClientData(
            "birth_date cannot be empty".to_string(),
        ));
    }

    if request.document_number.trim().is_empty() {
        return Err(AppError::InvalidClientData(
            "document_number cannot be empty".to_string(),
        ));
    }

    if request.country.trim().is_empty() {
        return Err(AppError::InvalidClientData(
            "country cannot be empty".to_string(),
        ));
    }

    Ok(())
}
