use serde::Deserialize;


#[derive(Debug, Deserialize)]       
pub struct ListPaymentsParams {
    pub sender: Option<String>,
    pub recipient: Option<String>,
    pub participant: Option<String>,
    pub created_after: Option<i64>,
    pub created_before: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SendTransactionPayload {
    pub recipient: String,
    pub amount: String,
}