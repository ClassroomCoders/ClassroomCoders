use async_nats::service::ServiceExt;
use common::get_nats_client;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    // Connect to the NATS server
    let client = get_nats_client()
        .await
        .expect("Unable to connect to the nats server");

    // Register the service: name + semver version
    let service = client
        .service_builder()
        .description("Authentication service")
        .start("auth", "0.1.0")
        .await?;

    // Groups prefix subjects: this endpoint listens on "auth.v1.login"
    let v1 = service.group("auth").group("v1");
    let mut login = v1.endpoint("login").await?;

    while let Some(request) = login.next().await {
        let payload = String::from_utf8_lossy(&request.message.payload);
        println!("login request: {payload}");

        // Ok(bytes) for success, or Err(service::error::Error { code, status }) for failure
        request.respond(Ok("logged in".into())).await?;
    }

    Ok(())
}
