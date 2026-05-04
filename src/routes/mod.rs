pub mod balance;
pub mod client;
pub mod transaction;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(client::configure)
            .configure(transaction::configure)
            .configure(balance::configure),
    );
}
