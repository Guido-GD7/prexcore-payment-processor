use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Client not found")]
    ClientNotFound,

    #[error("Document number already exists")]
    DuplicateDocumentNumber,

    #[error("Invalid client data: {0}")]
    InvalidClientData(String),

    #[error("Overdraft limit exceeded")]
    OverdraftLimitExceeded,

    #[error("Invalid amount. Amount must be greater than zero")]
    InvalidAmount,

    #[error("Invalid client id")]
    InvalidClientId,

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Internal server error")]
    InternalError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl AppError {
    fn code(&self) -> &'static str {
        match self {
            AppError::ClientNotFound => "CLIENT_NOT_FOUND",
            AppError::DuplicateDocumentNumber => "DUPLICATE_DOCUMENT_NUMBER",
            AppError::InvalidClientData(_) => "INVALID_CLIENT_DATA",
            AppError::OverdraftLimitExceeded => "OVERDRAFT_LIMIT_EXCEEDED",
            AppError::InvalidAmount => "INVALID_AMOUNT",
            AppError::InvalidClientId => "INVALID_CLIENT_ID",
            AppError::StorageError(_) => "STORAGE_ERROR",
            AppError::InternalError => "INTERNAL_ERROR",
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ClientNotFound => StatusCode::NOT_FOUND,

            AppError::DuplicateDocumentNumber => StatusCode::CONFLICT,

            AppError::InvalidClientData(_)
            | AppError::OverdraftLimitExceeded
            | AppError::InvalidAmount
            | AppError::InvalidClientId => StatusCode::BAD_REQUEST,

            AppError::StorageError(_) | AppError::InternalError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        // Convert domain/application errors into consistent JSON HTTP responses.
        let body = ErrorResponse {
            code: self.code().to_string(),
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(body)
    }
}
