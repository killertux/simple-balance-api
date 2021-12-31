use serde::Serialize;

pub type AccountId = String;
pub type Balance = i64;
pub type Amount = Balance;

#[derive(Serialize)]
pub struct Account {
    id: AccountId,
    balance: Balance,
}

impl Account {
    pub fn new(id: AccountId, balance: Balance) -> Self {
        Self { id, balance }
    }
}
