use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PersonInfo {
    pub discord_id: u64,
    pub balance: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentParams {
    pub to: Uuid,
    pub from: Uuid,
    pub amount: i64,
    pub force: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct PaymentParamsReferences<'a> {
    pub to: &'a Uuid,
    pub from: &'a Uuid,
    pub amount: &'a i64,
    pub force: Option<&'a bool>,
}
