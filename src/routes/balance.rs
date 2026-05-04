use crate::handlers::balance_handler;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/client_balance",
        web::get().to(balance_handler::client_balance),
    )
    .route(
        "/store_balances",
        web::post().to(balance_handler::store_balances),
    );
}
