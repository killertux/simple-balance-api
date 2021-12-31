mod webapp;

use webapp::WebApp;

#[async_std::main]
async fn main() -> tide::Result<()> {
    WebApp::new().run().await
}
