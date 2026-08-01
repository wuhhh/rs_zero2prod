//! tests/health_check.rs

// `tokio::test` os the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspet what code gets generated using
// `cargo expand --test health_check` <- name of the test file

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind address.");
    let local_addr = listener.local_addr().expect("Could not get local address.");

    // Arrange
    spawn_app(listener);
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("http://{}/health-check", local_addr))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app(listener: TcpListener) {
    let server = zero2prod::run(listener).expect("Failed to bind address.");
    // Launch the server as a background task.
    // tokio::spawn returns a JoinHandle to the spawned future, but we have no
    // use for it here. Dropping the handle detaches the task (it keeps running
    // rather than being aborted), so we drop it explicitly to signal intent.
    drop(tokio::spawn(server));
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind address.");
    let local_addr = listener.local_addr().expect("Could not get local address.");

    // Arrange
    spawn_app(listener);
    let client = reqwest::Client::new();

    // Post
    let params = [
        ("name", "DenverCoder9"),
        ("email", "denvercoder9@lycos.com"),
    ];

    // Act
    let response = client
        .post(format!("http://{}/subscribe", local_addr))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind address.");
    let local_addr = listener.local_addr().expect("Could not get local address.");

    // Arrange
    spawn_app(listener);
    let client = reqwest::Client::new();

    //
    let test_table = [
        (
            (("name", "DenverCoder9"), ("email", "")),
            "Missing the email",
        ),
        (
            (("name", ""), ("email", "denvercoder9@lycos.com")),
            "Missing the name",
        ),
        ((("name", ""), ("email", "")), "Missing the name and email"),
    ];

    for (params, error_message) in test_table {
        // Act
        let response = client
            .post(format!("http://{}/subscribe", local_addr))
            .form(&params)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
