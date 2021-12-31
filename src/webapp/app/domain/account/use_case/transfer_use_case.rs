use super::{Account, AccountId, AccountRepository, Amount};
use super::{DepositUseCase, WithdrawUseCase};
use anyhow::Result;

pub struct TransferUseCase<'a, T> {
    deposit_use_case: DepositUseCase<'a, T>,
    withdraw_use_case: WithdrawUseCase<'a, T>,
}

impl<'a, T> TransferUseCase<'a, T> {
    pub fn new(
        deposit_use_case: DepositUseCase<'a, T>,
        withdraw_use_case: WithdrawUseCase<'a, T>,
    ) -> Self {
        Self {
            deposit_use_case,
            withdraw_use_case,
        }
    }

    pub async fn execute(
        &self,
        origin_account_id: AccountId,
        destination_account_id: AccountId,
        amount: Amount,
    ) -> Result<(Account, Account)>
    where
        T: AccountRepository,
    {
        let withdraw_account = self
            .withdraw_use_case
            .execute(origin_account_id, amount)
            .await?;
        let deposit_account = self
            .deposit_use_case
            .execute(destination_account_id, amount)
            .await?;
        Ok((withdraw_account, deposit_account))
    }
}
