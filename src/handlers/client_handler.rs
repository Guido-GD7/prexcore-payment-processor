use crate::{
    errors::app_error::AppError,
    models::api::{NewClientRequest, NewClientResponse},
    services::client_service,
    state::app_state::AppState,
};
use actix_web::web;

pub async fn new_client(
    state: web::Data<AppState>,
    payload: web::Json<NewClientRequest>,
) -> Result<web::Json<NewClientResponse>, AppError> {
    let result: NewClientResponse =
        client_service::create_client(&state.context, payload.into_inner()).await?;

    Ok(web::Json(result))
}
