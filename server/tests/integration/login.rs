use crate::helpers::TestApi;
use app::login_payload::LoginPayload;
use reqwest::StatusCode;

#[tokio::test]
async fn authorize_valid_credentials() {
    let api = TestApi::spawn().await;
    let username = "Luca";
    let password = "1234";
    let credentials = LoginPayload {
        username: username.to_string(),
        pwd: password.to_string(),
    };

    let response = api
        .api_client
        .post(&format!("{}/api/login", &api.api_address))
        .json(&credentials)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.text().await.unwrap(), "Hello, Luca!");
}

#[tokio::test]
async fn reject_invalid_credentials() {
    let api = TestApi::spawn().await;
    let username = String::from("wrong");
    let pwd = String::from("0000");
    let credentials = LoginPayload { username, pwd };

    let response = api
        .api_client
        .post(&format!("{}/api/login", &api.api_address))
        .json(&credentials)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(response.text().await.unwrap(), "Invalid Credentials");
}

#[tokio::test]
async fn reject_missing_credentials() {
    let api = TestApi::spawn().await;
    let bad_body = serde_json::json!({
        "apple": "banana",
    });

    let response = api
        .api_client
        .post(&format!("{}/api/login", &api.api_address))
        .json(&bad_body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(response.text().await.unwrap(), "Invalid Credentials");
}