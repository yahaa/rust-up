mod secret;

#[macro_use]
extern crate log;

use kube::Client;

use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=debug");
    env_logger::init();
    let client = Client::try_default().await?;

    tokio::spawn(secret::watch_ns(client.clone()));

    let mut stream = signal(SignalKind::terminate())?;

    stream.recv().await;
    info!("recv SIGTERM, graceful shutdown...");

    Ok(())
}
