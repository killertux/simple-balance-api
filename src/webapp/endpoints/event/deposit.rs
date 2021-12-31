use super::{created_entity_response, Account, AccountId, Amount, DepositUseCase, WebApp};
use serde::Serialize;
use std::ops::Deref;

pub async fn handle(app: &WebApp, destination: AccountId, amount: Amount) -> tide::Result {
    DepositResponse::from(
        DepositUseCase::new(app.account_repository.deref())
            .execute(destination, amount)
            .await?,
    )
    .into()
}

#[derive(Serialize)]
struct DepositResponse {
    destination: Account,
}

impl From<Account> for DepositResponse {
    fn from(account: Account) -> Self {
        Self {
            destination: account,
        }
    }
}

impl From<DepositResponse> for tide::Result {
    fn from(response: DepositResponse) -> tide::Result {
        created_entity_response(&response)
    }
}
