use super::{ResetUseCase, WebApp};
use std::ops::Deref;
use tide::{Request, Result};

pub async fn reset(request: Request<WebApp>) -> Result {
    ResetUseCase::new(request.state().account_repository.deref())
        .execute()
        .await;
    Ok("OK".into())
}
