use aws_sdk_ecs as ecs;

#[::tokio::main]
async fn main() -> Result<(), ecs::Error> {
    let config = aws_config::load_from_env().await;
    let _client = aws_sdk_ecs::Client::new(&config);

    // ..make some calls with client

    Ok(())
}
