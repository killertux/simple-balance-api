use super::{AccountId, GetBalanceUseCase, WebApp};
use serde::Deserialize;
use std::ops::Deref;
use tide::{Request, Result};

pub async fn balance(req: Request<WebApp>) -> Result {
    let app = req.state();
    let query_account_id: QueryAccountId = req.query()?;
    Ok(GetBalanceUseCase::new(app.account_repository.deref())
        .execute(query_account_id.account_id)
        .await?
        .to_string()
        .into())
}

#[derive(Deserialize)]
struct QueryAccountId {
    pub account_id: AccountId,
}
