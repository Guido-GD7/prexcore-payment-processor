use actix_web::{App, HttpServer, web};

use prexcore_payment_processor::{
    config::app_config::AppConfig,
    routes::configure as configure_routes,
    state::{app_state::AppState, in_memory_store::InMemoryStore},
    storage::file_storage::FileStorage,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let app_config = AppConfig::from_env();

    let host = app_config.host.clone();
    let port = app_config.port;

    let storage = FileStorage::new(app_config.data_file_path.clone());
    let in_memory_store = InMemoryStore::default();

    // Shared application state is wrapped in Actix `web::Data`,
    // allowing every HTTP worker to access the same in-memory store and storage layer safely.
    let app_state = web::Data::new(AppState::new(in_memory_store, storage, app_config));

    HttpServer::new(move || {
        App::new()
            // Each worker receives a clone of the shared state handle, not a copy of the data itself.
            .app_data(app_state.clone())
            .configure(configure_routes)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
