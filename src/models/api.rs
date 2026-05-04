use crate::models::domain::ClientId;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewClientRequest {
    pub client_name: String,
    pub birth_date: String,
    pub document_number: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewClientResponse {
    pub client_id: ClientId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub client_id: ClientId,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub balance: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientBalanceQuery {
    pub user_id: ClientId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientBalanceResponse {
    pub client_name: String,
    pub birth_date: String,
    pub document_number: String,
    pub country: String,
    pub balance: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreBalancesResponse {
    pub file_name: String,
    pub stored_clients: u64,
}
