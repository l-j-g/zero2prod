use actix_web::rt::spawn;
use zero2prod::main;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await.expect("Failed to spawn our app.");

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// spawn_app is the only function that depends on our application code.
async fn spawn_app() -> std::io::Result<()> {
    zero2prod::run().await
}
