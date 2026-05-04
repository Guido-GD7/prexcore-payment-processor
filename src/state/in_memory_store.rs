use crate::models::domain::{Client, ClientId};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct InMemoryStore {
    // Primary storage: client_id -> Client
    pub clients: HashMap<ClientId, Client>,

    // Secondary index: document_number -> client_id
    pub document_index: HashMap<String, ClientId>,

    // Auto-incrementing ID generator
    pub next_client_id: ClientId,

    // Counter used for generating unique persistence file names
    pub file_counter: u64,
}

impl InMemoryStore {
    pub fn next_id(&mut self) -> ClientId {
        // Simple monotonic ID generator (not safe for distributed systems,
        // but sufficient for in-memory single-instance context)
        self.next_client_id += 1;
        self.next_client_id
    }
}
