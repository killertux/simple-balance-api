use super::{AccountId, AccountRepository, Amount};
use anyhow::{bail, Result};

pub struct GetBalanceUseCase<'a, T> {
    repository: &'a T,
}

impl<'a, T> GetBalanceUseCase<'a, T> {
    pub fn new(repository: &'a T) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, account_id: AccountId) -> Result<Amount>
    where
        T: AccountRepository,
    {
        if let Ok(balance) = self.repository.get_balance(&account_id).await {
            return Ok(balance);
        }
        bail!("Account not found with id {}", account_id)
    }
}
