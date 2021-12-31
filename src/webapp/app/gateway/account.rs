use super::{
    AccountId, AccountRepository, AccountRepositoryError, AccountRepositoryResult, Balance,
};
use async_trait::async_trait;
use dashmap::DashMap;

pub struct DashmapAccountRepository {
    dashmap: DashMap<AccountId, Balance>,
}

impl DashmapAccountRepository {
    pub fn new() -> Self {
        Self {
            dashmap: DashMap::new(),
        }
    }
}

#[async_trait]
impl AccountRepository for DashmapAccountRepository {
    async fn store_balance(
        &self,
        account_id: &AccountId,
        new_balance: Balance,
        old_balance: Option<Balance>,
    ) -> AccountRepositoryResult<()> {
        let current_balance = self.dashmap.get_mut(account_id);
        let account_id = account_id.clone();
        match (current_balance, old_balance) {
            (Some(mut balance), Some(old_balance)) if old_balance == *balance => {
                *balance = new_balance;
                Ok(())
            }
            (None, None) => {
                self.dashmap.insert(account_id, new_balance);
                Ok(())
            }
            _ => Err(AccountRepositoryError::ErrorUpdatingAmount {
                account_id,
                new_balance,
                old_balance,
            }),
        }
    }

    async fn get_balance(&self, account_id: &AccountId) -> AccountRepositoryResult<Balance> {
        match self.dashmap.get(account_id) {
            Some(balance) => Ok(*balance),
            None => Err(AccountRepositoryError::AccountNotFound(account_id.clone())),
        }
    }

    async fn reset(&self) {
        self.dashmap.clear();
    }
}
