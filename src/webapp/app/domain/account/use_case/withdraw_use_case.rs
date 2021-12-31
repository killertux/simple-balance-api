use super::{Account, AccountId, AccountRepository, Amount};
use anyhow::{bail, Result};

pub struct WithdrawUseCase<'a, T> {
    repository: &'a T,
}

impl<'a, T> WithdrawUseCase<'a, T> {
    pub fn new(repository: &'a T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, account_id: AccountId, amount: Amount) -> Result<Account>
    where
        T: AccountRepository,
    {
        let current_balance = self.repository.get_balance(&account_id).await.ok();
        if current_balance.is_none() {
            bail!("Account not found with id {}", account_id);
        }        let new_balance = current_balance.unwrap_or(0) - amount;
        self.repository
            .store_balance(&account_id, new_balance, current_balance)
            .await?;
        Ok(Account::new(account_id, new_balance))
    }
}
