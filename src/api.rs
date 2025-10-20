use crate::api_models::{ListPaymentsParams, SendTransactionPayload};
use crate::db::Payment;
use crate::state::AppState;
use crate::tx_sender;


use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get}, 
    Json, Router,
};
use chrono::{TimeZone, Utc};
use sqlx::{PgPool, Postgres, QueryBuilder};

async fn list_payments(
    State(app_state): State<AppState>, 
    Query(params): Query<ListPaymentsParams>,
) -> Json<Vec<Payment>> {
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT * FROM payments WHERE 1=1");

    if let Some(sender) = params.sender {
        query_builder.push(" AND sender = ");
        query_builder.push_bind(sender);
    }
    if let Some(recipient) = params.recipient {
        query_builder.push(" AND recipient = ");
        query_builder.push_bind(recipient);
    }
    if let Some(participant) = params.participant {
        query_builder.push(" AND (sender = ");
        query_builder.push_bind(participant.clone());
        query_builder.push(" OR recipient = ");
        query_builder.push_bind(participant);
        query_builder.push(")");
    }
    if let Some(ts) = params.created_after {
        if let Some(dt) = Utc.timestamp_opt(ts, 0).single() {
            query_builder.push(" AND \"timestamp\" > ");
            query_builder.push_bind(dt);
        }
    }
    if let Some(ts) = params.created_before {
        if let Some(dt) = Utc.timestamp_opt(ts, 0).single() {
            query_builder.push(" AND \"timestamp\" < ");
            query_builder.push_bind(dt);
        }
    }

    query_builder.push(" ORDER BY block_number DESC LIMIT 100");

    let payments = query_builder
        .build_query_as()
        .fetch_all(&app_state.pool) 
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to fetch payments: {}", e);
            vec![]
        });

    Json(payments)
}

async fn get_payment_by_id(
    State(app_state): State<AppState>, 
    Path(id): Path<i64>,
) -> Json<Option<Payment>> {
    let payment = sqlx::query_as("SELECT * FROM payments WHERE id = $1")
        .bind(id)
        .fetch_optional(&app_state.pool) 
        .await
        .unwrap_or(None);

    Json(payment)
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