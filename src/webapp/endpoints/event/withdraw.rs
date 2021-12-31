use super::{created_entity_response, Account, AccountId, Amount, WebApp, WithdrawUseCase};
use serde::Serialize;
use std::ops::Deref;

pub async fn handle(app: &WebApp, origin: AccountId, amount: Amount) -> tide::Result {
    WithdrawResponse::from(
        WithdrawUseCase::new(app.account_repository.deref())
            .execute(origin, amount)
            .await?,
    )
    .try_into()?
}

#[derive(Serialize)]
struct WithdrawResponse {
    origin: Account,
}

impl From<Account> for WithdrawResponse {
    fn from(account: Account) -> Self {
        Self { origin: account }
    }
}

impl From<WithdrawResponse> for tide::Result {
    fn from(response: WithdrawResponse) -> tide::Result {
        created_entity_response(&response)
    }
}
