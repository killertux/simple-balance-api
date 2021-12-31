use super::{AccountId, Balance};
use async_trait::async_trait;
use thiserror::Error;

pub type AccountRepositoryResult<T> = Result<T, AccountRepositoryError>;

#[derive(Error, Debug)]
pub enum AccountRepositoryError {
    #[error("Account wiht id `{0}` not found")]
    AccountNotFound(AccountId),
    #[error("Failed to update balance of account {account_id:?} from {old_balance:?} to {new_balance:?}")]
    ErrorUpdatingAmount {
        account_id: AccountId,
        new_balance: Balance,
        old_balance: Option<Balance>,
    },
}

#[async_trait]
pub trait AccountRepository {
    async fn store_balance(
        &self,
        account_id: &AccountId,
        new_balance: Balance,
        old_balance: Option<Balance>,
    ) -> AccountRepositoryResult<()>;

    async fn get_balance(&self, account_id: &AccountId) -> AccountRepositoryResult<Balance>;

    async fn reset(&self);
}
