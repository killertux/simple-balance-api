use super::{
    created_entity_response, Account, AccountId, Amount, DepositUseCase, TransferUseCase, WebApp,
    WithdrawUseCase,
};
use serde::Serialize;
use std::ops::Deref;

pub async fn handle(
    app: &WebApp,
    origin: AccountId,
    destination: AccountId,
    amount: Amount,
) -> tide::Result {
    let repository = app.account_repository.deref();
    TransferResponse::from(
        TransferUseCase::new(
            DepositUseCase::new(repository),
            WithdrawUseCase::new(repository),
        )
        .execute(origin, destination, amount)
        .await?,
    )
    .try_into()?
}

#[derive(Serialize)]
struct TransferResponse {
    origin: Account,
    destination: Account,
}

impl From<(Account, Account)> for TransferResponse {
    fn from((origin, destination): (Account, Account)) -> Self {
        Self {
            origin,
            destination,
        }
    }
}

impl From<TransferResponse> for tide::Result {
    fn from(response: TransferResponse) -> tide::Result {
        created_entity_response(&response)
    }
}
