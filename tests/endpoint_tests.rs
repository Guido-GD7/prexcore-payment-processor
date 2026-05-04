use std::path::Path;

use actix_web::{App, http::StatusCode, test, web};
use rust_decimal::Decimal;
use tempfile::tempdir;

use prexcore_payment_processor::{
    config::app_config::AppConfig,
    errors::app_error::ErrorResponse,
    models::api::{
        ClientBalanceResponse, NewClientRequest, NewClientResponse, StoreBalancesResponse,
        TransactionRequest, TransactionResponse,
    },
    routes,
    state::{app_state::AppState, in_memory_store::InMemoryStore},
    storage::file_storage::FileStorage,
};

fn build_test_state(base_dir: &Path) -> web::Data<AppState> {
    let app_config = AppConfig {
        host: "127.0.0.1".to_string(),
        port: 0,
        data_file_path: base_dir.to_string_lossy().to_string(),
        max_negative_balance: Decimal::from(-100),
        worker_count: 4,
    };

    let store = InMemoryStore::default();
    let storage = FileStorage::new(app_config.data_file_path.clone());

    web::Data::new(AppState::new(store, storage, app_config))
}

#[actix_web::test]
async fn new_client_success_and_duplicate_failure() {
    // Scenario:
    // 1. Create a client successfully.
    // 2. Try to create another client with the same document number.
    // 3. Expect a conflict response.
    let dir = tempdir().expect("failed to create temp dir");
    let state = build_test_state(dir.path());

    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .configure(routes::configure),
    )
    .await;

    let payload = NewClientRequest {
        client_name: "Ada Lovelace".to_string(),
        birth_date: "1815-12-10".to_string(),
        document_number: "DOC-123".to_string(),
        country: "UK".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/api/new_client")
        .set_json(&payload)
        .to_request();

    let created: NewClientResponse = test::call_and_read_body_json(&app, req).await;

    assert_eq!(created.client_id, 1);

    let duplicate_payload = NewClientRequest {
        client_name: "Grace Hopper".to_string(),
        birth_date: "1906-12-09".to_string(),
        document_number: "DOC-123".to_string(),
        country: "US".to_string(),
    };

    let duplicate_req = test::TestRequest::post()
        .uri("/api/new_client")
        .set_json(&duplicate_payload)
        .to_request();

    let duplicate_resp = test::call_service(&app, duplicate_req).await;

    assert_eq!(duplicate_resp.status(), StatusCode::CONFLICT);

    let duplicate_error: ErrorResponse = test::read_body_json(duplicate_resp).await;

    assert_eq!(duplicate_error.code, "DUPLICATE_DOCUMENT_NUMBER");
}

#[actix_web::test]
async fn credit_and_debit_transactions_success() {
    // Scenario:
    // 1. Create a client.
    // 2. Credit the account.
    // 3. Debit the account.
    // 4. Verify the final balance.
    let dir = tempdir().expect("failed to create temp dir");
    let state = build_test_state(dir.path());

    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .configure(routes::configure),
    )
    .await;

    let payload = NewClientRequest {
        client_name: "Ada Lovelace".to_string(),
        birth_date: "1815-12-10".to_string(),
        document_number: "DOC-123".to_string(),
        country: "UK".to_string(),
    };

    let create_req = test::TestRequest::post()
        .uri("/api/new_client")
        .set_json(&payload)
        .to_request();

    let created: NewClientResponse = test::call_and_read_body_json(&app, create_req).await;

    let credit_req = TransactionRequest {
        client_id: created.client_id,
        amount: Decimal::from(50),
    };

    let credit_http_req = test::TestRequest::post()
        .uri("/api/new_credit_transaction")
        .set_json(&credit_req)
        .to_request();

    let credit_resp: TransactionResponse =
        test::call_and_read_body_json(&app, credit_http_req).await;

    assert_eq!(credit_resp.balance, Decimal::from(50));

    let debit_req = TransactionRequest {
        client_id: created.client_id,
        amount: Decimal::from(20),
    };

    let debit_http_req = test::TestRequest::post()
        .uri("/api/new_debit_transaction")
        .set_json(&debit_req)
        .to_request();

    let debit_resp: TransactionResponse = test::call_and_read_body_json(&app, debit_http_req).await;

    assert_eq!(debit_resp.balance, Decimal::from(30));
}

#[actix_web::test]
async fn transaction_failures_for_not_found_and_overdraft() {
    // Scenario:
    // 1. Try to process a transaction for a missing client.
    // 2. Create a valid client.
    // 3. Try to debit beyond the configured overdraft limit.
    // 4. Verify both error responses.
    let dir = tempdir().expect("failed to create temp dir");
    let state = build_test_state(dir.path());

    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .configure(routes::configure),
    )
    .await;

    let missing_client_req = TransactionRequest {
        client_id: 999,
        amount: Decimal::from(10),
    };

    let missing_client_http_req = test::TestRequest::post()
        .uri("/api/new_credit_transaction")
        .set_json(&missing_client_req)
        .to_request();

    let missing_client_resp = test::call_service(&app, missing_client_http_req).await;

    assert_eq!(missing_client_resp.status(), StatusCode::NOT_FOUND);

    let missing_client_error: ErrorResponse = test::read_body_json(missing_client_resp).await;

    assert_eq!(missing_client_error.code, "CLIENT_NOT_FOUND");

    let payload = NewClientRequest {
        client_name: "Ada Lovelace".to_string(),
        birth_date: "1815-12-10".to_string(),
        document_number: "DOC-OVERDRAFT".to_string(),
        country: "UK".to_string(),
    };

    let create_req = test::TestRequest::post()
        .uri("/api/new_client")
        .set_json(&payload)
        .to_request();

    let created: NewClientResponse = test::call_and_read_body_json(&app, create_req).await;

    let overdraft_req = TransactionRequest {
        client_id: created.client_id,
        amount: Decimal::from(200),
    };

    let overdraft_http_req = test::TestRequest::post()
        .uri("/api/new_debit_transaction")
        .set_json(&overdraft_req)
        .to_request();

    let overdraft_resp = test::call_service(&app, overdraft_http_req).await;

    assert_eq!(overdraft_resp.status(), StatusCode::BAD_REQUEST);

    let overdraft_error: ErrorResponse = test::read_body_json(overdraft_resp).await;

    assert_eq!(overdraft_error.code, "OVERDRAFT_LIMIT_EXCEEDED");
}

#[actix_web::test]
async fn client_balance_success_and_not_found_failure() {
    // Scenario:
    // 1. Create a client and credit balance.
    // 2. Retrieve the existing client balance successfully.
    // 3. Try to retrieve balance for a missing client.
    // 4. Expect a not found response.
    let dir = tempdir().expect("failed to create temp dir");
    let state = build_test_state(dir.path());

    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .configure(routes::configure),
    )
    .await;

    let payload = NewClientRequest {
        client_name: "Ada Lovelace".to_string(),
        birth_date: "1815-12-10".to_string(),
        document_number: "DOC-BALANCE".to_string(),
        country: "UK".to_string(),
    };

    let create_req = test::TestRequest::post()
        .uri("/api/new_client")
        .set_json(&payload)
        .to_request();

    let created: NewClientResponse = test::call_and_read_body_json(&app, create_req).await;

    let credit_req = TransactionRequest {
        client_id: created.client_id,
        amount: Decimal::from(75),
    };

    let credit_http_req = test::TestRequest::post()
        .uri("/api/new_credit_transaction")
        .set_json(&credit_req)
        .to_request();

    let _: TransactionResponse = test::call_and_read_body_json(&app, credit_http_req).await;

    let balance_req = test::TestRequest::get()
        .uri(&format!(
            "/api/client_balance?user_id={}",
            created.client_id
        ))
        .to_request();

    let balance_resp: ClientBalanceResponse =
        test::call_and_read_body_json(&app, balance_req).await;

    assert_eq!(balance_resp.client_name, "Ada Lovelace");
    assert_eq!(balance_resp.balance, Decimal::from(75));

    let missing_balance_req = test::TestRequest::get()
        .uri("/api/client_balance?user_id=999")
        .to_request();

    let missing_balance_resp = test::call_service(&app, missing_balance_req).await;

    assert_eq!(missing_balance_resp.status(), StatusCode::NOT_FOUND);

    let missing_balance_error: ErrorResponse = test::read_body_json(missing_balance_resp).await;

    assert_eq!(missing_balance_error.code, "CLIENT_NOT_FOUND");
}

#[actix_web::test]
async fn store_balances_writes_file_and_resets_balance() {
    // Scenario:
    // 1. Create a client and credit balance.
    // 2. Persist balances to a .DAT file.
    // 3. Verify the file was created.
    // 4. Verify in-memory balances are reset after persistence.
    let dir = tempdir().expect("failed to create temp dir");
    let state = build_test_state(dir.path());

    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .configure(routes::configure),
    )
    .await;

    let payload = NewClientRequest {
        client_name: "Ada Lovelace".to_string(),
        birth_date: "1815-12-10".to_string(),
        document_number: "DOC-STORE".to_string(),
        country: "UK".to_string(),
    };

    let create_req = test::TestRequest::post()
        .uri("/api/new_client")
        .set_json(&payload)
        .to_request();

    let created: NewClientResponse = test::call_and_read_body_json(&app, create_req).await;

    let credit_req = TransactionRequest {
        client_id: created.client_id,
        amount: Decimal::from(42),
    };

    let credit_http_req = test::TestRequest::post()
        .uri("/api/new_credit_transaction")
        .set_json(&credit_req)
        .to_request();

    let _: TransactionResponse = test::call_and_read_body_json(&app, credit_http_req).await;

    let store_req = test::TestRequest::post()
        .uri("/api/store_balances")
        .to_request();

    let store_resp: StoreBalancesResponse = test::call_and_read_body_json(&app, store_req).await;

    assert_eq!(store_resp.stored_clients, 1);
    assert!(store_resp.file_name.ends_with(".DAT"));

    let stored_file_path = dir.path().join(&store_resp.file_name);
    assert!(stored_file_path.exists());

    let balance_req = test::TestRequest::get()
        .uri(&format!(
            "/api/client_balance?user_id={}",
            created.client_id
        ))
        .to_request();

    let balance_resp: ClientBalanceResponse =
        test::call_and_read_body_json(&app, balance_req).await;

    assert_eq!(balance_resp.balance, Decimal::ZERO);
}
