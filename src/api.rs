use crate::db::Payment;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sqlx::PgPool;

pub async fn get_all_payments(State(pool): State<PgPool>) -> Json<Vec<Payment>> {
    let payments = sqlx::query_as::<_, Payment>("SELECT * FROM payments ORDER BY id DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);
    Json(payments)
}
pub async fn get_payment_by_hash(
    State(pool): State<PgPool>,
    Path(tx_hash): Path<String>,
) -> Json<Option<Payment>> {
    let payment = sqlx::query_as::<_, Payment>("SELECT * FROM payments WHERE tx_hash = $1")
        .bind(tx_hash)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);
    Json(payment)
}
pub async fn get_payment_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Json<Option<Payment>> {
    let payment = sqlx::query_as::<_, Payment>("SELECT * FROM payments WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);
    Json(payment)
}

pub async fn run_api_server(pool: PgPool, listen_addr: String) {
    let app = Router::new()
        .route("/payments", get(get_all_payments))
        .route("/payments/hash/:tx_hash", get(get_payment_by_hash))
        .route("/payments/id/:id", get(get_payment_by_id))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();
    println!("API server listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}