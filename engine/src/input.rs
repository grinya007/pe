use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Action {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "chargeback")]
    ChargeBack,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "type")]
    pub action: Action,
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    pub transaction_id: u32,
    pub amount: Option<f32>,
}
