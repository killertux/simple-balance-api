use app::domain::{
    Account, AccountId, Amount, DepositUseCase, GetBalanceUseCase, ResetUseCase, TransferUseCase,
    WithdrawUseCase,
};
use app::gateway::DashmapAccountRepository;
use std::sync::Arc;

mod app;
mod endpoints;
#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct WebApp {
    pub account_repository: Arc<DashmapAccountRepository>,
}

impl WebApp {
    pub fn new() -> Self {
        Self {
            account_repository: Arc::new(DashmapAccountRepository::new()),
        }
    }

    pub async fn run(self) -> tide::Result<()> {
        self.build_server_with_routes()
            .listen("127.0.0.1:3030")
            .await?;
        Ok(())
    }

    fn build_server_with_routes(self) -> tide::Server<WebApp> {
        let server = tide::with_state(self);
        set_up_routes(server)
    }
}

fn set_up_routes(mut server: tide::Server<WebApp>) -> tide::Server<WebApp> {
    server.at("/balance").get(endpoints::balance);
    server.at("/reset").post(endpoints::reset);
    server.at("/event").post(endpoints::event);
    server.with(tide::utils::After(error_handler));
    server
}

async fn error_handler(mut res: tide::Response) -> tide::Result {
    if let Some(_) = res.take_error() {
        res.set_status(tide::StatusCode::NotFound);
        res.set_body("0");
    }
    Ok(res)
}
