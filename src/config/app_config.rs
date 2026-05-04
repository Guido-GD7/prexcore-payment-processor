use rust_decimal::Decimal;
use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub data_file_path: String,
    pub max_negative_balance: Decimal,
    pub worker_count: usize,
}

impl AppConfig {
    pub fn from_env() -> Self {
        // Load local environment variables from `.env` when available.
        dotenvy::dotenv().ok();

        let host: String = env::var("APP_HOST").expect("APP_HOST must be set");

        let port: u16 = env::var("APP_PORT")
            .expect("APP_PORT must be set")
            .parse::<u16>()
            .expect("APP_PORT must be a valid u16 number");

        let data_file_path: String =
            env::var("DATA_FILE_PATH").expect("DATA_FILE_PATH must be set");

        let max_negative_balance: Decimal = env::var("MAX_NEGATIVE_BALANCE")
            .expect("MAX_NEGATIVE_BALANCE must be set")
            .parse::<Decimal>()
            .expect("MAX_NEGATIVE_BALANCE must be a valid Decimal");

        let worker_count: usize = env::var("WORKER_COUNT")
            .expect("WORKER_COUNT must be set")
            .parse::<usize>()
            .expect("WORKER_COUNT must be a valid usize");

        Self {
            host,
            port,
            data_file_path: data_file_path.to_string(),
            max_negative_balance,
            worker_count,
        }
    }
}
