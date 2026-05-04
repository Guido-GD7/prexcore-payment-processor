use crate::handlers::client_handler;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/new_client", web::post().to(client_handler::new_client));
}
