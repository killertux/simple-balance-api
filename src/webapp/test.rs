use super::*;
use tide::http::StatusCode;
use tide::prelude::*;
use tide_testing::surf::Response;
use tide_testing::TideTestingExt;

#[async_std::test]
async fn run_test() {
    let server = WebApp::new().build_server_with_routes();
    //Reset state before starting tests
    assert_raw(server.post("/reset").await.unwrap(), "OK", StatusCode::Ok).await;
    //Get balance for non-existing account
    let result = server.get("/balance?account_id=1234").await.unwrap();
    assert_not_found(result).await;
    //Create account with initial balance
    let result = server
        .post("/event")
        .body(json!({
            "type": "deposit",
            "destination": "100",
            "amount": 10
        }))
        .await
        .unwrap();
    assert_json(
        result,
        json!({"destination": {"id": "100", "balance": 10}}),
        StatusCode::Created,
    )
    .await;
    //Deposit into existing account
    let result = server
        .post("/event")
        .body(json!({
            "type": "deposit",
            "destination": "100",
            "amount": 10
        }))
        .await
        .unwrap();
    assert_json(
        result,
        json!({"destination": {"id": "100", "balance": 20}}),
        StatusCode::Created,
    )
    .await;
    //Get balance for existing account
    let result = server.get("/balance?account_id=100").await.unwrap();
    assert_raw(result, "20", StatusCode::Ok).await;
    //Withdraw from non-existing account
    let result = server
        .post("/event")
        .body(json!({
            "type": "withdraw",
            "origin": "200",
            "amount": 10
        }))
        .await
        .unwrap();
    assert_not_found(result).await;

    //Withdraw from existing account
    let result = server
        .post("/event")
        .body(json!({
            "type": "withdraw",
            "origin": "100",
            "amount": 5
        }))
        .await
        .unwrap();
    assert_json(
        result,
        json!({"origin": {"id": "100", "balance": 15}}),
        StatusCode::Created,
    )
    .await;
    //Transfer from existing account
    let result = server
        .post("/event")
        .body(json!({
            "type": "transfer",
            "origin": "100",
            "amount": 15,
            "destination": "300",
        }))
        .await
        .unwrap();
    assert_json(
        result,
        json!({"origin": {"id": "100", "balance": 0}, "destination": {"id": "300", "balance": 15}}),
        StatusCode::Created,
    )
    .await;
    //Transfer from non-existing account
    let result = server
        .post("/event")
        .body(json!({
            "type": "transfer",
            "origin": "200",
            "amount": 15,
            "destination": "300",
        }))
        .await
        .unwrap();
    assert_not_found(result).await;
}

async fn assert_raw(mut result: Response, body: &str, status: StatusCode) {
    assert_eq!(body, result.body_string().await.unwrap());
    assert_eq!(status, result.status());
}

async fn assert_not_found(mut result: Response) {
    assert_eq!("0", result.body_string().await.unwrap());
    assert_eq!(StatusCode::NotFound, result.status());
}

async fn assert_json(mut result: Response, body: serde_json::Value, status: StatusCode) {
    let rbody: serde_json::Value = result.body_json().await.unwrap();
    assert_eq!(body, rbody);
    assert_eq!(status, result.status());
}
