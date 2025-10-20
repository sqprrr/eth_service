use crate::api_models::{ListPaymentsParams, SendTransactionPayload};
use crate::db::Payment;
use crate::state::AppState;
use crate::tx_sender;
use crate::storage::Storage;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get}, 
    Json, Router,
};

async fn list_payments(
    State(app_state): State<AppState>, 
    Query(params): Query<ListPaymentsParams>,
) -> Json<Vec<Payment>> {
    match app_state.pool.list_payments(&params).await {
        Ok(payments) => Json(payments),
        Err(e) => {
            eprintln!("Failed to fetch payments: {}", e);
            Json(vec![])
        }
    }
}

async fn get_payment_by_id(
    State(app_state): State<AppState>, 
    Path(id): Path<i64>,
) -> Json<Option<Payment>> {
    match app_state.pool.get_payment_by_id(id).await {
        Ok(payment) => Json(payment),
        Err(e) => {
            eprintln!("Error fetching payment by id: {}", e);
            Json(None)
        }
    }
}

async fn send_transaction(
    State(app_state): State<AppState>,
    Json(payload): Json<SendTransactionPayload>,
) -> Result<Json<String>, StatusCode> {
    match tx_sender::send_usdc_transfer(
        &app_state.settings, 
        payload.recipient,
        payload.amount,
    )
    .await
    {
        Ok(tx_hash) => Ok(Json(tx_hash)),
        Err(e) => {
            eprintln!("Failed to send transaction: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn run_api_server(app_state: AppState) {
    let listener = tokio::net::TcpListener::bind(&app_state.settings.api_listen_address)
        .await
        .unwrap();
    println!(
        "API server listening on http://{}",
        listener.local_addr().unwrap()
    );

    let app = Router::new()
        .route("/payments", get(list_payments).post(send_transaction))
        .route("/payments/:id", get(get_payment_by_id))
        .with_state(app_state);

    axum::serve(listener, app).await.unwrap();
}