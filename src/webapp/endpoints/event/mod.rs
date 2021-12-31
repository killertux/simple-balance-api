use super::{Account, AccountId, Amount, DepositUseCase, TransferUseCase, WebApp, WithdrawUseCase};
use serde::{Deserialize, Serialize};
use tide::Request;

mod deposit;
mod transfer;
mod withdraw;

pub async fn event(mut req: Request<WebApp>) -> tide::Result {
    let event: Event = req.body_json().await?;
    let app_state = req.state();
    event.handle(app_state).await
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Event {
    #[serde(alias = "deposit")]
    Deposit {
        destination: AccountId,
        amount: Amount,
    },
    #[serde(alias = "withdraw")]
    Withdraw { origin: AccountId, amount: Amount },
    #[serde(alias = "transfer")]
    Transfer {
        origin: AccountId,
        destination: AccountId,
        amount: Amount,
    },
}

impl Event {
    async fn handle(self, app: &WebApp) -> tide::Result {
        match self {
            Event::Deposit {
                destination,
                amount,
            } => deposit::handle(app, destination, amount).await,
            Event::Withdraw { origin, amount } => withdraw::handle(app, origin, amount).await,
            Event::Transfer {
                origin,
                destination,
                amount,
            } => transfer::handle(app, origin, destination, amount).await,
        }
    }
}

fn created_entity_response(entity: &impl Serialize) -> tide::Result {
    Ok(tide::Response::builder(tide::StatusCode::Created)
        .body(tide::Body::from_json(entity)?)
        .build())
}
