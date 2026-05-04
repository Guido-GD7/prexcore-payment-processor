use crate::{
    errors::app_error::AppError,
    models::api::{ClientBalanceQuery, ClientBalanceResponse, StoreBalancesResponse},
    services::balance_service,
    state::app_state::AppState,
};
use actix_web::web;

pub async fn client_balance(
    state: web::Data<AppState>,
    payload: web::Query<ClientBalanceQuery>,
) -> Result<web::Json<ClientBalanceResponse>, AppError> {
    let result: ClientBalanceResponse =
        balance_service::client_balance(&state.context, payload.user_id).await?;

    Ok(web::Json(result))
}

pub async fn store_balances(
    state: web::Data<AppState>,
) -> Result<web::Json<StoreBalancesResponse>, AppError> {
    let result: StoreBalancesResponse = balance_service::store_balances(&state.context).await?;

    Ok(web::Json(result))
}
