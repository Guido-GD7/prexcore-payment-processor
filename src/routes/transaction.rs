use crate::handlers::transaction_handler;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/new_credit_transaction",
        web::post().to(transaction_handler::process_credit_transaction_worker),
    )
    .route(
        "/new_debit_transaction",
        web::post().to(transaction_handler::process_debit_transaction_worker),
    );
}
