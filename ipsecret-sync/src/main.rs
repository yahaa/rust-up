#[macro_use]
extern crate log;

mod secret;

use ipsecret_sync::config;
use kube::Client;

use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=debug");
    env_logger::init();

    let client = Client::try_default().await?;
    let cf = config::ConfigFactory::new(
        client.clone(),
        "docker-registry".to_string(),
        "default".to_string(),
    );

    info!("{:?}", cf.read_config().await);

    tokio::spawn(secret::watch_ns(client.clone()));

    signal(SignalKind::terminate())?.recv().await;

    info!("recv SIGTERM, graceful shutdown...");

    Ok(())
}
