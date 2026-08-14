use std::{net::TcpListener, str};

use reqwest::{Client, Response};
use techsihir_newsletter::startup::run;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    let client = reqwest::Client::new();

    // Act

    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=Abdulbasit%20Ibrahim&email=abdulbasit%40techsihir.com";
    let response = send_post_request(body, client, &app_address).await;

    //Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    //Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Abdulbasit%20Ibrahim", "missing the email"),
        ("&email=abdulbasit%40techsihir.com", "missing the name"),
        ("", "missing both email and name"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = send_post_request(invalid_body, client.clone(), &app_address).await;

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
async fn send_post_request(req_body: &str, client: Client, address: &str) -> Response {
    client
        .post(format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(req_body.to_string())
        .send()
        .await
        .expect("Failed to execute request.")
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
