use crate::{
    config::app_config::AppConfig,
    processors::{event_dispatcher::EventDispatcher, transaction_processor::TransactionProcessor},
    state::in_memory_store::InMemoryStore,
    storage::file_storage::FileStorage,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppContext {
    // Shared in-memory store protected by RwLock for concurrent access
    pub store: Arc<RwLock<InMemoryStore>>,

    // Storage abstraction (file-based in current implementation)
    pub storage: Arc<FileStorage>,

    // Application configuration shared across components
    pub app_config: Arc<AppConfig>,
}

#[derive(Clone)]
pub struct AppState {
    pub context: AppContext,

    // Dispatcher responsible for routing events to worker processors
    pub transaction_dispatcher: Arc<EventDispatcher>,
}

impl AppState {
    pub fn new(store: InMemoryStore, storage: FileStorage, app_config: AppConfig) -> Self {
        let context = AppContext {
            store: Arc::new(RwLock::new(store)),
            storage: Arc::new(storage),
            app_config: Arc::new(app_config.clone()),
        };

        let transaction_processor = Arc::new(TransactionProcessor::new(context.clone()));

        // Worker count is configurable via environment.
        // This allows tuning concurrency without changing code.
        let transaction_dispatcher = Arc::new(EventDispatcher::new(
            app_config.worker_count,
            transaction_processor,
        ));

        Self {
            context,
            transaction_dispatcher,
        }
    }
}
