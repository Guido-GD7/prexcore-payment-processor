use crate::{errors::app_error::AppError, models::domain::Client};
use chrono::Local;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug)]
pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    pub fn new(base_path: String) -> Self {
        Self {
            base_path: PathBuf::from(base_path),
        }
    }

    pub async fn store_balances(
        &self,
        clients: Vec<Client>,
        file_counter: u64,
    ) -> Result<String, AppError> {
        // Ensure the base directory exists before writing files.
        fs::create_dir_all(&self.base_path)
            .await
            .map_err(|err| AppError::StorageError(err.to_string()))?;

        // Generate file name using current date + incremental counter.
        let date = Local::now().format("%d%m%Y").to_string();
        let file_name = format!("{}_{}.DAT", date, file_counter);

        let file_path = self.base_path.join(&file_name);

        // Sort clients by ID to ensure deterministic output.
        let mut sorted_clients = clients;
        sorted_clients.sort_by_key(|client| client.id);

        let mut content = String::new();

        // Serialize each client as: "<id> <balance>"
        for client in sorted_clients {
            content.push_str(&format!("{} {}\n", client.id, client.balance));
        }

        // Write the full snapshot to disk asynchronously.
        fs::write(&file_path, content)
            .await
            .map_err(|err| AppError::StorageError(err.to_string()))?;

        Ok(file_name)
    }
}
